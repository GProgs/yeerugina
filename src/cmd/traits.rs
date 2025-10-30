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
