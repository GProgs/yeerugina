extern crate derive_more;

use derive_more::Display;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::io;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use strum_macros;

#[derive(Debug)]
pub struct Lamp {
	name: String,
	ip: String,
	stream: Option<TcpStream>,
}

impl Lamp {
	pub fn new(name: String, ip: String) -> Self {
		Self {
			name,
			ip,
			stream: None,
		}
	}

	pub fn connect(&mut self) -> io::Result<()> {
		self.stream = Some(TcpStream::connect(&self.ip)?);
		Ok(())
	}

        pub fn send_cmd(&self, cmd: Command) -> io::Result<()> {
            if self.stream.is_none() {
                return Err(io::Error::new(io::ErrorKind::NotConnected,"Lamp is not connected yet"));
            }

            let byte_arr: &[u8] = cmd.to_request().as_bytes();

            Ok(())
        }
}

#[derive(Debug,Deserialize)]
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

#[derive(Debug,strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Effect {
	Sudden,
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

#[derive(Debug,strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
//#[derive(Display)]
//#[display(fmt = r#"{"id":1,"method":"{}","params":"{}"}"#, )]
pub enum Command {
	GetProp(Vec<String>),
	SetCtAbx(u8, Effect, usize),
	SetRgb(u32, Effect, usize),
	SetHsv(u8, u8, Effect, usize),
	SetBright(u8, Effect, usize),
	SetPower(bool, Effect, usize, Option<usize>),
        Toggle,
}

impl Command {
	pub fn to_request(&self) -> String {
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
			Command::SetPower(pow, eff, dur, maybe_mod) => { // handle optional
				let mode = maybe_mod.unwrap_or_default(); // can't use mod
				format!(r#"{},{},"{}",{}"#, pow, eff, dur, mode)
			},
                        Command::Toggle => String::new(),
		};
                // Construct the final request, adding \r\n to the end
		format!(concat!(r#"{{"id":1,"method":"{}","params":[{}]}}"#,"\r\n"), self, param_part)
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
