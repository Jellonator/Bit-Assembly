extern crate gmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use asm::util::*;
use std::io;
use std::io::Write;
use std::str::FromStr;
use asm::assembler::Assembler;
use asm::environment::Environment;
use std::env;
use std::collections::HashMap;

mod asm;

fn add_external_calls(asm:&mut Assembler) {
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
		unsafe {
			let s = String::from_utf8_unchecked(chars);
			print!("{}", s);
			io::stdout().flush().ok().expect("Could not flush stdout");
		}
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
	asm.add_external_call("prompt", |v,e,_|{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Invalid Input!");
		e.input_string = "".to_string();
		let mut first_line = true;
		for line in input.lines() {
			if first_line {
				first_line = false;
			} else {
				e.input_string.push('\n');
			}
			e.input_string.push_str(line.as_ref());
		}
		//let boolvec = str_to_boolvec(input.as_ref());
		//let pos = v.get_ptr_position(e);
		//let size = v.get_size(e);
		//e.set_bits_boolvec(boolvec.as_slice(), pos, size);
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
}

fn load_text(code:&str) {
	let mut env = Environment::new();
	let mut asm = Assembler::new();

	add_external_calls(&mut asm);

	for line in code.lines() {
		asm.parse_line(&line.to_string());
	}

	asm.run(&mut env);
}

fn load_file(file_name:&str) {
	let mut env = Environment::new();
	let mut asm = Assembler::new();

	add_external_calls(&mut asm);

	let file = File::open(file_name).unwrap();
	let buffer = BufReader::new(&file);
	for line in buffer.lines() {
		let l:String = line.unwrap();
		asm.parse_line(&l);
	}

	asm.run(&mut env);
}

enum Req {
	Yes,
	Maybe,
	No
}

struct ArgType {
	name:String,
	short:Option<String>,
	arg:Req
}

fn main() {
	let valid_args = vec![
		ArgType{name:"help".to_string(), short:Some("h".to_string()), arg:Req::No},
		ArgType{name:"file".to_string(), short:Some("h".to_string()), arg:Req::Yes},
		ArgType{name:"text".to_string(), short:Some("h".to_string()), arg:Req::Yes}
	];

	let mut args:HashMap<String, String> = HashMap::new();

	let mut current_arg_name:String = "file".to_string();

	let mut is_first = true;
	for element in env::args().collect::<Vec<String>>() {
		if is_first {
			is_first = false;
			continue;
		}
		//argument
		if element.len() >= 2 && &element[0..2] == "--" {
			let element = &element[2..];
			let does_contain = valid_args.iter().filter(
				|a| a.name == element
			).count() != 0;

			if !does_contain {
				panic!("No such argument of name '{}'!", element);
			}

			current_arg_name = element.to_string();

			args.insert(current_arg_name.clone(), "".to_string());

		//shortened argument
		} else if element.len() >= 1 && &element[0..1] == "-" {
			let element = &element[1..];
			let mut arg_name = "".to_string();
			for arg in &valid_args {
				if arg.short == Some(element.to_string()) {
					arg_name = arg.name.clone();
					break;
				}
			}

			if arg_name == "" {
				panic!("No such argument of name '{}'!", element);
			}

			args.insert(current_arg_name.clone(), "".to_string());

		//not an argument
		} else {
			println!("{}", element);
			let arg = args.get_mut(&current_arg_name).expect("No argument");
			arg.push_str(element.as_ref());
		}
	}

	if args.contains_key("help") {
		println!("{}",
"Welcome to Bit Assembly!
For help on how to write Bit Assembly, refer to the 'doc.md' file.

Usage:
    bit-asm --file {file name}.asm
    bit-asm --text {assembly}");

	} else if args.contains_key("file") {
		load_file(args.get("file").expect("This shouldnt happen"));

	} else if args.contains_key("text") {
		load_text(args.get("text").expect("This shouldnt happen"));

	} else {
		println!("type 'bit-asm --help' for help on how to use bit assembly");
	}
}
