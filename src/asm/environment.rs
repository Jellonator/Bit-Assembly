extern crate gmp;
extern crate time;
use super::util::*;
use super::assembler::Assembler;
use std::mem::size_of;

pub struct Environment {
	stack: Vec<bool>,
	callstack: Vec<usize>,
	pub instruction: usize,
	pub validity: bool,
	pub input_string: String,
	pub randstate: gmp::rand::RandState
}

#[allow(dead_code)]
impl Environment {
	pub fn new() -> Environment {
		let mut ret = Environment {
			stack: Vec::new(),
			callstack: Vec::new(),
			instruction: 0,
			validity: true,
			input_string: "".to_string(),
			randstate: gmp::rand::RandState::new()
		};
		ret.randstate.seed_ui(time::get_time().sec as u64);
		ret
	}

	pub fn stack_len(&self) -> usize {
		self.stack.len()
	}

	pub fn push(&mut self, bits: usize, value: bool) {
		let len = self.stack.len();
		self.stack.resize(len + bits, value);
	}

	pub fn pop(&mut self, bits: usize) {
		let len = self.stack.len();
		self.stack.truncate(len - bits);
	}

	pub fn slice(&self, start: usize, end: usize) -> &[bool] {
		&self.stack[start..end]
	}

	pub fn print_bytes(&self, bits_per_byte:usize) {
		for i in 0..(self.stack_len()/bits_per_byte) {
			let bits = self.slice(i*bits_per_byte, (i+1)*bits_per_byte);
			let num = boolvec_to_bignum(bits);
			print!("{}, ", num);
		}
		println!("");
	}

	pub fn set_bits_boolvec(&mut self, num: &[bool], pos:usize, len:usize) {
		for i in 0..len {
			self.stack[pos + i] = match i < num.len() {
				true => num[i],
				false => false,
			}
		}
	}

	pub fn set_bits_usize(&mut self, num: usize, pos:usize, len:usize) {
		for i in 0..len {
			self.stack[pos + i] = match i < size_of::<usize>()*8 {
				true => num & (1 << i) != 0,
				false => false,
			}
		}
	}

	pub fn set_bits_bignum(&mut self, num: &gmp::mpz::Mpz, pos:usize, len:usize) {
		self.set_bits_boolvec(&bignum_to_boolvec(num), pos, len);
	}

	pub fn call(&mut self, asm: &Assembler, name: &str) {
		self.callstack.push(self.instruction);
		self.instruction = *asm.labels.get(name).expect(
			format!("No such label of name {}!", name).as_ref()
		);
	}

	pub fn ret(&mut self) {
		let pos = self.callstack.pop().expect("Attempt to return on empty call stack!");
		self.instruction = pos;
	}

	pub fn goto(&mut self, asm: &Assembler, name: &str) {
		self.instruction = *asm.labels.get(name).expect(
			format!("No such label of name {}!", name).as_ref()
		);
	}
}
