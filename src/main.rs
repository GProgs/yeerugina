//use ctrlc;
use log::{debug, error, info, warn};
use paho_mqtt as mqtt;
use yeerugina::lamp::Lamp;
use yeerugina::mqtt::{mqtt_props, parse_mqtt_command, sub_id};
use yeerugina::structs::{Command, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();
	println!("Hello, world!");

	let conf = Config::read_file(String::from("config.toml"))?;
	info!("Config loaded");

	// Create lamp struct
	// Could we handle the AddrParseError more cleanly?
	let mut lamp = Lamp::new(conf.lamp.name, conf.lamp.ip)?;

	// Creating options here
	let create_opts = mqtt::CreateOptionsBuilder::new()
		.server_uri(format!("mqtt://{}", conf.mqtt.ip))
		.client_id(conf.mqtt.client_id)
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

	// Connect to the lamp and broker
	debug!("Connecting to the lamp");
	let lamp_rw_timeouts = conf.lamp.get_read_write_timeouts();
	let lamp_res = lamp.connect(
		lamp_rw_timeouts,
		conf.lamp.connection_tries,
		conf.lamp.connection_tries_wait,
		conf.lamp.connection_timeout,
	)?;
        // Emit a warning if we could not set the timeouts
	if lamp_res != lamp_rw_timeouts {
		warn!("Actual timeouts different from configured ones: {lamp_res}");
	}
        // Connect to the broker
        debug!("Connecting to the broker");
	let rsp: mqtt::ServerResponse = cli.connect(conn_opts)?;

	if let Some(conn_rsp) = rsp.connect_response() {
		info!(
			"Connected to {}, MQTT v. {}",
			conn_rsp.server_uri, conn_rsp.mqtt_version
		);

		// Check if the server remembers us or no?
		debug!("Checking session presence");
		if !conn_rsp.session_present {
			info!("Subscribing to topic");
			cli.subscribe_with_options(
				conf.mqtt.topic,
				conf.mqtt.qos,
				None,
				sub_id(conf.mqtt.sub_id),
			)?;
		}
	}

	// Implement Ctrl-C
	let ctrlc_cli = cli.clone();
	let ctrlc_res = ctrlc::set_handler(move || {
		info!("Received Ctrl+C signal");
		ctrlc_cli.stop_consuming();
	});
	// Failure to add Ctrl+C is not fatal, but an error.
	if let Err(e) = ctrlc_res {
		error!("Could not add Ctrl+C handling: {e}");
	};
	//println!("{ctrlc_res:?}");

	info!("Message reception loop ON");
	for msg in rx.iter() {
		if let Some(msg) = msg {
			let (msg_topic, msg_payload, msg_qos, msg_retain, msg_props) = (
				msg.topic(),
				msg.payload_str(), // Cow<'_,str>
				msg.qos(),
				msg.retained(),
				msg.properties(),
			);
			info!(
				"Received message. Topic {}, QoS {}, retain {}, props {:?}, content: {}",
				msg_topic, msg_qos, msg_retain, msg_props, msg_payload
			);
			// Subscription ID: i32
			let Some(msg_sub_id) = msg_props.get_int(mqtt::PropertyCode::SubscriptionIdentifier)
			else {
				info!("Message has no associated subscription ID; skipping");
				continue;
			};
			if msg_sub_id != conf.mqtt.sub_id {
				info!("Message has wrong subscription ID; skipping");
				continue;
			}
			// Parse the command
			let cmd = parse_mqtt_command(String::from(msg_payload))?;
			if let Err(e) = cmd {
				error!("Could not parse MQTT command: {e}");
				continue;
			} else {
				cmd = cmd.unwrap();
			}
			// Pass the command to our lamp

			todo!();
		} else if !cli.is_connected() {
			error!("Connection to MQTT broker lost");
			todo!(); // reconnect here
		} else {
			error!("Received None message");
			continue;
		}
	}

	Ok(())
}
