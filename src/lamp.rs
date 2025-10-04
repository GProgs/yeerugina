use crate::structs::Command;
use log::{info, warn};
use regex::bytes::Regex;
use std::io;
use std::io::Write;
use std::net::{AddrParseError, SocketAddr, TcpStream};
use std::time::Duration;
//use yeerugina::structs::*;

// Just FYI we're deriving Debug for all structs here
// because that's recommended.
// Stream is an Option because we're not connected initially.
// Using a counter that wraps around (wrapping_add)

/// Structure (record) describing a Yeelight lamp.
///
/// The struct contains:
///     - a name chosen by the user
///     - the lamp's IP address as a SocketAddr
///     - connection to the lamp as a TcpStream
///     - a wrapping counter to keep track of commands
///
/// Example:
/// ```
/// use yeerugina::lamp::Lamp;
///
/// let mut lamp = Lamp::new(
///     String::from("Livingroom"),
///     String::from("192.168.1.3:55443"),
/// );
/// ```
#[derive(Debug)]
pub struct Lamp {
	_name: String,
	ip: SocketAddr,
	stream: Option<TcpStream>,
	cmd_count: u8,
}

impl Lamp {
	/// Creates a new Lamp struct from a user-given name and IP address.
	///
	/// The function parses the IP address String into a SocketAddr and creates the struct.
	/// The function will return an AddrParseError if the IP string cannot be parsed.
	///
	/// Example: (Assume you have created a mutable lamp.)
	/// Example:
	/// ```
	/// use yeerugina::lamp::Lamp;
	///
	/// let mut lamp = Lamp::new(
	///     String::from("Livingroom"),
	///     String::from("192.168.1.3:55443"),
	/// );
	/// ```
	pub fn new(_name: String, ip_str: String) -> Result<Self, AddrParseError> {
		let ip: SocketAddr = ip_str.parse()?;
		Ok(Self {
			_name,
			ip,
			stream: None,
			cmd_count: 0u8,
		})
	}

	/// Try to connect to the lamp, returning a Result.
	///
	/// If successful, the Result will contain the read and write timeouts of the lamp.
	/// None means the read/write operation may block indefinitely.
	/// Note that in accordance with connect_timeout() in std::net::TcpStream,
	/// the conn_timeout parameter should not be zero.
	///
	/// Example, assuming you have created a Lamp:
	/// ```
	/// use std::time;
	/// // Read/write timeouts
	/// let rw_timeouts = (Some(time::Duration::from_secs(3)), None);
	/// // How many times to attempt to connect to the lamp
	/// let conn_tries = 5u8;
	/// // How long to wait between each connection attempt
	/// let conn_wait = rw_timeouts.0.unwrap();
	/// // Connection timeout (i.e. how long each attempt takes)
	/// let conn_timeout = rw_timeouts.0.unwrap();
	/// lamp.connect(rw_timeouts, conn_tries, conn_wait, conn_timeout)?;
	/// ```
	///
	/// Initially, the function will enter a loop where it attempts to connect to the lamp.
	/// If successful, the function proceeds to set the read and write timeouts
	/// to the values provided in the read_write_timeouts tuple.
	/// If errors arise during the setting stage, they will not interrupt the function.
	/// Finally, the actual ("real") timeout values are returned as the Result.
	pub fn connect(
		&mut self, read_write_timeouts: (Option<Duration>, Option<Duration>), conn_tries: u8,
		conn_wait: Duration, conn_timeout: Duration,
	) -> io::Result<(Option<Duration>, Option<Duration>)> {
		if conn_timeout.is_zero() {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"conn_timeout cannot be zero",
			));
		};
		let mut try_counter = 0u8;
		loop {
			info!("Start connection attempt loop");
			let maybe_stream = TcpStream::connect_timeout(&self.ip, conn_timeout);
			try_counter += 1;
			match maybe_stream {
				Ok(stream) => {
					self.stream = Some(stream);
					break;
				},
				Err(e) if try_counter < conn_tries => {
					warn!("Connection failed (try {try_counter}/{conn_tries}): {e}");
					std::thread::sleep(conn_wait);
				},
				Err(e) => {
					warn!("Could not connect after {try_counter}/{conn_tries} tries; giving up");
					return Err(e);
				},
			};
		}
		//self.stream = Some(TcpStream::connect(&self.ip)?);
		// Using unwrap() since we just defined self.stream = Some(...)
		let stream: &mut TcpStream = self
			.stream
			.as_mut()
			.expect("Could not get mutable ref to stream");
		// Try to set the read and write timeouts
		if let Err(e) = stream.set_read_timeout(read_write_timeouts.0) {
			warn!("Could not set TcpStream read timeout: {e}");
		};
		if let Err(e) = stream.set_write_timeout(read_write_timeouts.1) {
			warn!("Could not set TcpStream write timeout: {e}");
		};
		// Get the values for the timeouts here
		// Note that if both operations fail
		// only the read_timeout failure will be propagated
		Ok((stream.read_timeout()?, stream.write_timeout()?))
		/*
		match (stream.read_timeout(), stream.write_timeout()) {
			(Ok(rt), Ok(wt)) => Ok((rt, wt)),
			(Err(er), Err(ew)) => {
				warn!("Reading timeouts failed; read {er}, write {ew}");
				info!("Returning only read timeout error");
				Err(er)
			},
			(Err(er), _) => {
				warn!("Reading read timeout failed; {er}");
				Err(er)
			},
			(_, Err(ew)) => {
				warn!("Reading write timeout failed; {ew}");
				Err(ew)
			},
		}
				*/
	}

	/// Try to send a command, returning the ID of said command.
	///
	/// The function takes in a Command enum, constructs the necessary byte string
	/// and then transmits the said string over the TcpStream.
	/// The internal command counter is incremented by one using wrapping_add().
	/// Any transmission errors (or trying to send_cmd on an unconnected Lamp)
	/// will be passed to the std::io::Result.
	///
	/// Example, assuming you have created a lamp:
	/// ```
	/// use yeerugina::structs::{Command, Effect};
	/// let cmd = Command::SetRgb(0xdeadfeu32, Effect::Smooth, 2000);
	/// let cmd_id: u8 = lamp.send_cmd(cmd)?;
	/// ```
	pub fn send_cmd(&mut self, cmd: Command) -> io::Result<u8> {
		// Use stream instead of self.stream later on.
		// Return io::Error if not connected yet.
		// ref mut because shared reference and moves...
		// let Some(stream) makes the borrow checker cry :'(
		let Some(ref mut stream) = self.stream else {
			return Err(io::Error::new(
				io::ErrorKind::NotConnected,
				"Lamp is not connected yet",
			));
		};
		// Get the ID for the message
		let id = self.cmd_count;
		// Construct message bytes
		let req = cmd.to_request(id);
		let byte_arr: &[u8] = req.as_bytes();
		// Output and increment counter
		stream.write_all(byte_arr)?;
		//self.cmd_count += 1;
		self.cmd_count = self.cmd_count.wrapping_add(1);

		Ok(id)
	}

	// Check that resp corresponds to the most recent command submitted to this lamp.
	pub fn is_latest_cmd(&self, resp: &[u8]) -> Result<bool, String> {
		// map_err replaces let-else construction
		let re = Regex::new(r#""id":\d+"#).map_err(|e| e.to_string())?;
		// Match the response, then Option -> Result<...,&str>
		let cap = re.captures(resp).ok_or("No ID match found")?;
		let (_, [resp_id_bytes]) = cap.extract();
		let resp_id = (str::from_utf8(resp_id_bytes).map_err(|e| e.to_string())?)
			.parse::<u8>()
			.map_err(|e| e.to_string())?;

		Ok(resp_id == self.cmd_count.wrapping_sub(1))
	}

	// Take in a response from the lamp
	// Return Ok(None) if succeeded and nothing returned
	//   (for example, when using set_rgb or toggle)
	// Ok(String) if get_prop was called and we got values back
	// Err(String) if something went wrong
	pub fn parse_response(resp: &[u8]) -> Result<Option<String>, String> {
		todo!()
		//let Ok(re) = Regex::new(todo!()) else {
		//	return Err(String::from("Could not create regex"));
		//};
	}
}
