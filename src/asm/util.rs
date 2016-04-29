#![allow(dead_code)]
extern crate gmp;

pub fn remove_comments(value: &mut String, comment: char) {
	match value.find(comment) {
		Some(pos) => {
			value.truncate(pos);
		},
		_ => {}
	}
}

pub fn boolvec_to_bignum(vec:&[bool]) -> gmp::mpz::Mpz {
	let mut ret = gmp::mpz::Mpz::zero();
	ret.reserve(vec.len());
	for i in 0..vec.len() {
		match vec[i]{
			true  => ret.setbit(i),
			false => ret.clrbit(i)
		}
	}

	return ret;
}

pub fn boolvec_to_usize(vec:&[bool]) -> usize {
	let mut ret:usize = 0;

	for i in 0..vec.len() {
		if vec[i]{
			ret |= 1 << i;
		}
	}

	return ret;
}

pub fn boolvec_to_u8(vec:&[bool]) -> u8 {
	//backwards from other vec -> num implementations,
	//printed characters have different endianess
	let mut ret:u8 = 0;

	for i in 0..vec.len() {
		if vec[i]{
			ret |= 128 >> i;
		}
	}

	return ret;
}

pub fn char_to_boolvec(c:char) -> Vec<bool> {
	let mut temp_string:String = String::new();
	temp_string.push(c);
	return str_to_boolvec(temp_string.as_ref());
}

pub fn bignum_to_usize(num: &gmp::mpz::Mpz) -> usize {
	let mut ret:usize = 0;

	for i in 0..num.bit_length() {
		if num.tstbit(i) {
			ret |= 1 << i;
		}
	}

	return ret;
}

pub fn bignum_to_boolvec(num: &gmp::mpz::Mpz) -> Vec<bool> {
	let num_bits = num.bit_length();
	let mut ret = vec![false; num_bits];
	for i in 0..num_bits {
		ret[i] = num.tstbit(i);
	}
	ret
}

pub fn str_to_boolvec(s:&str) -> Vec<bool> {
	let mut ret:Vec<bool> = vec![];

	for b in s.as_bytes() {
		for i in 0..8 {
			ret.push((*b as u32) & (128 >> i) != 0);
		}
	}

	ret
}

pub fn usize_len(num:usize) -> usize {
	let mut ret = 0;
	let mut num = num;
	while num != 0 {
		ret += 1;
		num /= 2;
	}
	ret
}

pub fn usize_to_boolvec(num: usize) -> Vec<bool> {
	let mut ret = vec![];
	let mut num = num;
	while num != 0 {
		ret.push(num % 2 == 1);
		num /= 2;
	}
	ret
}

pub fn usize_to_bignum(num: usize) -> gmp::mpz::Mpz {
	let mut ret = gmp::mpz::Mpz::zero();
	let mut i = 0;
	let mut num = num;
	while num != 0 {
		match num % 2 == 1 {
			true => ret.setbit(i),
			false => ret.clrbit(i),
		}
		num /= 2;
		i += 1;
	}
	ret
}
