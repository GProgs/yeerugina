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
	name: String,
	ip: String,
	stream: Option<TcpStream>,
        cmd_count: u8,
}

impl Lamp {
	pub fn new(name: String, ip: String) -> Self {
		Self {
			name,
			ip,
			stream: None,
			cmd_count: 0u8,
		}
	}

	pub fn connect(&mut self) -> io::Result<()> {
		self.stream = Some(TcpStream::connect(&self.ip)?);
		Ok(())
	}

	pub fn send_cmd(&mut self, cmd: Command) -> io::Result<()> {
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
                // Construct message bytes
		let req = cmd.to_request(self.cmd_count);
		let byte_arr: &[u8] = req.as_bytes();
                // Output and increment counter
		stream.write(byte_arr)?;
		//self.cmd_count += 1;
                self.cmd_count = self.cmd_count.wrapping_add(1);

		Ok(())
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

