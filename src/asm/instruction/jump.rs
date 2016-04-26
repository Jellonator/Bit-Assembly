use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;

pub struct Jump { label:String }
pub struct JumpEqual { label:String, op1:Value, op2:Value }
pub struct JumpNotEqual { label:String, op1:Value, op2:Value }
pub struct JumpGreater { label:String, op1:Value, op2:Value }
pub struct JumpGreaterEqual { label:String, op1:Value, op2:Value }
pub struct JumpLess { label:String, op1:Value, op2:Value }
pub struct JumpLessEqual { label:String, op1:Value, op2:Value }

impl Instruction for Jump {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 1, "Instruction 'jmp' requires 1 argument.");
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
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'je' requires 3 arguments.");
		Box::new(
			JumpEqual {
				op1:   Value::new(args[0]).expect("Argument 0 is not valid."),
				op2:   Value::new(args[1]).expect("Argument 1 is not valid."),
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
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'jne' requires 3 arguments.");
		Box::new(
			JumpNotEqual {
				op1:   Value::new(args[0]).expect("Argument 0 is not valid."),
				op2:   Value::new(args[1]).expect("Argument 1 is not valid."),
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
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'jg' requires 3 arguments.");
		Box::new(
			JumpGreater {
				op1:   Value::new(args[0]).expect("Argument 0 is not valid."),
				op2:   Value::new(args[1]).expect("Argument 1 is not valid."),
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
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'jge' requires 3 arguments.");
		Box::new(
			JumpGreaterEqual {
				op1:   Value::new(args[0]).expect("Argument 0 is not valid."),
				op2:   Value::new(args[1]).expect("Argument 1 is not valid."),
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
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'jl' requires 3 arguments.");
		Box::new(
			JumpLess {
				op1:   Value::new(args[0]).expect("Argument 0 is not valid."),
				op2:   Value::new(args[1]).expect("Argument 1 is not valid."),
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
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'jle' requires 3 arguments.");
		Box::new(
			JumpLessEqual {
				op1:   Value::new(args[0]).expect("Argument 0 is not valid."),
				op2:   Value::new(args[1]).expect("Argument 1 is not valid."),
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
