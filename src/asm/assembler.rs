const COMMENT_CHAR:char = ';';
const ARGUMENT_CHAR:char = ',';

use super::environment::Environment;
use super::instruction::Instruction;
use super::instruction::create_instruction;
use super::util::remove_comments;
use std::collections::HashMap;

pub struct Assembler {
	code:Vec<Box<Instruction>>,
	pub labels:HashMap<String, usize>
}

impl Assembler {
	pub fn new() -> Assembler {
		Assembler {
			code: Vec::new(),
			labels: HashMap::new()
		}
	}

	//private because reasons
	fn parse_args(&mut self, iname: &String, arguments: &[&str]){
		self.code.push(create_instruction(iname.as_ref(), arguments));
	}

	pub fn parse_line(&mut self, linearg: &String) {
		let mut line:String = linearg.trim().to_string();
		remove_comments(&mut line, COMMENT_CHAR);

		if line.chars().next() == Some('.') {
			let len = line.len();
			let name = line[1..len].to_string();
			println!("Added label: {}", name);
			self.labels.insert(name, self.code.len());
			return;
		}

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
