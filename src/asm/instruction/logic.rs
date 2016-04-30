use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;
use super::super::error::*;

pub struct And { to: Value, op1: Value, op2: Value }
pub struct Or { to: Value, op1: Value, op2: Value }
pub struct Xor { to: Value, op1: Value, op2: Value }
pub struct Not { to: Value, op: Value }
pub struct LShift { to: Value, op1: Value, op2:Value }
pub struct RShift { to: Value, op1: Value, op2:Value }

impl Instruction for And {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			And {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) & self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Or {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Or {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) | self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Xor {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Xor {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) ^ self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Not {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(2));

		Box::new(
			Not {
				to: Value::new(args[0], err, true),
				op: Value::new(args[1], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op.can_coerce(self.to.get_size(env), env) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let mut val = self.op.get_boolvec(env);
		for i in 0..val.len() {
			val[i] = !val[i];
		}
		let size = self.to.get_size(env);
		val.resize(size, true);
		env.set_bits_boolvec(val.as_slice(), pos, size)
	}
}

impl Instruction for LShift {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			LShift {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) << self.op2.get_usize(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for RShift {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			RShift {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Arguments are not all same size!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) >> self.op2.get_usize(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}
