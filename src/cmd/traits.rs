pub trait Command {
	//type Params: Sized + CommaPrint;

	// This method signature is not compatible w/ vtables
	//fn limit_cond(&self) -> impl Fn(&Self::Params) -> bool;
	fn is_valid(&self) -> bool;
	fn get_method(&self) -> String;
	fn get_inner_request(&self) -> String;

	fn request(&self) -> String {
		self.get_inner_request()
			.replace("{}", self.get_method().as_ref())
	}

	// Either this or use the impl_new! macro.
}

// Blanket impl Debug
/*
impl<T: Command> fmt::Debug for T {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

	}
}
*/

pub trait ParamPrint {
	fn comma_print(&self) -> String;
}

impl ParamPrint for u16 {
	fn comma_print(&self) -> String {
		self.to_string()
		//format!("{}", self)
	}
}

impl ParamPrint for (u16, u8) {
	fn comma_print(&self) -> String {
		format!("{},{}", self.0, self.1)
	}
}

impl ParamPrint for bool {
	fn comma_print(&self) -> String {
		match self {
			true => "\"on\"",
			false => "\"off\"",
		}
		.to_string()
	}
}
