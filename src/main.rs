use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
extern crate gmp;

mod asm;
use asm::value::Value;

fn print_value(value: &Option<Value>) {
	match *value {
		Some(ref val) => println!("{}", val),
		None => println!("None")
	}
}

fn main() {
	// let mut data = String::new();
	let mut env = asm::environment::Environment::new();
	let mut asm = asm::assembler::Assembler::new();

	let file = File::open("thing.asm").unwrap();
	let buffer = BufReader::new(&file);
	for line in buffer.lines() {
		let l:String = line.unwrap();
		asm.parse_line(&l);
	}

	asm.run(&mut env);
	env.print_bytes( 32);
}
