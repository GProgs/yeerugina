use color::{ColorSpace, OpaqueColor, Rgba8, Srgb};
use log::debug;
use serde::Deserialize;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use strum_macros;
use strum_macros::EnumString;

/* TODO list here:
 * - Integrate OpaqueColor into our ecosystem better
 * - We might need a stateful memory that updates a state memory based on MQTT messages
 * - Create a "dummy version" of Lamp so that we can test our code (no-op connect() method, print
 * bytes from send_cmd(),...)
 *
 * - Create some kind of logic (i.e. separate method etc.) to read responses
 * - Write the regex for the parse command (see the todo!() macro)
 * - (for a lot later) figure out openssl... won't build on my Windows laptop
 *   because clang lib missing...
 */

/// Struct that stores settings of the program.
///
/// The struct is divided into two parts:
/// One for the lamp, another for the MQTT connection.
#[derive(Debug, Deserialize)]
pub struct Config {
	/// Sub-struct containing lamp-related settings.
	pub lamp: LampConfig,
	/// Sub-struct containing settings for the MQTT connection.
	pub mqtt: MqttConfig,
}

/// Struct containing the IP address and several timeout values.
///
/// The default_duration pertains to the length of the smooth color transition of the lamp.
/// The read/write timeouts are related to the TcpStream. None means the corresponding functions
/// can block indefinitely.
/// connection_tries indicates how many times the program should attempt to connect before giving
/// up. The _wait variable is the time between attempts, while connection_timeout is related to the
/// TcpStream::connect_timeout() function.
#[derive(Debug, Deserialize)]
#[serde(rename = "lamp", rename_all = "kebab-case")]
pub struct LampConfig {
	/// A name for identifying the lamp.
	pub name: String,
	/// IP address and port of the lamp.
	pub ip: SocketAddr,
	/// How long a smooth color transition takes
	#[serde(with = "humantime_serde")]
	pub default_duration: Duration,
	/// How long TcpStream waits for incoming data.
	#[serde(
		deserialize_with = "humantime_serde_opt",
		default = "default_timeout_opt"
	)]
	pub read_timeout: Option<Duration>,
	/// How long TcpStream takes to send data (at maximum).
	#[serde(
		deserialize_with = "humantime_serde_opt",
		default = "default_timeout_opt"
	)]
	pub write_timeout: Option<Duration>,
	/// How many tries to attempt to connect before giving up.
	pub connection_tries: u8,
	/// For how long to wait between connection attempts.
	#[serde(with = "humantime_serde", default = "default_wait")]
	pub connection_tries_wait: Duration,
	/// How long each connection attempt takes (at maximum).
	#[serde(with = "humantime_serde", default = "default_wait")]
	pub connection_timeout: Duration,
}

/// The default value for connection_tries_{wait,timeout}.
fn default_wait() -> Duration {
	Duration::from_secs(5)
}

/// The default value for {read,write}_timeout
fn default_timeout_opt() -> Option<Duration> {
	Some(Duration::from_secs(5))
}

/// Custom deserializer function for Option<Duration>
fn humantime_serde_opt<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let opt = Option::<String>::deserialize(deserializer)?;
	debug!("Deserialized option {opt:?}");
	match opt {
		None => Ok(None), // don't care, will be replaced by default_timeout()
		Some(s) if s.is_empty() => Ok(None), // this one will actually be None
		Some(s) => humantime::parse_duration(&s)
			.map(Some)
			.map_err(serde::de::Error::custom),
	}
}

impl LampConfig {
	/// Get a tuple containing the read and write timeouts of the lamp.
	pub fn get_read_write_timeouts(&self) -> (Option<Duration>, Option<Duration>) {
		(self.read_timeout, self.write_timeout)
	}

	/// Return a ConnectionSettings struct.
	pub fn get_connection_settings(&self) -> ConnectionSettings {
		ConnectionSettings {
			read_timeout: self.read_timeout,
			write_timeout: self.write_timeout,
			conn_timeout: self.connection_timeout,
			conn_tries: self.connection_tries,
			conn_wait: self.connection_tries_wait,
		}
	}
}

/// Struct containing settings that are used to define the MQTT connection.
#[derive(Debug, Deserialize)]
#[serde(rename = "mqtt", rename_all = "kebab-case")]
pub struct MqttConfig {
	/// IP address and port of the MQTT broker.
	pub ip: SocketAddr,
	/// Client identifier used as the name of this program:
	#[serde(default = "default_id")]
	pub client_id: String,
	/// What topic the program uses as input.
	pub topic: String,
	/// Subscription ID used for this topic.
	pub sub_id: i32,
	/// Define the QoS value for the subscription.
	#[serde(default = "default_qos")]
	pub qos: u32,
	/// Last will and testament (LWT) payload.
	pub lwt_payload: String,
}

/// Default client ID.
fn default_id() -> String {
	String::from("yeerugina")
}

/// Default QoS value
fn default_qos() -> u32 {
	1u32
}

impl Config {
	/// Deserialize a .toml file containing the settings and produce a Config struct.
	pub fn read_file(path: String) -> Result<Self, String> {
		debug!("Reading config from {path}");
		let cont = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
		debug!("File read successfully");
		toml::from_str(&cont).map_err(|e| e.to_string())
	}
}

/// A struct containing settings that is passed to Lamp::connect().

type OptDuration = Option<Duration>;
#[derive(Debug)]
pub struct ConnectionSettings {
	/// Read timeout for TcpStream
	pub read_timeout: OptDuration,
	/// Write timeout for TcpStream
	pub write_timeout: OptDuration,
	/// Connection timeout for TcpStream::connect_timeout()
	pub conn_timeout: Duration,
	/// How many times to attempt to connect to the lamp
	pub conn_tries: u8,
	/// How long to wait between each connection attempt
	pub conn_wait: Duration,
}

// need default due to EnumString trait bound
/// Enum that indicates the two possible color transitions supported in YeeLight lamps.
/// Sudden means that the color will change without any time (i.e. instantly),
/// while Smooth transitions take place over some length of time.
#[derive(
	Clone, Copy, Debug, Default, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString,
)]
#[strum(serialize_all = "lowercase")]
pub enum Effect {
	/// Instant transition.
	Sudden,
	/// Smooth transition (used by default)
	#[default]
	Smooth,
}

// I'm sorry for this clusterduck.
// OpaqueColor<CS> doesn't implement PartialEq, Eq, or Default
// which are all needed for strum_macros::EnumString
#[derive(Clone, Debug)]
struct OpaqueColorWrapper<CS> {
	color: OpaqueColor<CS>,
}

impl<CS: ColorSpace> PartialEq for OpaqueColorWrapper<CS> {
	fn eq(&self, other: &Self) -> bool {
		self.color.components == other.color.components && self.color.cs == other.color.cs
	}
}

impl<CS: ColorSpace> Eq for OpaqueColorWrapper<CS> {}

impl<CS: ColorSpace> Default for OpaqueColorWrapper<CS> {
	fn default() -> Self {
		OpaqueColorWrapper {
			color: OpaqueColor::BLACK,
		}
	}
}

/// Enum that contains all possible commands supported by YeeLight lamps.
///
/// Note that parsing logic is NOT included in the Command enum. Instead, the user is responsible
/// for parsing any Strings to Commands. See mqtt.rs.
#[derive(Clone, Debug, PartialEq, Eq, strum_macros::Display, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Command { // TODO create a newtype struct containing only InnerCommand
	/// Get properties of the lamp (i.e. current color temperature, brightness...)
	GetProp(Vec<String>),
	/// Set the color temperature of the lamp.
	SetCtAbx(usize),
	/// Set the color of the lamp using a 24 bit hexadecimal value.
	/// 0xRRGGBB
	SetRgb(usize),
	/// Set the color of the lamp by hue and saturation.
	SetHsv(usize, usize),
	/// Additional command: Set the color of the lamp by passing in an OpaqueColor.
	SetOpaqueColor(OpaqueColorWrapper<Srgb>), // this doesn't implement PartialEq or Eq
	/// Set the brightness of the lamp in percentages.
	SetBright(usize),
	/// TODO write documentation here
	//SetPower(bool, Effect, usize, Option<usize>),
	/// Toggle the state of the lamp (i.e. off -> on, on -> off)
	Toggle,
}

impl Command {
        /// Create a new Command::SetCtAbx enum.
        pub fn new_ct_abx(val: usize) -> Result<Self,String> {
            if (val < 1000) | (val > 7000) { // TODO get the actual values
                Err(String::from("Color temperature out of bounds"))
            } else {
                Ok(Self::SetCtAbx(val))
            }
        }

        pub fn new_rgb(val: usize) -> Result<Self,String> {
            if val > 0xFFFFFF { // TODO get the actual values
                Err(String::from("Invalid RGB value; must be less than 0xFFFFFF"))
            } else {
                Ok(Self::SetRgb(val))
            }
        }

        pub fn new_hsv(hue: usize, sat: usize) -> Result<Self,String> {
            if (hue > 255) | (sat > 255) { // TODO get the actual values
                Err(String::from("Hue and/or saturation out of bounds"))
            } else {
                Ok(Self::SetHsv(hue,sat))
            }
        }

        // TODO finish the rest of the new_ methods

	/// Convert a Command to a String, given an integer to use as an ID.
	pub fn to_request(&self, id: u8, eff: &Effect, dur: &Duration) -> String {
		// Shadow dur (we care only about the millisecond value)
		//let dur = dur.as_millis();
		// Create a comma-separated list of parameters.
		// For example, "on","smooth",500
		// or 60,30,"sudden"
		// If a method does NOT expect parameters, use an EMPTY STRING.
		let param_part: String = match self {
			Command::GetProp(params) => format!("\"{}\"", params.join("\",\"")), // quotes
			Command::SetCtAbx(val) | Command::SetRgb(val) | Command::SetBright(val) => format!(r#"{},"{}",{}"#, val, eff, dur.as_millis()),
                        //Command::SetCtAbx(ct_val) => format!(r#"{},"{}",{}"#, ct_val, eff, dur),
			//Command::SetRgb(rgb) => format!(r#"{},"{}",{}"#, rgb, eff, dur),
			Command::SetHsv(hue, sat) => format!(r#"{},{},"{}",{}"#, hue, sat, eff, dur.as_millis()),
			// Convert OpaqueColor to r,g,b values
			// combine them with u32::from_be_bytes
			// and recurse back thru SetRgb enum
			Command::SetOpaqueColor(col_wrap) => {
				//let rgba: Rgba8 = col.to_rgba8();
				let Rgba8 {
					r: red,
					g: green,
					b: blue,
					a: _,
				} = col_wrap.color.to_rgba8();
				//let rgb: u32 = (red << 16) + (green << 8) + blue;
				let rgb = u32::from_be_bytes([0x0, red, green, blue]) as usize;
				let rgb_cmd = Command::SetRgb(rgb);
				rgb_cmd.to_request(id, eff, dur)
			},
			//Command::SetBright(bri) => format!(r#"{},"{}",{}"#, bri, eff, dur),
			/*
						Command::SetPower(pow, eff, dur, maybe_mod) => {
				// handle optional
				let mode = maybe_mod.unwrap_or_default(); // can't use mod
				format!(r#"{},{},"{}",{}"#, pow, eff, dur, mode)
			},
						*/
			Command::Toggle => String::new(),
		};
		format!(
			concat!(r#"{{"id":{},"method":"{}","params":[{}]}}"#, "\r\n"),
			id, self, param_part
		)
	}
}
