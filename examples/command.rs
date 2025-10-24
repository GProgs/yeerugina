use std::time::Duration;
use yeerugina::cmd::{Command, CommandKind, Effect, Value, ValueKind};

fn main() {
	// 1) Create a valid and consistent monadic command.
        // And print it as well.
	let param_1 = Some(
		Value::new(3700, ValueKind::ColorTemp)
			.expect("Couldn't create a valid color temperature Value"),
	);
	let effect = Effect::default(); // we will reuse this because it implements Copy
        let smooth_effect = Effect::new_smooth(Duration::from_millis(350));
	let monadic_cmd = Command::new(CommandKind::SetCtAbx, param_1, None, effect);
	if let Err(e) = monadic_cmd {
		eprintln!("I goofed; the error is {}", e);
	} else {
		println!("My monadic command is {:?}!", monadic_cmd);
                println!("{}",monadic_cmd.expect("Should be Ok").to_request(32).expect("We should have a request"));
	}
	// 2) Demonstrate a command without any arguments
	let no_args_cmd = Command::new(CommandKind::Toggle, None, None, effect);
	display_command(&no_args_cmd, "My command with no args is: ");
	println!("Just for funsies: {:#?}", &no_args_cmd);
	// 3) Create a bad Hsv command and then a good one
	let our_hue = Value::new(150, ValueKind::Hue).expect("Couldn't create Hue");
	let our_sat = Value::new(42, ValueKind::Sat).expect("Couldn't create Sat");
	let our_bad_param = Value::new(80, ValueKind::Bright).expect("Couldn't create Brightness");

	let bad_hue_cmd = Command::new(CommandKind::SetHsv, Some(our_hue), Some(our_bad_param), effect);
	display_command(&bad_hue_cmd, "This should not have worked: ");
	let good_hue_cmd = Command::new(CommandKind::SetHsv, Some(our_hue), Some(our_sat), smooth_effect);
	display_command(&good_hue_cmd, "This should defo work! ");
        println!("{}",good_hue_cmd.expect("Should be Ok").to_request(16).expect("We should have a request"));
}

// Here I can get rid of a decent amount of boilerplate.
// Take in a reference to our result s.t. we can still use it afterwards!
fn display_command(maybe_cmd: &Result<Command, String>, fmt_str: &str) -> () {
	if let Err(e) = maybe_cmd {
		eprintln!("I goofed; the error is {}", e);
	} else {
		print!("{}", fmt_str);
		println!("{:?}", *maybe_cmd);
	}


}
