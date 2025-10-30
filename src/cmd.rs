use log::info;
use std::fmt;
use std::time::Duration;

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

pub trait Command {
	//type Params: Sized + CommaPrint;

	// This method signature is not compatible w/ vtables
	//fn limit_cond(&self) -> impl Fn(&Self::Params) -> bool;
	fn is_valid(&self) -> bool;
	//fn get_params(&self) -> Self::Params;
	fn request(&self) -> String;

	// Either this or use the impl_new! macro.
}

// Blanket impl Debug
/*
impl<T: Command> fmt::Debug for T {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

	}
}
*/

pub trait CommaPrint {
	fn comma_print(&self) -> String;
}

/*

pub struct SetCtAbx {
	ct: u16,
}

impl Command for SetCtAbx {
	type Params = u16;

	fn limit_cond(&self) -> Fn(Params) -> bool {
		|ct| (1700..=6500).contains(ct)
	}

	fn request(&self, id: u8) -> String {
		String::from("this is a test")
	}
}

*/

#[derive(Debug)] // TODO write my own impl?
struct NoData;

struct InnerCommand<T> {
	pub id: u8,
	params: T,
}

impl<T> InnerCommand<T> {
	/*
	fn new(id: u8, params: T) -> Self {
		Self { id, params }
	}
	*/

	/*
	fn request(&self) -> String {
		let param_part = format!("{:?}",self.params);
		todo!()
	}
	*/
}

impl<T: fmt::Debug> InnerCommand<T> {
	fn request(&self) -> String {
		let param_part = format!("{:?}", self.params);
		todo!()
	}
}

/*
impl<NoData> InnerCommand<NoData> {
	fn request(&self) -> String {
		String::from("dummy")
	}
}
*/

impl CommaPrint for u16 {
	fn comma_print(&self) -> String {
		format!("{},", self)
	}
}

impl CommaPrint for (u16, u8) {
	fn comma_print(&self) -> String {
		format!("{},{}", self.0, self.1)
	}
}

macro_rules! impl_new {
	($name:ident,$ty:ty) => {
		impl $name {
			fn new(id: u8, params: $ty) -> Option<Self> {
				let mby_me = Self(InnerCommand { id, params });
				if !mby_me.is_valid() {
					return None;
				}
				Some(mby_me)
			}
		}
	};
}

pub struct SetCtAbx(InnerCommand<u16>);
pub struct SetHsv(InnerCommand<(u16, u8)>);
impl_new!(SetHsv, (u16, u8));
pub struct Toggle(InnerCommand<NoData>);

impl Command for SetCtAbx {
	//type Params = u16;
	/*
	fn limit_cond() -> impl Fn(&Self::Params) -> bool {
		|ct| (1700..=6500).contains(ct)
	}
	*/

	fn is_valid(&self) -> bool {
		(1700..=6500).contains(&self.0.params)
	}

	fn request(&self) -> String {
		let foobar = format!("hello {}", self.0.params.comma_print());
		let another_foobar = self.0.request();
		todo!()
	}
}

impl Command for SetHsv {
	//type Params = (u16, u8);
	/*
	fn limit_cond() -> impl Fn(&Self::Params) -> bool {
		|(hue, sat)| (0..=359).contains(hue) && (0..=100).contains(sat)
	}
	*/

	fn is_valid(&self) -> bool {
		(0..360).contains(&self.0.params.0) && (0..=100).contains(&self.0.params.0)
	}

	fn request(&self) -> String {
		todo!()
	}
}

// Create constructors that leverage the limit_cond method

/*
impl SetCtAbx {
	fn new(id: u8, params: u16) -> Option<Self> {
		if !(Self::is_valid(&params)) {
			return None;
		}
		Some(Self::from(InnerCommand { id, params }))
	}
}

*/

//impl_new!(SetCtAbx,u16);
