use yeerugina::cmd::{Value, ValueKind};

fn main() {
	// Example code that demonstrates working with Values.
	// 1) Show the limits of color temperature, and create two Values.
	// 2)

	println!(
		"Limit for color temp is {:#?}",
		Value::limit(ValueKind::ColorTemp)
	);
	let valid_temp = Value::new(3500, ValueKind::ColorTemp).expect("3500K should be ok");
	println!("3500K value is {valid_temp}; debug print {valid_temp:?}");
	if let Err(e) = Value::new(15000, ValueKind::ColorTemp) {
		eprintln!("Couldn't create 15000K value: {e}");
	}
}
