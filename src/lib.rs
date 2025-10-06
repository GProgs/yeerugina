#![warn(missing_docs)]

//! An interface between a MQTT broker and a YeeLight lamp.
//!
//! The program will process messages sent under some MQTT topic, parse them and pass them onward
//! to the lamp by sending them through a TcpStream.

/// Module containing the Lamp struct.
pub mod lamp;
/// Module containing other structs used by the program.
pub mod structs;

pub use lamp::Lamp;
pub use structs::Command;
