use log::debug;
use yeerugina::lamp::Lamp;

#[derive(Debug)]
pub struct DummyLamp {
	// Expose the inner Lamp
	pub lamp: Lamp,
}

impl DummyLamp {
	pub fn new() -> Self {
		let lamp = Lamp::new(String::from("dummylamp"), String::from("127.0.0.1:6666"))
			.expect("Could not create lamp");
		Self { lamp }
	}

	// Dummy connect method
	pub fn connect(&mut self) -> () {
		debug!("Dummy connect called");
	}
}
