//extern crate derive_more;

//use derive_more::Display;
use serde::Deserialize;
//use time::UtcDateTime;
use std::error::Error;

//use std::time::{SystemTime, UNIX_EPOCH};
use strum_macros;

/* TODO list here:
 * - Mark this commit as unlinted, so we need to lint later
 * - DONE Create some kind of ID for every request
 *   s.t. we can keep track of success/failed requests
 *   (this could be of format HHMM, like 1234 for example)
 * - Create some kind of logic (i.e. separate method etc.) to read responses
 * - Write the regex for the parse command (see the todo!() macro)
 * - Actually implement deserialization for Config w/ serde
 *   s.t. we can read a config.toml file into a Config struct
 * - (for a lot later) figure out openssl... won't build on my Windows laptop
 *   because clang lib missing...
 */

// Just FYI we're deriving Debug for all structs here
// because that's recommended.
// Struct field stream is Option<_> because we're not connected
//   until Lamp.connect() is called.
// Using a counter that wraps around.

// Configuration file where we persist info about the lamp
//   i.e. what its IP is and where our MQTT broker is
#[derive(Debug, Deserialize)]
pub struct Config {
	pub lamp_ip: String,
	pub mqtt_addr: String,
}

impl Config {
	pub fn read_file(path: String) -> Result<Self, Box<dyn Error>> {
		let cont = std::fs::read_to_string(path)?;
		Ok(toml::from_str(&cont)?)
	}
}

// need default due to EnumString trait bound
#[derive(Debug, Default, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Effect {
	Sudden,
	#[default]
	Smooth,
}

/*
impl fmt::Display for Effect {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,"{}",match self {
			Effect::Sudden => "sudden",
			Effect::Smooth => "smooth",
		})
	}
}
*/

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
//#[derive(Display)]
//#[display(fmt = r#"{"id":1,"method":"{}","params":"{}"}"#, )]
pub enum Command {
	GetProp(Vec<String>),
	SetCtAbx(u16, Effect, usize),
	SetRgb(u32, Effect, usize),
	SetHsv(u8, u8, Effect, usize),
	SetBright(u8, Effect, usize),
	SetPower(bool, Effect, usize, Option<usize>),
	Toggle,
}

impl Command {
	pub fn to_request(&self, id: u8) -> String {
		//let param_part = match self {
		//    GetProp(ps) => ps.to_string(), // unwrap Vec from GetProp
		//};

		// Create a comma-separated list of parameters.
		// For example, "on","smooth",500
		// or 60,30,"sudden"
		// If a method does NOT expect parameters, use an EMPTY STRING.
		let param_part: String = match self {
			Command::GetProp(params) => format!("\"{}\"", params.join("\",\"")), // quotes
			Command::SetCtAbx(ct_val, eff, dur) => format!(r#"{},"{}",{}"#, ct_val, eff, dur),
			Command::SetRgb(rgb, eff, dur) => format!(r#"{},"{}",{}"#, rgb, eff, dur),
			Command::SetHsv(hue, sat, eff, dur) => format!(r#"{},{},"{}",{}"#, hue, sat, eff, dur),
			Command::SetBright(bri, eff, dur) => format!(r#"{},"{}",{}"#, bri, eff, dur),
			Command::SetPower(pow, eff, dur, maybe_mod) => {
				// handle optional
				let mode = maybe_mod.unwrap_or_default(); // can't use mod
				format!(r#"{},{},"{}",{}"#, pow, eff, dur, mode)
			},
			Command::Toggle => String::new(),
		};
		//let now = UtcDateTime::now(); // Alternative - let the send_cmd() method handle the
		// ID stuff. Besides, it needs to verify that the
		// command worked (or not).
		//let id: String = format!("{}{}{}",now.hour(),now.minute(),now.second());
		// Construct the request, adding \r\n to the end
		format!(
			concat!(r#"{{"id":{},"method":"{}","params":[{}]}}"#, "\r\n"),
			id, self, param_part
		)
	}
}

/*
impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out_str: String = match self {
			command::GetProp(params) =>
		}
	}
}
*/
