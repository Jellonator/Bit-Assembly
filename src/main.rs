use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use asm::util::*;
use std::io;
use std::io::Write;
use std::str::FromStr;
extern crate gmp;

mod asm;

fn main() {
	// let mut data = String::new();
	let mut env = asm::environment::Environment::new();
	let mut asm = asm::assembler::Assembler::new();

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
			chars.push(boolvec_to_u8(nums));
		}
		unsafe {
			let s = String::from_utf8_unchecked(chars);
			print!("{}", s);
			io::stdout().flush().ok().expect("Could not flush stdout");
		}
	});
	
	asm.add_external_call("valid", |v,e,_a|{
		let pos = v.get_ptr_position(e);
		let num = match e.validity {
			true => gmp::mpz::Mpz::one(),
			false => gmp::mpz::Mpz::zero()
		};
		e.set_bits_bignum(&num, pos, v.get_size());
	});
	asm.add_external_call("input", |v,e,_|{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Invalid Input!");
		let boolvec = str_to_boolvec(input.as_ref());
		let pos = v.get_ptr_position(e);
		e.set_bits_boolvec(boolvec.as_slice(), pos, v.get_size());
	});
	asm.add_external_call("inputnum", |v,e,_|{
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Invalid Input!");
		e.validity = true;
		let num = match gmp::mpz::Mpz::from_str(input.as_ref()) {
			Ok(val) => val,
			Err(_) => {
				e.validity = false;
				gmp::mpz::Mpz::zero()
			}
		};
		let pos = v.get_ptr_position(e);
		e.set_bits_bignum(&num, pos, v.get_size());
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
