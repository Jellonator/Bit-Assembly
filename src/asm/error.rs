use super::value::Value;
use std::fmt;

pub enum ErrorType {
	Generic ( String ),
	InvalidValue ( Value ),
	ArgumentError { instruction:String, num:usize, min:usize, max:Option<usize> },
}

pub struct Error {
	text: String,
	line: usize,
	file: Option<String>
}

impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ErrorType::Generic (ref generic) => write!(f, "Generic: {}", generic),
			ErrorType::InvalidValue (ref val) => write!(f, "InvalidValue: {}", val),
			ErrorType::ArgumentError { ref instruction, num, min, max } => {
				match max {
					Some (maxval) => write!(f,
						"ArgumentError: instruction '{}' got {} arguments, expected {}-{}.",
						instruction, num, min, maxval),
					None => write!(f,
						"ArgumentError: instruction '{}' got {} arguments, expected {}.",
						instruction, num, min)
				}
			}
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

	pub fn throw(&self, errortype: ErrorType) {
		panic!(
			"Error on line {}{}, {}!\n>>> {}",
			self.line,
			match self.file {
				Some(ref name) => format!(" in file \"{}\"", name),
				None => "".to_string()
			},
			errortype,
			self.text
		);
	}

	pub fn check_args(&self, instruction_name:&str, num:usize, min:usize, max:Option<usize>) {
		let is_error = match max {
			Some(maxval) => num >= min && num <= maxval,
			None => num == min
		};
		if is_error {
			self.throw(ErrorType::ArgumentError{
				instruction: instruction_name.to_string(),
				num: num,
				min: min,
				max: max
			});
		}
	}
}
