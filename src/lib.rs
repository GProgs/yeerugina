#![warn(missing_docs)]

//! An interface between a MQTT broker and a YeeLight lamp.
//!
//! The program will process messages sent under some MQTT topic, parse them and pass them onward
//! to the lamp by sending them through a TcpStream.

/// (Sub)module containing structs and traits related to commands (effects and values).
pub mod cmd;
/// Module containing traits related to commands.
//pub mod cmd_traits;
/// Module containing structs related to the program configuration.
pub mod config;
/// Module containing the Lamp struct.
pub mod lamp;
/// Module containing the definitions of valid Yeelight commands.
//pub mod lamp_cmds;
/// Module containing functions that pertain to MQTT.
/// For instance, functions taking in input messages are defined here.
pub mod mqtt;

/*
/// Module containing objects needed for stateful lamp control.
pub mod stateful;
*/

//pub use lamp::Lamp;
//pub use structs::Command;
