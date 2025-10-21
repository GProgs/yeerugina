use crate::lamp::Lamp;
use crate::cmd::Command;
use std::io;

#[derive(Debug)]
enum LampColorState {
	RGB(u32),
	ColorTemp(usize),
	Hsv(u8, u8),
}

enum LampStateChange {
	SetRgb(u32),
	AddTemp(isize),
	SetTemp(usize),

	AddBright(isize),
	SetBright(usize),
	Toggle,
}

// THIS WON'T WORK. WE NEED THE CURRENT CHANGE FOR ADDITIVE CHANGES.
/*
impl From<LampStateChange> for Command {
	fn from(change: LampStateChange) -> Self {
		type Chan = LampStateChange;
		match change {
			Chan::SetRgb(rgb) => Command::SetRgb(rgb),
			_ => todo!(),
		}
	}
}
*/

impl LampColorState {
	/// Consume the current LampColorState and output a new LampColorState.
	pub fn do_change(self, change: LampStateChange) -> io::Result<Self> {
		type Chan = LampStateChange;
		match (self, change) {
			(Self::RGB(_), Chan::SetRgb(rgb)) => Ok(Self::RGB(rgb)),
			(Self::ColorTemp(temp), Chan::AddTemp(dtemp)) => Ok(Self::ColorTemp(temp + dtemp)),
			(Self::ColorTemp(_), Chan::SetTemp(temp)) => Ok(Self::ColorTemp(temp)),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"State change inconsistent with current state",
			)),
		}
	}

	// TODO maybe add some method that takes &self and a LampStateChange
	// and outputs a tuple consisting of a new LampColorState
	// together with the Command needed to get to this state
}

#[derive(Debug)]
struct StatefulLamp {
	lamp: Lamp,
	color: LampColorState,
	bright: u8,
}

impl StatefulLamp {
	pub fn change_state(&mut self, change: LampStateChange) -> io::Result<()> {}
}
