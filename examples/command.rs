use yeerugina::cmd::{Command, CommandKind, Effect, Value, ValueKind};

fn main() {
	// 1) Create a valid and consistent monadic command.
	let param_1 = Some(
		Value::new(3700, ValueKind::ColorTemp)
			.expect("Couldn't create a valid color temperature Value"),
	);
	let effect = Effect::default();
	let monadic_cmd = Command::new(CommandKind::SetCtAbx, param_1, None, effect);
	if let Err(e) = monadic_cmd {
		eprintln!("I goofed; the error is {}", e);
	} else {
		println!("My monadic command is {:?}!", monadic_cmd);
	}
}
