use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use asm::util::*;
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

	asm.add_external_call("printnum", |a| print!("{}", boolvec_to_bignum(a)));
	asm.add_external_call("print", |a| {
		let mut chars:Vec<u8> = vec![];
		for i in 0..(a.len()/8) {
			let nums = &a[i*8..(i+1)*8];
			chars.push(boolvec_to_u8(nums));
		}
		unsafe {
			let s = String::from_utf8_unchecked(chars);
			print!("{}", s);
		}
	});

	let file = File::open("thing.asm").unwrap();
	let buffer = BufReader::new(&file);
	for line in buffer.lines() {
		let l:String = line.unwrap();
		asm.parse_line(&l);
	}

	asm.run(&mut env);
	//env.print_bytes( 32);
	println!("");
}
