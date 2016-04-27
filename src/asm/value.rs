extern crate regex;
extern crate gmp;
use std::fmt;
use self::regex::Regex;
use std::str::FromStr;
use super::environment::Environment;
use super::util::*;

pub enum Value {
	Boolvec (Vec<bool>),
	Bignum (gmp::mpz::Mpz),
	Pointer {pos:Box<Value>, len:Box<Value>, rev:bool}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Value::Boolvec(ref vec) => write!(f, "v{}", boolvec_to_bignum(vec)),
			Value::Bignum(ref vec) => write!(f, "n{}", vec),
			Value::Pointer{ref pos, ref len, ref rev} => {
				let negative_str = match *rev{ true => "-", false => "" };
				write!(f, "[{}{}:{}]", negative_str, pos, len)
			}
		}
	}
}

#[allow(dead_code)]
impl Value {
	pub fn is_ptr(&self) -> bool {
		match *self {
			Value::Pointer{..} => true,
			_ => false
		}
	}

	pub fn get_ptr_position(&self, env: &Environment) -> usize {
		match *self {
			Value::Pointer{ref pos, ref rev, ..} => match *rev{
				true => env.stack_len() - pos.get_usize(env),
				false => pos.get_usize(env)
			},
			_ => panic!("This value is not a pointer!")
		}
	}

	pub fn get_usize(&self, env:&Environment) -> usize {
		bignum_to_usize(&self.get_bignum(env))
	}

	pub fn get_bignum(&self, env: &Environment) -> gmp::mpz::Mpz {
		match *self {
			Value::Bignum(ref num) => num.clone(),
			Value::Boolvec(ref vec) => boolvec_to_bignum(vec),
			Value::Pointer{..} => {
				let size = self.get_ptr_size(env);
				let pos = self.get_ptr_position(env);
				let bits = env.slice(pos, pos + size);
				boolvec_to_bignum(bits)
			}
		}
	}

	pub fn get_boolvec(&self, env: &Environment) -> Vec<bool> {
		match *self {
			Value::Bignum(ref num) => bignum_to_boolvec(num),
			Value::Boolvec(ref vec) => vec.clone(),
			Value::Pointer{..} => {
				let size = self.get_ptr_size(env);
				let pos = self.get_ptr_position(env);
				let bits = env.slice(pos, pos + size);
				bits.to_vec()
			}
		}
	}

	pub fn get_ptr_size(&self, env: &Environment) -> usize {
		match *self {
			Value::Pointer{ref len, ..} => len.get_usize(env),
			_ => panic!("This value is not a pointer!")
		}
	}

	pub fn can_coerce(&self, new_size:usize, env: &Environment) -> bool {
		match *self {
			Value::Pointer{ref len, ..} => len.get_usize(env) <= new_size,
			Value::Boolvec(ref vec) => vec.len() <= new_size,
			Value::Bignum(ref num) => num.bit_length() <= new_size
		}
	}

	pub fn get_size(&self, env: &Environment) -> usize {
		match *self {
			Value::Pointer{ref len, ..} => len.get_usize(env),
			Value::Boolvec(ref vec) => vec.len(),
			Value::Bignum(ref num) => num.bit_length()
		}
	}

	pub fn new(value:&str) -> Option<Value> {
		let value = value.trim();
		//println!("Parsing: '{}'", value);
		let re_ptr = Regex::new(r"^\[(.*?)\]$").unwrap();

		if re_ptr.is_match(value) {
			//Is a pointer
			let mut args = ("".to_string(), "".to_string());
			let mut bracket_n = 0;
			let mut i = 0;
			let mut arg_i = 0;
			let mut from_back = false;
			for c in value.chars() {
				let skip = i < 1 || i >= value.len() - 1 || c.is_whitespace();
				i += 1;
				if skip {continue};

				if c == '[' {
					bracket_n += 1;
				} else if c == ']' {
					bracket_n -= 1;
				}
				if c == ':' && bracket_n == 0 {
					arg_i = 1;
				} else if c == '-' && args.0.len() == 0 && arg_i == 0 {
					from_back = true;
				} else  {
					if      arg_i == 0 {args.0.push(c);}
					else if arg_i == 1 {args.1.push(c);}
				}
			}
			if args.1 == "" {args.1 = "1".to_string();}
			let position = Value::new(&args.0);
			let length = Value::new(&args.1);
			return match (position, length) {
				(Some(pos_val), Some(len_val)) => {
					Some(Value::Pointer {
						pos: Box::new(pos_val),
						len: Box::new(len_val),
						rev: from_back
					})
				},
				_ => None
			}
		} else if value.chars().next() == Some('b') {
			let mut boolvec:Vec<bool> = vec![];
			boolvec.reserve(value.len() - 1);
			for c in value[1..].chars() {
				boolvec.push(match c{
					'0' => false,
					'1' => true,
					other => panic!("'{}' is not a valid character in a boolvec!", other)
				});
			}
			return Some(Value::Boolvec(boolvec));
		} else {
			//Is not a pointer
			return match gmp::mpz::Mpz::from_str(value){
				Ok(val) => Some(Value::Bignum(val)),
				Err(_) => None,
			};
		}
	}
}
