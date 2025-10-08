//use yeerugina::lamp::Lamp;
use ctrlc;
use log::{debug, error, info};
use paho_mqtt as mqtt;
use yeerugina::mqtt::mqtt_props;
use yeerugina::structs::{Command, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();
	println!("Hello, world!");

	let conf = Config::read_file(String::from("config.toml"))?;
	info!("Config loaded");

	// Creating options here
	let create_opts = mqtt::CreateOptionsBuilder::new()
		.server_uri(format!("mqtt://{}", conf.mqtt.ip))
		.client_id(conf.mqtt.id)
		.finalize();
	debug!("MQTT settings created");

	// Create the client and other stuff
	let cli = mqtt::Client::new(create_opts)?;
	info!("MQTT client created");
	let rx: mqtt::Receiver<_> = cli.start_consuming();
	debug!("MQTT receiver initialized");

	// last will and testament
	let lwt = mqtt::MessageBuilder::new()
		.topic("lwt")
		.payload(conf.mqtt.lwt_payload)
		.finalize();
	debug!("LWT message created");

	// connection settings
	let conn_opts = mqtt::ConnectOptionsBuilder::new_v5()
		.clean_start(false)
		.properties(mqtt_props())
		.will_message(lwt)
		.finalize();
	debug!("Connection options created");

	// Connect to the broker
	let rsp: mqtt::ServerResponse = cli.connect(conn_opts)?;

	if let Some(conn_rsp) = rsp.connect_response() {
		todo!();
	}

	// Implement Ctrl-C
	let ctrlc_cli = cli.clone();
	if let Err(e) = ctrlc::set_handler(move || {
		info!("Received Ctrl+C signal");
		ctrlc_cli.stop_consuming();
	}) {
		error!("Could not add Ctrl+C handling: {e}");
	}

	Ok(())
}
