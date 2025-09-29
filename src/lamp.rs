use crate::structs::Command;
use regex::bytes::Regex;
use std::io;
use std::io::Write;
use std::net::TcpStream;
//use yeerugina::structs::*;

// Just FYI we're deriving Debug for all structs here
// because that's recommended.
// Struct field stream is Option<_> because we're not connected
//   until Lamp.connect() is called.
// Using a counter that wraps around.
#[derive(Debug)]
pub struct Lamp {
	_name: String,
	ip: String,
	stream: Option<TcpStream>,
	cmd_count: u8,
}

impl Lamp {
	pub fn new(_name: String, ip: String) -> Self {
		Self {
			_name,
			ip,
			stream: None,
			cmd_count: 0u8,
		}
	}

        // Try to connect to the lamp, returning a Result.
	pub fn connect(&mut self) -> io::Result<()> {
		self.stream = Some(TcpStream::connect(&self.ip)?);
		Ok(())
	}

        // Try to send a command, returning the ID of said command.
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
		//let Ok(re) = Regex::new(r#""id":\d+"#) else {
		//	return Err(String::from("Could not create regex"));
		//};

		// Match the response, then Option -> Result<...,&str>
		let cap = re.captures(resp).ok_or("No ID match found")?;
		let (_, [resp_id_bytes]) = cap.extract();
		//let resp_id = str::from_utf8(resp_id_bytes).map(|b| b.parse::<u8>()).map_err(|e| e.to_string());
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
		let Ok(re) = Regex::new(todo!()) else {
			return Err(String::from("Could not create regex"));
		};
	}
}
