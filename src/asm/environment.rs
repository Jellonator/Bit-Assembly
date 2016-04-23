extern crate gmp;
use super::util::*;
use std::cmp;

pub struct Environment {
	stack:Vec<bool>
}

impl Environment {
	pub fn new() -> Environment {
		Environment {stack: Vec::new()}
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

	pub fn set_bits_bignum(&mut self, num: &gmp::mpz::Mpz, pos:usize, len:usize) {
		self.set_bits_boolvec(&bignum_to_boolvec(num), pos, len);
	}
}
