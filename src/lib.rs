#![warn(missing_docs)]

//! An interface between a MQTT broker and a YeeLight lamp.
//!
//! The program will process messages sent under some MQTT topic, parse them and pass them onward
//! to the lamp by sending them through a TcpStream.

/// Module containing structs related to commands, including effects and values.
pub mod cmd;
/// Module containing structs related to the program configuration.
pub mod config;
/// Module containing the Lamp struct.
pub mod lamp;
/// Module containing functions that pertain to MQTT.
/// For instance, functions taking in input messages are defined here.
pub mod mqtt;

/*
/// Module containing objects needed for stateful lamp control.
pub mod stateful;
*/

//pub use lamp::Lamp;
//pub use structs::Command;
