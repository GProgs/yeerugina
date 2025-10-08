use crate::structs::{Command, Effect};
use paho_mqtt::PropertyCode::*;
use paho_mqtt::{Message, Properties, properties};

/// Parse a paho_mqtt::Message to a Command.
/// Returns either the command or a failure message as a String.
pub fn parse_mqtt_command(msg: Message) -> Result<Command, String> {
	todo!()
}

/// Create required MQTT properties.
pub fn mqtt_props() -> Properties {
	properties![
		MessageExpiryInterval => 3600,
		ContentType => "application/json",
		//SubscriptionIdentifier => 1i32,
	]
}
