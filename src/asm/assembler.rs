const COMMENT_CHAR:char = ';';
const ARGUMENT_CHAR:char = ',';

use super::environment::Environment;
use super::instruction::Instruction;
use super::util::remove_comments;
use std::collections::HashMap;

pub struct Assembler {
	code:Vec<Instruction>,
	labels:HashMap<String, usize>,
	instruction: usize
}

impl Assembler {
	pub fn new() -> Assembler {
		Assembler {
			code: Vec::new(),
			labels: HashMap::new(),
			instruction: 0
		}
	}

	//private because reasons
	fn parse_args(&mut self, iname: &String, arguments: &[&str]){
		self.code.push(Instruction::new(iname.as_ref(), arguments));
	}

	pub fn parse_line(&mut self, linearg: &String) {
		let mut line:String = linearg.clone();
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

		let arg_vec:Vec<&str> = arg_string.split(ARGUMENT_CHAR).collect();

		self.parse_args(&name, &arg_vec);
	}

	pub fn run(&mut self, env: &mut Environment) {
		let len = self.code.len();
		while self.instruction < len {
			let mut to_label = None;

			{
				let ins = &self.code[self.instruction];
				self.instruction += 1;
				to_label = ins.exec(env);
			}

			match to_label {
				Some(label_name) => self.goto(&label_name),
				_ => {}
			}
		}
	}

	pub fn goto(&mut self, label: &String){
		match self.labels.get(label) {
			Some(pos) => self.instruction = *pos,
			None => panic!("No such label name of {}!", label)
		}
	}
}
