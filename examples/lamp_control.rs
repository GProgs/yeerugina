use std::{thread, time};
use yeerugina::lamp::Lamp;
use yeerugina::structs::{Command, Effect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create and connect to lamp
	let mut lamp = Lamp::new(
		String::from("Livingroom"),
		String::from("192.168.1.3:55443"),
	);
        // Define timeouts here
        let timeouts = (
            Some(time::Duration::from_secs(3)),
            None,
        );
	lamp.connect(timeouts)?;

	// Create commands
	//let cmd = Command::GetProp(vec![
	//    String::from("power"),
	//    String::from("not_exist"),
	//    String::from("bright"),
	//]);
	let cmd = Command::SetRgb(0xdeadfeu32, Effect::Smooth, 2000);
	let cmd2 = Command::SetCtAbx(2800u16, Effect::Smooth, 2000);
	let cmd3 = Command::SetCtAbx(4000u16, Effect::Smooth, 2000);
	let wait = time::Duration::from_secs(6);

	// Send commands
	let cmd_id: u8 = lamp.send_cmd(cmd)?;
	thread::sleep(wait);
	let cmd2_id: u8 = lamp.send_cmd(cmd2)?;
	thread::sleep(wait);
	let cmd3_id: u8 = lamp.send_cmd(cmd3)?;
	thread::sleep(wait);

	// Print IDs to the user
	println!("Command IDs are {cmd_id}, {cmd2_id}, and {cmd3_id}");

	// TcpStream will be dropped automatically
	Ok(())
}
