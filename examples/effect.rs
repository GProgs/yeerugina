use yeerugina::cmd::Effect;
use std::time::Duration;

fn main() {
	let eff = Effect::new(None);
	println!("Effect 1 is {:?}", eff);
	let eff2 = Effect::new(Some(Duration::from_millis(555)));
	println!("Effect 2 should be 555 ms, got {eff2:?}");
	let eff_def = Effect::default();
	println!("The default effect is {eff_def:?}");
	let bad_eff = Effect::new(Some(Duration::default()));
	println!("This bad effect should be 0ms, got {bad_eff:?}");
}
