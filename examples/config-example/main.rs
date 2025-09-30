use log::info;
use yeerugina::structs::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

        let curwd = std::env::current_dir()?;
        let curwd_str = curwd.display();
        info!("Current working directory is {curwd_str}");

	let my_config = Config::read_file(format!("{curwd_str}/examples/config-example/testconfig.toml"))?;
	info!("Example config loaded");

	println!("{my_config:?}");

        println!("When we lose connection, we say {} and that's that!",my_config.mqtt.lwt_payload);

	Ok(())
}
