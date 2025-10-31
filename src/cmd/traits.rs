/// A trait that indicates a Yeelight command.
///
/// The exposed methods enforce valid parameter values and include two getters.
/// The trait is also responsible for constructing the request as a String.
pub trait Command {
	/// Returns a bool indicating whether the implementor is valid or not.
	///
	/// An example implementation for a struct containing an RGB value:
	/// ```
	/// pub struct PureGreen(InnerCommand<(u8,u8,u8)>);
	///
	/// impl Command for PureGreen {
	///     impl_getters!();
	///     fn is_valid(&self) -> bool {
	///         let (r, g, b) = &self.0.params;
	///         r == 0 && b == 0
	///     }
	/// }
	/// ```
	fn is_valid(&self) -> bool;
	/// Returns the method name as defined in the Yeelight documentation.
	fn get_method(&self) -> String;
	/// Returns a partially constructed request String.
	///
	/// Since the InnerCommand is concerned only with the storage of data, the method name is missing here.
	fn get_inner_request(&self) -> String;
	/// Returns a fully formatted request String.
	fn request(&self) -> String {
		self.get_inner_request()
			.replace("{}", self.get_method().as_ref())
	}
}

/// A trait that enables the printing of parameters.
pub trait ArgsFormatter {
	/// Returns the parameters separated by commas.
	///
	/// Example: Using the following impl
	/// ```
	/// impl ArgsFormatter for (u16, u8) {
	///     fn comma_print(&self) -> String {
	///         format!("{},{}", self.0, self.1)
	///     }
	/// }
	/// ```
	/// and calling
	/// ```
	/// let my_args: (u16,u8) = (12345,67);
	/// println!("{}",my_args.comma_print())
	/// ```
	/// should return "12345,67".
	fn comma_print(&self) -> String;
}

impl ArgsFormatter for u16 {
	fn comma_print(&self) -> String {
		self.to_string()
	}
}

impl ArgsFormatter for (u16, u8) {
	fn comma_print(&self) -> String {
		format!("{},{}", self.0, self.1)
	}
}

impl ArgsFormatter for bool {
	fn comma_print(&self) -> String {
		match self {
			true => "\"on\"",
			false => "\"off\"",
		}
		.to_string()
	}
}
