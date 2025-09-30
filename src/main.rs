//use yeerugina::lamp::Lamp;
use log::info;
use yeerugina::structs::{Command, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();
	println!("Hello, world!");

	let conf = Config::read_file(String::from("config.toml"))?;
	info!("Config loaded");

	Ok(())
}
