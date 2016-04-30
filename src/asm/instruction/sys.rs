extern crate gmp;

use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;
use super::super::error::*;

pub struct Ret;
pub struct Call(String);
pub struct Ext{name: String, val: Value}

impl Instruction for Ret {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(0));

		Box::new(
			Ret
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		env.ret();
	}
}

impl Instruction for Call {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(1));

		Box::new(
			Call(args[0].to_string())
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		env.call(asm, self.0.as_ref());
	}
}

impl Instruction for Ext {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Range(1, 2));

		let value = match args.len() == 2 {
			true => Value::new(args[1], err, false),
			false => Value::Bignum(gmp::mpz::Mpz::one())
		};
		Box::new(
			Ext{
				name: args[0].to_string(),
				val: value
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		let call = asm.ext_calls.get(&self.name);
		match call {
			Some(f) => f(&self.val, env, asm),
			None => panic!("No such external call of name '{}!'", self.name)
		}
	}
}
