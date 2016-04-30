use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;
use super::super::error::*;

pub struct Jump { label:String }
pub struct JumpEqual { label:String, op1:Value, op2:Value }
pub struct JumpNotEqual { label:String, op1:Value, op2:Value }
pub struct JumpGreater { label:String, op1:Value, op2:Value }
pub struct JumpGreaterEqual { label:String, op1:Value, op2:Value }
pub struct JumpLess { label:String, op1:Value, op2:Value }
pub struct JumpLessEqual { label:String, op1:Value, op2:Value }

impl Instruction for Jump {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(1));

		Box::new(
			Jump { label: args[0].to_string() }
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		//Some(self.label.clone())
		env.goto(asm, self.label.as_ref());
	}
}

impl Instruction for JumpEqual {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			JumpEqual {
				op1:   Value::new(args[0], err, false),
				op2:   Value::new(args[1], err, false),
				label: args[2].to_string(),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if self.op1.get_bignum(env) == self.op2.get_bignum(env) {
			env.goto(asm, self.label.as_ref());
		}
	}
}

impl Instruction for JumpNotEqual {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			JumpNotEqual {
				op1:   Value::new(args[0], err, false),
				op2:   Value::new(args[1], err, false),
				label: args[2].to_string(),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if self.op1.get_bignum(env) != self.op2.get_bignum(env) {
			env.goto(asm, self.label.as_ref());
		}
	}
}

impl Instruction for JumpGreater {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			JumpGreater {
				op1:   Value::new(args[0], err, false),
				op2:   Value::new(args[1], err, false),
				label: args[2].to_string(),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if self.op1.get_bignum(env) > self.op2.get_bignum(env) {
			env.goto(asm, self.label.as_ref());
		}
	}
}

impl Instruction for JumpGreaterEqual {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			JumpGreaterEqual {
				op1:   Value::new(args[0], err, false),
				op2:   Value::new(args[1], err, false),
				label: args[2].to_string(),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if self.op1.get_bignum(env) >= self.op2.get_bignum(env) {
			env.goto(asm, self.label.as_ref());
		}
	}
}

impl Instruction for JumpLess {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			JumpLess {
				op1:   Value::new(args[0], err, false),
				op2:   Value::new(args[1], err, false),
				label: args[2].to_string(),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if self.op1.get_bignum(env) < self.op2.get_bignum(env) {
			env.goto(asm, self.label.as_ref());
		}
	}
}

impl Instruction for JumpLessEqual {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(3));

		Box::new(
			JumpLessEqual {
				op1:   Value::new(args[0], err, false),
				op2:   Value::new(args[1], err, false),
				label: args[2].to_string(),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if self.op1.get_bignum(env) <= self.op2.get_bignum(env) {
			env.goto(asm, self.label.as_ref());
		}
	}
}
