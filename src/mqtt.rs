use crate::cmd::Command;

#[cfg(feature = "mqtt")]
use {
	paho_mqtt::PropertyCode::*,
	paho_mqtt::{Message, Properties, properties},
};

/// Parse a paho_mqtt::Message to a Command.
/// Returns either the command or a failure message as a String.
pub fn parse_mqtt_command(_msg: String) -> Result<Command, String> {
	todo!()
}

/// Create required MQTT properties.
#[cfg(feature = "mqtt")]
pub fn mqtt_props() -> Properties {
	properties![
		MessageExpiryInterval => 3600,
		ContentType => "application/json",
		//SubscriptionIdentifier => 1i32,
	]
}

/// Convert an i32 to a subscription ID property.
#[cfg(feature = "mqtt")]
pub fn sub_id(id: i32) -> Properties {
	properties![
			SubscriptionIdentifier => id
	]
}
