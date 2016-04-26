use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;

pub struct And { to: Value, op1: Value, op2: Value }
pub struct Or { to: Value, op1: Value, op2: Value }
pub struct Xor { to: Value, op1: Value, op2: Value }
pub struct Not { to: Value, op: Value }
pub struct LShift { to: Value, op1: Value, op2:Value }
pub struct RShift { to: Value, op1: Value, op2:Value }

impl Instruction for And {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'and' requires 3 arguments.");
		Box::new(
			And {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) & self.op2.get_bignum(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}

impl Instruction for Or {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'sub' requires 3 arguments.");
		Box::new(
			Or {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) | self.op2.get_bignum(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}

impl Instruction for Xor {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'sub' requires 3 arguments.");
		Box::new(
			Xor {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) ^ self.op2.get_bignum(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}

impl Instruction for Not {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'sub' requires 3 arguments.");
		Box::new(
			Not {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op: Value::new(args[1]).expect("Argument 1 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op.can_coerce(self.to.get_size()) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let mut val = self.op.get_boolvec(env);
		for i in 0..val.len() {
			val[i] = !val[i];
		}
		env.set_bits_boolvec(val.as_slice(), pos, self.to.get_size())
	}
}

impl Instruction for LShift {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'sub' requires 3 arguments.");
		Box::new(
			LShift {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) << self.op2.get_usize(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}

impl Instruction for RShift {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'sub' requires 3 arguments.");
		Box::new(
			RShift {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) >> self.op2.get_usize(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}
