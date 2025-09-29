use yeerugina::structs::Command;

fn main() {
	// Create two commands
	let cmd = Command::GetProp(
		["power", "not_exist", "bright"]
			.iter()
			.map(|s| s.to_string())
			.collect(),
	);
	let cmd2 = Command::Toggle;

	// Imagine this as the ID counter inside the lamp.
	// Well, now we use wrapped_add but it's close enough
	let mut counter = std::num::Wrapping(254u8);

	// Demonstrate printing
	println!("Command is {}", cmd);
	if let Command::GetProp(vals) = &cmd {
		println!("We are requesting {:?}", vals);
	}
	println!("Request is {}", cmd.to_request(counter.0));

	// Increment by 2 to demonstrate wrapping
	counter += 2;
	println!("Request for cmd2 is {}", cmd2.to_request(counter.0));
}
