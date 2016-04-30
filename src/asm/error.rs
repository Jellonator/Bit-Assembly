use super::value::Value;
use std::fmt;
use std::process;

pub enum ArgumentType {
	Exact(usize),
	Range(usize, usize),
	AtLeast(usize),
	AtMost(usize)
}

pub enum ErrorType {
	Generic ( String ),
	InvalidValue ( String ),
	InvalidPointer ( Value ),
	ArgumentError { typename:String, name:String, num:usize, range:ArgumentType },
	NonExistent { typename: String, value: String },
	Empty ( String ),
}

pub struct Error {
	text: String,
	line: usize,
	file: Option<String>
}

impl ArgumentType {
	pub fn is_valid(&self, value: usize) -> bool {
		match *self {
			ArgumentType::Exact(num) =>      value == num,
			ArgumentType::Range(min, max) => value >= min && value <= max,
			ArgumentType::AtLeast(num) =>    value >= num,
			ArgumentType::AtMost(num) =>     value <= num
		}
	}
}

impl fmt::Display for ArgumentType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ArgumentType::Exact(num) =>      write!(f,          "{} argument(s)", num),
			ArgumentType::Range(min, max) => write!(f,       "{}-{} argument(s)", min, max),
			ArgumentType::AtLeast(num) =>    write!(f, "at least {} argument(s)", num),
			ArgumentType::AtMost(num) =>     write!(f,  "at most {} argument(s)", num)
		}
	}
}

impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ErrorType::Generic (ref generic) => write!(f, "Generic: {}", generic),
			ErrorType::InvalidValue (ref val) => write!(f, "InvalidValue: {}", val),
			ErrorType::ArgumentError { ref typename, ref name, ref num, ref range } =>
				write!(f, "ArgumentError: {} '{}' was given {} argument(s), expected {}",
				typename, name, num, range),
			ErrorType::NonExistent { ref typename, ref value } =>
				write!(f, "NameError: no such {} of name '{}'", typename, value),
			ErrorType::Empty ( ref name ) => write!(f, "{} is empty", name),
			ErrorType::InvalidPointer ( ref ptr ) => write!(f, "{} is not a valid pointer!", ptr)
		}
	}
}

#[allow(dead_code)]
impl Error {
	pub fn new(text:String, line:usize, filename:Option<String>) -> Error {
		Error {
			text: text,
			line: line,
			file: filename
		}
	}

	pub fn throw(&self, errortype: ErrorType) -> ! {
		println!(
			"Error on line {}{}, {}.\n>>> {}",
			self.line,
			match self.file {
				Some(ref name) => format!(" in file \"{}\"", name),
				None => "".to_string()
			},
			errortype,
			self.text
		);
		process::exit(0)
	}

	pub fn check_args(&self, typename: &str, name:&str, num:usize, range: ArgumentType) {
		if !range.is_valid(num) {
			self.throw(ErrorType::ArgumentError{
				typename: typename.to_string(),
				name: name.to_string(),
				num: num,
				range: range
			});
		}
	}
}
