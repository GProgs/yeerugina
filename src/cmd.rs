use log::{error, info};
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

/// A public struct used to indicate how the lamp should run this command.
///
/// New Effects are constructed by calling Effect::new_smooth(dur: Duration) for smooth transitions and
/// Effect::new_sudden() for sudden transitions. Please note that the Default is Effect(Sudden).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect(EffectInner);

/// A private enum that is wrapped by Effect.
///
/// This enum is private as there is a minimum value for the Duration contained in
/// EffectInner::Smooth (eq. 30 milliseconds). External users are not expected to instantiate these
/// enums directly. Instead, use the wrapper and its constructor functions Effect::new_smooth() and
/// Effect::new_sudden().
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum EffectInner {
	/// A transition that takes place immediately (i.e. without delay).
	#[default]
	Sudden,
	/// A transition that takes place over some time (i.e. smooth fade from red to blue).
	Smooth(Duration),
}

impl Effect {
	/// Create a new Smooth Effect from a Duration.
	///
	/// If dur is zero value, the Effect returned will be Sudden.
	/// Otherwise, dur is used directly. However, if its value is less than 30 milliseconds,
	/// the value will be clamped to 30 milliseconds.
	pub fn new_smooth(dur: Duration) -> Self {
		match dur {
			_ if dur.is_zero() => {
				info!("Zero duration converted to sudden effect");
				Effect(EffectInner::Sudden)
			},
			_ if dur.as_millis() < 30 => {
				info!("Clamped smooth effect duration to 30 ms");
				Effect(EffectInner::Smooth(Duration::from_millis(30)))
			},
			_ => Effect(EffectInner::Smooth(dur)),
		}
	}

	/// Create a Sudden Effect.
	///
	/// This function is quite trivial. It just creates a struct containing the Sudden value.
	pub fn new_sudden() -> Self {
		Effect(EffectInner::Sudden)
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

/// A public struct to indicate some kind of property of the lamp.
///
/// The first element of the tuple struct contains the value (i.e. color temperature in kelvins or RGB value -> 4500u32 or 0xDEAD67u32)
/// while the second element indicates the type of the value (see above).
///
/// Values are instantiated using Value::new(), which takes in both the numerical value and the
/// type of value (ValueKind).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Value(u32, ValueKind);

/// A public enum describing the different types of properties.
///
/// The names of the enums should be trivial.
///
/// However, an additional note on ValueKind::Hue and ValueKind::Sat is in order. While one could
/// describe hue and saturation using (u32,u32), this would lead to the inevitable necessity of
/// generics. In order to avoid having to deal with generics, it was conciously desided to separate
/// the two values into their own ValueKinds. This division will be reflected down the line as the
/// necessity of having two separate fields in Command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//#[strum_discriminants(vis(pub))]
//#[strum_discriminants(name(ValueKind))]
pub enum ValueKind {
	/// Color temperature of the lamp in kelvins. Must be within 1700-6500 K (inclusive).
	ColorTemp,
	/// RGB value of the lamp. Must be within 0x0 ..= 0xFFFFFF.
	///
	/// Using big-endian notation, the first byte needs to be zero. The second byte conveys the red value, ranging from 0x00 (eq. decimal 0) to 0xFF (eq. decimal 255). The third and fourth bytes convey the green and blue values respectively.
	///
	/// That is, the RGB value as a u32 would be of the form 0x00rrggbb, where r,g,b are the
	/// red,green,blue values respectively. As a little hint, the function u32::from_be_bytes()
	/// can be of use when constructing the RGB value from individual red, green, and blue
	/// components.
	Rgb,
	/// The hue in the HSV system. Must be within 0-359 (inclusive). Use together with
	/// ValueKind::Sat.
	Hue,
	/// The saturation in the HSV system. Must be within 0-100 (inclusive). Use together with
	/// ValueKind::Hue.
	Sat,
	/// The brightness of the lamp in percent. Must be within 0-100 (inclusive). This can be
	/// used independently of the other ValueKinds, as this controls brightness, not color.
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
	/// ```
	/// will indicate that a valid brightness
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
			Err(format!("Value out of bounds; should be in {lim:#?}"))
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
	param_1: Option<Value>,
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
	/// Set the color temperature of the lamp.
	SetCtAbx,
	/// Set the lamp to some color described with RGB.
	SetRgb,
	/// Set the lamp to some color described with the HSV system.
	///
	/// When using CommandKind::SetHsv, populate the param_1 and param_2 fields of Command with
	/// a Value with ValueKind::Hue, and a Some(Value) with ValueKind::Sat. In the absence of a
	/// value for param_2, a saturation of 100 will be used by default.
	SetHsv,
	/// Set the brightness of the lamp.
	///
	/// This will not overwrite the color currently present on the lamp - it adjusts the
	/// brightness of the lamp, making it brighter or darker.
	SetBright,
	//SetPower,
	/// Toggle the lamp on and off.
	///
	/// If the lamp is on, turns it off. If the lamp is off, turns it on.
	Toggle,
}

impl CommandKind {
	/// Get the ValueKinds associated with this CommandKind.
	///
	/// The first element of the array corresponds to param_1 in Command, and the same for the
	/// second element. None means that there is no related parameter.
	pub fn associated(&self) -> [Option<ValueKind>; 2] {
		match self {
			CommandKind::SetCtAbx => [Some(ValueKind::ColorTemp), None],
			CommandKind::SetRgb => [Some(ValueKind::Rgb), None],
			CommandKind::SetHsv => [Some(ValueKind::Hue), Some(ValueKind::Sat)],
			CommandKind::SetBright => [Some(ValueKind::Bright), None],
			CommandKind::Toggle => [None; 2],
		}
	}
}

impl Command {
	/// Print the (expected) associated ValueKinds with this Command.
	pub fn associated(&self) -> [Option<ValueKind>; 2] {
		self.kind.associated()
	}

	/// Determine whether the command is consistent (i.e. the parameters within correspond with
	/// the CommandKind::associated().
	pub fn is_consistent(&self) -> bool {
		let expected = self.associated(); // what we expect to have
		let get_kind = |p: Option<Value>| p.map(|p1| p1.get_kind());
		// what we actually have
		let actual = [get_kind(self.param_1), get_kind(self.param_2)];
		expected == actual
	}

	/// Convert a Command to a String, given an integer to use as an ID.
	pub fn to_request(&self, id: u8) -> Result<String, String> {
		// Consistency check
		// This should catch errors like having a ColorTemp in a Brightness command
		// and missing one of the two components of SetHsv
		if !self.is_consistent() {
			return Err(format!("Command is inconsistent. {:?}", self));
		}

		// For reference:
		// Effect has a custom Display impl
		// CommandKind has a Display impl from strum

		let param_part: String = match self.kind {
			// Dyadic command
			CommandKind::SetHsv => {
				// Check for the presence of parameters
				if self.param_1.is_none() {
					let emsg = "Hue missing from a SetHsv command";
					error!("{}", &emsg);
					return Err(emsg.to_string());
				}
				if self.param_2.is_none() {
					info!("Saturation missing; assuming 100");
				}
				format!(
					r#"{},{},{}"#,
					self.param_1.expect("Expected Option to be Some(_)").get(),
					self.param_2.map_or(100, |p| p.get()),
					self.effect,
				)
			},
			// Command with no arguments
			CommandKind::Toggle => String::new(),
			// Handle monadic commands here
			_ => {
				if self.param_1.is_none() {
					let emsg = "Value missing from a monadic command";
					error!("{}", &emsg);
					return Err(emsg.to_string());
				}
				format!(
					r#"{},{}"#,
					self.param_1.expect("Expected Option to be Some(_)").get(),
					self.effect
				)
			},
		};
		Ok(format!(
			concat!(r#"{{"id":{},"method":"{}","params":[{}]}}"#, "\r\n"),
			id, self.kind, param_part
		))
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
