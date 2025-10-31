use std::time::Duration;

use yeerugina::cmd::lamp::SetCtAbx;
use yeerugina::cmd::objects::Effect;
use yeerugina::cmd::traits::Command;

fn main() -> () {
	let mycmd = SetCtAbx::new(32u8, 3700, Effect::new_sudden()).expect("This should be good");
	let mycmd2 = SetCtAbx::new(33u8, 5400, Effect::new_smooth(Duration::from_millis(2800)))
		.expect("This should be good");

	let myreq = mycmd.request();
	let myreq2 = mycmd2.request();
	println!("{}", &myreq);
	println!("{}", &myreq2);
}
