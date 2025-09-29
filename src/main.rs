//use yeerugina::lamp::Lamp;
use yeerugina::structs::Command;

fn main() {
	println!("Hello, world!");
	let cmd = Command::GetProp(
		["power", "not_exist", "bright"]
			.iter()
			.map(|s| s.to_string())
			.collect(),
	);
	let cmd2 = Command::Toggle;
	let mut counter = std::num::Wrapping(254u8);
	println!("Command is {}", cmd);
	if let Command::GetProp(vals) = &cmd {
		println!("Command field is {:?}", vals);
	}
	println!("to_command is {}", cmd.to_request(counter.0));
	counter += 2;
	println!("to_request for cmd2 is {}", cmd2.to_request(counter.0));
}
