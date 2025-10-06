use crate::structs::{Command, Effect};
use paho_mqtt::Message;

/// Parse a paho_mqtt::Message to a Command.
/// Returns either the command or a failure message as a String.
fn parse_mqtt_command(msg: Message) -> Result<Command, String> {
	todo!()
}
