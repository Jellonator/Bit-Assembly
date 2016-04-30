extern crate gmp;
use super::value::Value;

const COMMENT_CHAR:char = ';';
const ARGUMENT_CHAR:char = ',';

use super::environment::Environment;
use super::instruction::Instruction;
use super::instruction::create_instruction;
use super::error::Error;
use super::util::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

	pub fn new(do_print_parsed: bool) -> Assembler {
		Assembler {
			code:         Vec::new(),
			labels:       HashMap::new(),
			defines:      Vec::new(),
			ext_calls:    HashMap::new(),
			print_parsed: do_print_parsed
		}
	}

	//private because reasons
	fn parse_args(&mut self, iname: &String, arguments: &[&str]){
		let instruction = create_instruction(iname.as_ref(), arguments, self);
		self.code.push(instruction);
	}

	fn parse_macros(&mut self, line: &String) -> bool {
		//macro
		if line.chars().next() == Some('!') {
			let macro_text = line[1..].trim();
			let macro_args:Vec<&str> = macro_text.split_whitespace().collect();
			let macro_total_args:&str = &macro_text[macro_args[0].len()..].trim();
			if macro_args.len() < 1 {
				panic!("Empty macro!");
			}
			match macro_args[0] {
				"define" => {
					if macro_args.len() < 3 {
						panic!("Macro 'define' requires at least 3 arguments!");
					}
					let name = macro_args[1].to_string();
					let args = macro_args[2..].join(" ");
					//println!("Defined '{}' as '{}'", name, args);
					self.defines.push((name, args));
				},
				"include" => {
					let file = File::open(macro_total_args).unwrap();
					let buffer = BufReader::new(&file);
					let mut linenum = 0;
					for line in buffer.lines() {
						linenum += 1;
						let l:String = line.unwrap();
						self.parse_line(&l, linenum, Some(macro_total_args.to_string()));
					}
				},
				name => panic!("Unknown macro name {}!", name)
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
		let error = Error::new(line.clone(), linenum, filename);

		//parse strings into 'b10101010' format
		line = self.parse_strings(&line);

		//use macros and labels
		if self.parse_macros(&line) {
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

		self.parse_args(&name, &arg_vec);
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
