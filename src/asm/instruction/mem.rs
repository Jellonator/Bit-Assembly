use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;

pub struct Push( Value, Option<Value> );
pub struct Pop( Value);
pub struct Mov{ to: Value, from: Value }

impl Instruction for Push {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(
			args.len() == 1 || args.len() == 2,
			"Instruction 'push' requires 1 or 2 arguments."
		);
		let val = match args.len() >= 2 {
			true => Value::new(args[1]),
			false => None
		};
		Box::new(
			Push(
				Value::new(args[0]).expect("Argument 0 is not valid."),
				val
			)
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		let size = self.0.get_usize(env);
		let pos = env.stack_len();
		env.push(size, false);
		match self.1 {
			Some(ref val) => {
				let num = val.get_bignum(env);
				env.set_bits_bignum(&num, pos, size);
			},
			None => {}
		}
	}
}

impl Instruction for Pop {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 1, "Instruction 'pop' requires 1 arguments.");
		Box::new(
			Pop(Value::new(args[0]).expect("Argument 0 is not valid."))
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		let size = self.0.get_usize(env);
		env.pop(size);
	}
}

impl Instruction for Mov {
	fn new(_: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 2, "Instruction 'mov' requires 2 arguments.");
		Box::new(
			Mov {
				to: Value::new(args[0]).expect("Argument 0 is not valid."),
				from: Value::new(args[1]).expect("Argument 1 is not valid."),
			}
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		if !self.from.can_coerce(self.to.get_size(env), env) {
			panic!("Argument is bigger than assignment!");
		}
		let pos = self.to.get_ptr_position(&env);
		let val = self.from.get_bignum(&env);
		let size = self.to.get_size(env);
		env.set_bits_bignum(&val, pos, size);
	}
}
