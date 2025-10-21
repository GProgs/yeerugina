use log::info;
use std::fmt;
use std::time::Duration;
use strum_macros;
//use strum_macros::{EnumDiscriminants, EnumString, FromRepr};

// This is how we enforce privacy and correct values:
// Expose public newtype containing a private inner enum
// Define a private inner enum
// In the impl for the newtype, create a public constructor
//
// Please derive Clone, Copy, Debug, PartialEq, Eq
// and maybe Default as well if you can.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect(EffectInner);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum EffectInner {
	#[default]
	Sudden,
	Smooth(Duration),
}

impl Effect {
	/// Create a new Effect from an optional Duration.
	///
	/// If mb_dur is None or zero value, the Effect returned will be Sudden.
	/// For mb_dur with Some(duration), the Effect returned will be Smooth with a duration of
	/// max(30 ms, duration), so the minimum value of the duration is 30 milliseconds.
	pub fn new(mb_dur: Option<Duration>) -> Self {
		match mb_dur {
			None => Effect(EffectInner::Sudden),
			Some(dur) if dur.is_zero() => Effect(EffectInner::Sudden),
			Some(dur) if dur.as_millis() < 30 => {
				info!("Clamped smooth effect duration to 30 ms");
				Effect(EffectInner::Smooth(Duration::from_millis(30)))
			},
			Some(dur) => Effect(EffectInner::Smooth(dur)),
		}
	}
}

// Implementation for Effect will print out either
// "sudden" or "smooth",1234 for example.
// We didn't use strum_macros::Display since we want to print the duration as well.
impl fmt::Display for Effect {
	/// Print the effect (and duration, if applicable).
	///
	/// Example:
	/// ```
	/// let slow_transition = Effect::new(Some(Duration::from_millis(2345)));
	/// println!("Here's my slow transition! {}",slow_transition);
	/// let instant = Effect::new(None);
	/// println!("Look, this is fast! {}",instant);
	/// ```
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0 {
			EffectInner::Sudden => write!(f, "\"sudden\""),
			EffectInner::Smooth(dur) => write!(f, "\"smooth\", {}", dur.as_millis()),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Value(u32, ValueKind);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//#[strum_discriminants(vis(pub))]
//#[strum_discriminants(name(ValueKind))]
pub enum ValueKind {
	ColorTemp,
	Rgb,
	Hue,
	Sat,
	Bright,
}

impl Value {
	/// Get the allowed range of values for each type of value.
	///
	/// This function is meant primarily for input validation by
	/// Value::new (which is a public function).
	///
	/// Example:
	/// ```
	/// Value::limit(ValueKind::Bright)
	/// ``` will indicate that a valid brightness
	/// value must be between 0 and 100 (incl. ends).
	pub fn limit(kind: ValueKind) -> std::ops::RangeInclusive<u32> {
		match kind {
			ValueKind::ColorTemp => 1700..=6500,
			ValueKind::Rgb => 0..=0xFFFFFF,
			ValueKind::Hue => 0..=359,
			ValueKind::Sat => 0..=100,
			ValueKind::Bright => 0..=100,
		}
	}

	/// Create a new Value struct.
	///
	/// If successful, the function returns a Value, otherwise it will return an error string.
	/// Example:
	/// ```
	/// let my_color = 0xDEAD67u32;
	/// let my_value = Value::new(my_color,ValueKind::Rgb)?;
	/// ```
	pub fn new(val: u32, kind: ValueKind) -> Result<Self, String> {
		let lim = Value::limit(kind);
		if lim.contains(&val) {
			Ok(Self(val, kind))
			/*
			Ok(match kind {
				ValueKind::ColorTemp => Self(val,ValueKind::ColorTemp),
				ValueKind::Rgb => Self(val,ValueKind::Rgb),
				ValueKind::Hue => Self(val,ValueKind::Hue),
				ValueKind::Sat => Self(val,ValueKind::Sat),
				ValueKind::Bright => Self(val,ValueKind::Bright),
			})
			*/
		} else {
			Err(String::from("Value out of bounds; should be in {lim}"))
		}
	}

	/// Get the u32 value contained within the Value struct.
	pub fn get(&self) -> u32 {
		self.0
	}

	/// Get the ValueKind contained within the Value struct.
	pub fn get_kind(&self) -> ValueKind {
		self.1
	}
}

/// Struct that represents a Yeelight command.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Command {
	/// Type of the command.
	kind: CommandKind,
	/// The main parameter of the command. Ignored when using CommandKind::Toggle.
	param_1: Value,
	/// Optional second parameter. Ignored for other commands except CommandKind::SetHsv.
	param_2: Option<Value>,
	/// Transition effect and its speed, if smooth.
	effect: Effect,
}

/// Enum that indicates available commands in the Yeelight API.
/// The idea is that Display gives the name of the command (i.e. set_ct_abx)
/// while the enum itself is stored inside of the Command enum above.
#[derive(Clone, Copy, Debug, PartialEq, Eq, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum CommandKind {
	SetCtAbx,
	SetRgb,
	SetHsv,
	SetBright,
	//SetPower,
	Toggle,
}

impl Command {
	/// Convert a Command to a String, given an integer to use as an ID.
	pub fn to_request(&self, id: u8) -> String {
		// For reference:
		// Effect has a custom Display impl
		// CommandKind has a Display impl from strum

		let param_part: String = match self.kind {
			// Dyadic command
			CommandKind::SetHsv => {
				if self.param_2.is_none() {
					info!("Saturation missing; assuming 100");
				}
				format!(
					r#"{},{},{}"#,
					self.param_1.get(),
					self.param_2.map_or(100, |p| p.get()),
					self.effect,
				)
			},
			CommandKind::Toggle => String::new(),
			// Handle monadic commands here
			_ => format!(r#"{},{}"#, self.param_1.get(), self.effect),
		};
		format!(
			concat!(r#"{{"id":{},"method":"{}","params":[{}]}}"#, "\r\n"),
			id, self.kind, param_part
		)
	}
}

/*
pub struct Command(CommandInner);

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(name(CommandDiscrim))]
enum CommandInner {
	Cmd1,
	Cmd2(usize),
	Cmd3((usize,usize)),
}

pub trait Value {}
impl Value for usize {}
impl Value for (usize,usize) {}

impl Command {
	pub fn new<T: Value>(disc: CommandDiscrim, val: T) -> Self {
		match disc {
			CommandDiscrim::Cmd2 => Command(CommandInner::Cmd2(val)),
			_ => Command(CommandInner::Cmd1),
		}
	}
}
*/
