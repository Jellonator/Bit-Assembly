extern crate gmp;
use super::value::Value;

const COMMENT_CHAR:char = ';';
const ARGUMENT_CHAR:char = ',';

use super::environment::Environment;
use super::instruction::Instruction;
use super::instruction::create_instruction;
use super::error::*;
use super::util::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io;
use std::io::Write;
use std::str::FromStr;

pub struct Assembler {
	code:       Vec<Box<Instruction>>,
	pub labels: HashMap<String, usize>,
	defines:    Vec<(String, String)>,
	pub ext_calls:  HashMap<String, Box<Fn(&Value, &mut Environment, &Assembler)>>,
	pub print_parsed: bool
}

impl Assembler {
	fn parse_strings(&self, line:&String) -> String {
		let mut new_line:String = String::new();
		let mut is_literal_char = false;
		let mut is_in_stringval = false;
		for c in line.chars() {
			if !is_in_stringval && c == '"' {
				is_in_stringval = true;
				is_literal_char = false;
				new_line.push('b');
				continue;
			}
			if is_in_stringval {
				if c == '\\' && !is_literal_char {
					is_literal_char = true;
					continue;
				}
				if c == '"' && !is_literal_char {
					is_in_stringval = false;
					continue;
				}
				let mut char_add:char = c;
				if is_literal_char {
					char_add = match c {
						'\\' => '\\',
						'n'  => '\n',
						't'  => '\t',
						other => other
					};
					is_literal_char = false;
				}
				for b in char_to_boolvec(char_add) {
					new_line.push(match b {
						true => '1',
						false => '0'
					})
				}
			} else {
				new_line.push(c);
			}
		}
		return new_line;
	}

	pub fn add_external_call<F>(&mut self, name: &str, external: F)
		where F : 'static + Fn(&Value, &mut Environment, &Assembler) {

		self.ext_calls.insert(name.to_string(), Box::new(external));
	}

	fn add_default_external_calls(asm: &mut Assembler) {
		asm.add_external_call("printnum", |v,e,_| {
			print!("{}", boolvec_to_bignum(v.get_boolvec(e).as_slice()));
			io::stdout().flush().ok().expect("Could not flush stdout");
		});
		asm.add_external_call("print", |v,e,_| {
			let mut chars:Vec<u8> = vec![];
			let boolvec = v.get_boolvec(e);
			let val = boolvec.as_slice();
			for i in 0..(val.len()/8) {
				let nums = &val[i*8..(i+1)*8];
				let c = boolvec_to_u8(nums);
				if c == 0 {
					break;
				}
				chars.push(c);
			}
			let s = String::from_utf8_lossy(chars.as_slice()).to_string();
			print!("{}", s);
			io::stdout().flush().ok().expect("Could not flush stdout");
		});
		asm.add_external_call("valid", |v,e,_a| {
			let pos = v.get_ptr_position(e);
			let num = match e.validity {
				true => gmp::mpz::Mpz::one(),
				false => gmp::mpz::Mpz::zero()
			};
			let size = v.get_size(e);
			e.set_bits_bignum(&num, pos, size);
		});
		asm.add_external_call("prompt", |_v,e,_a|{
			let mut input = String::new();
			io::stdin().read_line(&mut input).expect("Invalid Input!");
			e.input_string = "".to_string();

			for line in input.lines() {
				e.input_string.push_str(line.as_ref());
			}
		});
		asm.add_external_call("inputnumlen", |v,e,_|{
			e.validity = true;
			let num = match gmp::mpz::Mpz::from_str(e.input_string.as_ref()) {
				Ok(val) => val,
				Err(_) => {
					e.validity = false;
					gmp::mpz::Mpz::zero()
				}
			};
			let num_size = num.bit_length();
			let pos = v.get_ptr_position(e);
			let size = v.get_ptr_size(e);
			e.set_bits_usize(num_size, pos, size);
		});
		asm.add_external_call("inputlen", |v,e,_|{
			let len_bits = e.input_string.len() * 8;
			let pos = v.get_ptr_position(e);
			let size = v.get_size(e);
			e.set_bits_usize(len_bits, pos, size);
		});
		asm.add_external_call("input", |v,e,_|{
			let boolvec = str_to_boolvec(e.input_string.as_ref());
			let pos = v.get_ptr_position(e);
			let size = v.get_size(e);
			e.set_bits_boolvec(boolvec.as_slice(), pos, size);
		});
		asm.add_external_call("inputnum", |v,e,_|{
			e.validity = true;
			let num = match gmp::mpz::Mpz::from_str(e.input_string.as_ref()) {
				Ok(val) => val,
				Err(_) => {
					e.validity = false;
					gmp::mpz::Mpz::zero()
				}
			};
			let pos = v.get_ptr_position(e);
			let size = v.get_size(e);
			e.set_bits_bignum(&num, pos, size);
		});

		asm.add_external_call("random", move |v,e,_|{
			let val = v.get_bignum(e);
			let result = e.randstate.urandom(&val);
			let pos = v.get_ptr_position(e);
			let size = v.get_ptr_size(e);
			e.set_bits_bignum(&result, pos, size);
		});
	}

	pub fn new(do_print_parsed: bool) -> Assembler {
		let mut asm = Assembler {
			code:         Vec::new(),
			labels:       HashMap::new(),
			defines:      Vec::new(),
			ext_calls:    HashMap::new(),
			print_parsed: do_print_parsed
		};
		Assembler::add_default_external_calls(&mut asm);
		asm
	}

	//private because reasons
	fn parse_args(&mut self, iname: &String, arguments: &[&str], err: &Error){
		let instruction = create_instruction(iname.as_ref(), arguments, self, err);
		self.code.push(instruction);
	}

	fn parse_macros(&mut self, line: &String, err: &Error) -> bool {
		//macro
		if line.chars().next() == Some('!') {
			let macro_text = line[1..].trim();
			let mut macro_args:Vec<&str> = macro_text.split_whitespace().collect();
			if macro_args.len() < 1 {
				err.throw(ErrorType::Empty("macro".to_string()))
			}
			let macro_name = macro_args[0];
			let macro_total_args:&str = &macro_text[macro_name.len()..].trim();
			macro_args.remove(0);
			match macro_name {
				"define" => {
					err.check_args("macro", macro_name, macro_args.len(), ArgumentType::AtLeast(2));
					let name = macro_args[0].to_string();
					let args = macro_args[1..].join(" ");
					self.defines.push((name, args));
				},
				"include" => {
					err.check_args("macro", macro_name, macro_args.len(), ArgumentType::AtLeast(1));
					let file = File::open(macro_total_args).unwrap();
					let buffer = BufReader::new(&file);
					let mut linenum = 0;
					for line in buffer.lines() {
						linenum += 1;
						let l:String = line.unwrap();
						self.parse_line(&l, linenum, Some(macro_total_args.to_string()));
					}
				},
				name => err.throw(ErrorType::NonExistent{
					typename:"macro".to_string(),
					value:name.to_string()
				})
			}
			return true;
		}
		return false;
	}

	fn parse_defines(&self, line:&String) -> String {
		let mut sepchars:Vec<char> = Vec::new();
		let strings:Vec<&str> = line.split(|c:char| {
			if c.is_whitespace() {
				sepchars.push(c);
				return true;
			}
			match c {
				',' | '[' | ']' | ':' | '-' | '<' | '>' => {
					sepchars.push(c);
					true
				},
				_ => false
			}
		}).collect::<Vec<&str>>();
		sepchars.reverse();
		strings.iter().map(|w|{
			let mut ret = String::new();
			let mut did = false;
			for def in &self.defines {
				if def.0 == *w {
					ret.push_str(&format!("{}", def.1));
					did = true;
					break;
				}
			}
			if !did {
				ret.push_str(&format!("{}", w));
			}
			match sepchars.pop() {
				Some(c) => ret.push(c),
				None => {}
			};
			ret
		}).collect::<String>()
	}

	fn parse_labels(&mut self, line:&String) -> bool {
		if line.chars().next() == Some('.') {
			let name = line[1..].to_string();
			self.labels.insert(name, self.code.len());
			return true;
		}
		return false;
	}

	pub fn parse_line(&mut self, linearg: &String, linenum: usize, filename: Option<String>) {
		//trim and remove comments
		let mut line:String = linearg.to_string();
		remove_comments(&mut line, COMMENT_CHAR);
		line = line.trim().to_string();

		//Create error handler
		let err = Error::new(line.clone(), linenum, filename);

		//parse strings into 'b10101010' format
		line = self.parse_strings(&line);

		//use macros and labels
		if self.parse_macros(&line, &err) {
			return;
		}
		if self.parse_labels(&line) {
			return;
		}

		line = self.parse_defines(&line);

		//parse name and arguments
		let mut arg_string:String = String::new();
		let name:String = match line.split_whitespace().next(){
			Some(val) => {val.to_string()},
			None => return,//No instruction, skip this line
		};
		match line.find(name.as_str()){
			Some(pos) => {arg_string = line[(pos+name.len())..].trim().to_string();}
			None => {},
		}
		let arg_vec:Vec<&str> = arg_string
			.split(ARGUMENT_CHAR)
			.map(|val|val.trim())
			.filter(|val|val.trim() != "")
			.collect();

		self.parse_args(&name, &arg_vec, &err);
	}

	pub fn run(&mut self, env: &mut Environment) {
		let len = self.code.len();
		while env.instruction < len {
			let ins = &self.code[env.instruction];
			env.instruction += 1;
			ins.exec(env, self);
		}
	}
}
