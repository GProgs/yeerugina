use crate::cmd::{
	objects::{InnerCommand, NoData},
	traits::{CommaPrint, Command},
};

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
