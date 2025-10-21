use yeerugina::cmd::Effect;
use std::time::Duration;

fn main() {
	let eff = Effect::new_sudden();
	println!("Effect 1 is {:?}", eff);
	let eff2 = Effect::new_smooth(Duration::from_millis(555));
	println!("Effect 2 should be 555 ms, got {eff2:?}");
	let eff_def = Effect::default();
	println!("The default effect is {eff_def:?}");
	let bad_eff = Effect::new_smooth(Duration::default());
	println!("This bad effect should be 0ms, got {bad_eff:?}");
        let worse_eff = Effect::new_smooth(Duration::from_millis(10));
        println!("This should be only 10ms, got {worse_eff:?}");
}
