use yeerugina::structs::Command;

fn main() {
	println!("Hello, world!");
	let cmd = Command::GetProp(
		vec!["power", "not_exist", "bright"]
			.iter()
			.map(|s| s.to_string())
			.collect(),
	);
        let cmd2 = Command::Toggle;
	println!("Command is {}", cmd);
	if let Command::GetProp(vals) = &cmd {
		println!("Command field is {:?}", vals);
	}
	println!("to_command is {}", cmd.to_request());
        println!("to_request for cmd2 is {}", cmd2.to_request());
}
