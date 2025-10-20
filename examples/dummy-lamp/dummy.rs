use crate::lamp::Lamp;
use crate::structs::Effect;
use log::debug;
use std::time::Duration;

pub struct DummyLamp {
	// Expose the inner Lamp
	pub lamp: Lamp,
}

impl DummyLamp {
	pub fn new() -> Self {
		Self {
			lamp: Lamp::new(
				String::from("dummylamp"),
				String::from("127.0.0.1:6666"),
				Effect::default(),
				Duration::from_millis(1500),
			),
		}
	}

	// Dummy connect method
	pub fn connect(&mut self) -> () {
		debug!("Dummy connect called");
	}
}
