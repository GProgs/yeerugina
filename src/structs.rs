use log::info;
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;
use strum_macros;

/* TODO list here:
 * - Create some kind of logic (i.e. separate method etc.) to read responses
 * - Write the regex for the parse command (see the todo!() macro)
 * - (for a lot later) figure out openssl... won't build on my Windows laptop
 *   because clang lib missing...
 */

// Just FYI we're deriving Debug for all structs here
// because that's recommended.

// Configuration file where we persist info about the lamp
//   i.e. what its IP is and where our MQTT broker is
#[derive(Debug, Deserialize)]
pub struct Config {
	pub lamp: LampConfig,
	pub mqtt: MqttConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "lamp", rename_all = "kebab-case")]
pub struct LampConfig {
	pub ip: SocketAddr,
	#[serde(with = "humantime_serde")]
	pub default_duration: Duration,
	#[serde(
		deserialize_with = "humantime_serde_opt",
		default = "default_timeout_opt"
	)]
	pub read_timeout: Option<Duration>,
	#[serde(
		deserialize_with = "humantime_serde_opt",
		default = "default_timeout_opt"
	)]
	pub write_timeout: Option<Duration>,
	pub connection_tries: u8,
	#[serde(with = "humantime_serde", default = "default_wait")]
	pub connection_tries_wait: Duration,
	#[serde(with = "humantime_serde", default = "default_wait")]
	pub connection_timeout: Duration,
}
// {read,write}_timeout: None means calls should block indefinitely.

// Use 5 secs if the user doesn't give another value.
// This is for Lamp.connect(...,conn_wait,...,conn_timeout)
fn default_wait() -> Duration {
	Duration::from_secs(5)
}

// This is for the TcpStream inside of Lamp
fn default_timeout_opt() -> Option<Duration> {
	Some(Duration::from_secs(5))
}

// Custom deserializer function for Option<Duration>
fn humantime_serde_opt<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let opt = Option::<String>::deserialize(deserializer)?;
	info!("Deserialized option {opt:?}");
	match opt {
		None => Ok(None), // don't care, will be replaced by default_timeout()
		Some(s) if s.is_empty() => Ok(None), // this one will actually be None
		Some(s) => humantime::parse_duration(&s)
			.map(Some)
			.map_err(serde::de::Error::custom),
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename = "mqtt", rename_all = "kebab-case")]
pub struct MqttConfig {
	pub ip: SocketAddr,
	pub topic: String,
	pub lwt_payload: String,
}

impl Config {
	pub fn read_file(path: String) -> Result<Self, String> {
		let cont = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
		toml::from_str(&cont).map_err(|e| e.to_string())
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
