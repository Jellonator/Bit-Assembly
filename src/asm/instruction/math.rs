use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;

pub struct Add{ to: Value, op1: Value, op2: Value }
pub struct Sub{ to: Value, op1: Value, op2: Value }

impl Instruction for Add {
	fn new(name: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'add' requires 3 arguments.");
		Box::new(
			Add {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) + self.op2.get_bignum(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}

impl Instruction for Sub {
	fn new(name: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 3, "Instruction 'sub' requires 3 arguments.");
		Box::new(
			Sub {
				to:  Value::new(args[0]).expect("Argument 0 is invalid."),
				op1: Value::new(args[1]).expect("Argument 1 is invalid."),
				op2: Value::new(args[2]).expect("Argument 2 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if !self.op1.can_coerce(self.to.get_size())
		|| !self.op2.can_coerce(self.to.get_size()) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.op1.get_bignum(&env) - self.op2.get_bignum(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}
