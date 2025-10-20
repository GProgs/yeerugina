pub mod dummy;

use crate::dummy::DummyLamp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut my_lamp = DummyLamp::new();
	println!("{:?}", &my_lamp);

	Ok(())
}
