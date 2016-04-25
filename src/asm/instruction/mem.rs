use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;

pub struct Push(usize);
pub struct Pop(usize);
pub struct Mov{ to: Value, from: Value }

impl Instruction for Push {
	fn new(name: &str, arguments: &[&str]) -> Box<Instruction> {
		assert!(arguments.len() == 1, "Instruction 'push' requires 1 argument.");
		Box::new(
			Push(arguments[0].parse().expect("Argument 0 is not valid."))
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		env.push(self.0, false);
	}
}

impl Instruction for Pop {
	fn new(name: &str, arguments: &[&str]) -> Box<Instruction> {
		assert!(arguments.len() == 1, "Instruction 'push' requires 1 argument.");
		Box::new(
			Pop(arguments[0].parse().expect("Argument 0 is not valid."))
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		env.pop(self.0);
	}
}

impl Instruction for Mov {
	fn new(name: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 2, "Instruction 'mov' requires 2 arguments.");
		Box::new(
			Mov {
				to: Value::new(args[0]).expect("Argument 0 is not valid."),
				from: Value::new(args[1]).expect("Argument 1 is not valid."),
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		if !self.from.can_coerce(self.to.get_size()) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.from.get_bignum(&env);
		env.set_bits_bignum(&val, pos, self.to.get_size());
	}
}
