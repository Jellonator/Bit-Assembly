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
	let mut ret:u8 = 0;

	for i in 0..vec.len() {
		if vec[i]{
			ret |= 1 << i;
		}
	}

	return ret;
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
