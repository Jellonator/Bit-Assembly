use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;
use super::super::error::*;

pub struct Add{ to: Value, op1: Value, op2: Value }
pub struct Sub{ to: Value, op1: Value, op2: Value }
pub struct Mul{ to: Value, op1: Value, op2: Value }
pub struct Div{ to: Value, op1: Value, op2: Value }
pub struct Mod{ to: Value, op1: Value, op2: Value }

impl Instruction for Add {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Add {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) + self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Sub {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Sub {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) - self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Mul {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Mul {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) * self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Div {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Div {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) / self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}

impl Instruction for Mod {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			Mod {
				to:  Value::new(args[0], err, true),
				op1: Value::new(args[1], err, false),
				op2: Value::new(args[2], err, false)
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size(env), env)
		|| !self.op2.can_coerce(self.to.get_size(env), env) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) % self.op2.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}
