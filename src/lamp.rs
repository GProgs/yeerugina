use crate::cmd::traits::Command;
use crate::config::ConnectionSettings;
use log::{debug, error, info, trace, warn};
use regex::bytes::Regex;
use std::io;
use std::io::Write;
use std::net::{AddrParseError, SocketAddr, TcpStream};
use std::time::Duration;

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
	name: String,
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
	/// use yeerugina::lamp::{Lamp,Effect};
	/// use std::time::Duration;
	///
	/// let mut lamp = Lamp::new(
	///     String::from("Livingroom"),
	///     String::from("192.168.1.3:55443"),
	/// );
	/// ```
	pub fn new(name: String, ip_str: String) -> Result<Self, AddrParseError> {
		trace!("{} | Creating a new lamp", name);
		let ip: SocketAddr = ip_str.parse()?;
		Ok(Self {
			name,
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
	/// // Construct ConnectionSettings
	/// let conn_settings = ConnectionSettings {
	///     read_timeout: rw_timeouts.0,
	///     write_timeout: rw_timeouts.1,
	///     conn_timeout,
	///     conn_tries,
	///     conn_wait,
	/// }
	/// lamp.connect(conn_settings)?;
	/// ```
	///
	/// Initially, the function will enter a loop where it attempts to connect to the lamp.
	/// If successful, the function proceeds to set the read and write timeouts
	/// to the values provided in the read_write_timeouts tuple.
	/// If errors arise during the setting stage, they will not interrupt the function.
	/// Finally, the actual ("real") timeout values are returned as the Result.
	pub fn connect(
		&mut self, conn_settings: ConnectionSettings,
	) -> io::Result<(Option<Duration>, Option<Duration>)> {
		let ConnectionSettings {
			read_timeout,
			write_timeout,
			conn_timeout,
			conn_tries,
			conn_wait,
		} = conn_settings;
		if conn_timeout.is_zero() {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"conn_timeout cannot be zero",
			));
		};
		info!("{} | Connecting lamp", self.name);
		let mut try_counter = 0u8;
		loop {
			debug!("{} | Start connection attempt loop", self.name);
			let maybe_stream = TcpStream::connect_timeout(&self.ip, conn_timeout);
			try_counter += 1;
			match maybe_stream {
				Ok(stream) => {
					debug!("{} | TcpStream returned from connect_timeout()", self.name);
					self.stream = Some(stream);
					break;
				},
				Err(e) if try_counter < conn_tries => {
					info!("Connection failed (try {try_counter}/{conn_tries}): {e}");
					std::thread::sleep(conn_wait);
				},
				Err(e) => {
					warn!("Could not connect after {try_counter}/{conn_tries} tries; giving up");
					return Err(e);
				},
			};
		}
		// Using unwrap() since we just defined self.stream = Some(...)
		let stream: &mut TcpStream = self
			.stream
			.as_mut()
			.expect("Could not get mutable ref to stream");
		// Try to set the read and write timeouts
		trace!("{} | Setting timeout values", self.name);
		if let Err(e) = stream.set_read_timeout(read_timeout) {
			warn!("Could not set TcpStream read timeout: {e}");
		};
		if let Err(e) = stream.set_write_timeout(write_timeout) {
			warn!("Could not set TcpStream write timeout: {e}");
		};
		// Get the values for the timeouts here
		// Note that if both operations fail
		// only the read_timeout failure will be propagated
		Ok((stream.read_timeout()?, stream.write_timeout()?))
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
	pub fn send_cmd(&mut self, cmd: impl Command + std::fmt::Debug) -> io::Result<u8> {
		// About the signature: "impl" for opaque type, "&dyn" for dyn dispatch
		// or generic...
		// Also remove std::fmt::Debug trait bound if we get rid of the debug! call
		debug!("{} | Attempting to send command {cmd:?}", self.name);
		// Use stream instead of self.stream later on.
		// Return io::Error if not connected yet.
		// ref mut because shared reference and moves...
		// let Some(stream) makes the borrow checker cry :'(
		let Some(ref mut stream) = self.stream else {
			warn!("{} | Lamp not connected, cannot send command", self.name);
			return Err(io::Error::new(
				io::ErrorKind::NotConnected,
				"Lamp is not connected yet",
			));
		};
		// Get the ID for the message
		let id = self.cmd_count;
		debug!("{} Command ID {id}", self.name);
		// Construct message bytes
		let req: String = cmd.request(); // now we don't have Result...
		let byte_arr: &[u8] = req.as_bytes();
		// Output and increment counter
		trace!("{} | Writing bytes to TcpStream", self.name);
		stream.write_all(byte_arr)?;
		//self.cmd_count += 1;
		self.cmd_count = self.cmd_count.wrapping_add(1);
		debug!("{} | New Command ID {}", self.name, self.cmd_count);

		Ok(id)
	}

	// Check that resp corresponds to the most recent command submitted to this lamp.

	/// Checks that a response originates from the most recently sent command.
	/// Returns a boolean if successful, an error otherwise.
	pub fn is_latest_cmd(&self, resp: &[u8]) -> Result<bool, String> {
		trace!("{} | Checking response ID", self.name);
		// map_err replaces let-else construction
		let re = Regex::new(r#""id":\d+"#).map_err(|e| e.to_string())?;
		// Match the response, then Option -> Result<...,&str>
		let cap = re.captures(resp).ok_or("No ID match found")?;
		trace!("{} | Captured {cap:?}", self.name);
		let (_, [resp_id_bytes]) = cap.extract();
		let resp_id = (str::from_utf8(resp_id_bytes).map_err(|e| e.to_string())?)
			.parse::<u8>()
			.map_err(|e| e.to_string())?;
		trace!("{} | Obtained response ID {resp_id}", self.name);

		Ok(resp_id == self.cmd_count.wrapping_sub(1))
	}

	/// Take in a response from the lamp and parse it.
	///
	/// Return Ok(None) if succeeded and nothing returned
	/// (for example, when using set_rgb or toggle)
	/// Ok(String) if get_prop was called and we got values back
	/// Err(String) if something went wrong
	pub fn parse_response(_resp: &[u8]) -> Result<Option<String>, String> {
		todo!()
	}
}
