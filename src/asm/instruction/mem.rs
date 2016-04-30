use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;
use super::super::error::*;

pub struct Push( Value, Option<Value> );
pub struct Pop( Value);
pub struct Mov{ to: Value, from: Value }

impl Instruction for Push {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Range(1, 2));

		let val = match args.len() >= 2 {
			true => Some(Value::new(args[1], err, false)),
			false => None
		};
		Box::new(
			Push(
				Value::new(args[0], err, false),
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
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(1));

		Box::new(
			Pop(Value::new(args[0], err, false))
		)
	}

	fn exec(&self, env: &mut Environment, _: &Assembler) {
		let size = self.0.get_usize(env);
		env.pop(size);
	}
}

impl Instruction for Mov {
	fn new(name: &str, args: &[&str], err: &Error) -> Box<Instruction> {
		err.check_args("instruction", name, args.len(), ArgumentType::Exact(2));

		Box::new(
			Mov {
				to: Value::new(args[0], err, true),
				from: Value::new(args[1], err, false),
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
