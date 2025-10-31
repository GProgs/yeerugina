use crate::cmd::{
	objects::{Effect, InnerCommand, NoData},
	traits::Command,
};

// Macro for implementing constructors
macro_rules! impl_new {
	($name:ident,$ty:ty) => {
		impl $name {
			fn new(id: u8, params: $ty, effect: Effect) -> Option<Self> {
				let mby_me = Self(InnerCommand { id, params, effect });
				if !mby_me.is_valid() {
					return None;
				}
				Some(mby_me)
			}
		}
	};
}

// Macro for implementing the getter methods of traits::Command
macro_rules! impl_getters {
	() => {
		fn get_method(&self) -> String {
			stringcase::snake_case(stringify!($name))
			//stringify!($name).to_snake_case()
		}

		fn get_inner_request(&self) -> String {
			self.0.inner_request()
		}
	};
}

pub struct SetCtAbx(InnerCommand<u16>);
impl_new!(SetCtAbx, u16);
pub struct SetHsv(InnerCommand<(u16, u8)>);
impl_new!(SetHsv, (u16, u8));
pub struct Toggle(InnerCommand<NoData>);
impl_new!(Toggle, NoData);

impl Command for SetCtAbx {
	impl_getters!();
	fn is_valid(&self) -> bool {
		(1700..=6500).contains(&self.0.params)
	}
}

impl Command for SetHsv {
	impl_getters!();
	fn is_valid(&self) -> bool {
		(0..360).contains(&self.0.params.0) && (0..=100).contains(&self.0.params.0)
	}
}

impl Command for Toggle {
	impl_getters!();
	fn is_valid(&self) -> bool {
		true
	}
}
