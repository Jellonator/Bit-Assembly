use super::super::value::Value;
use super::super::environment::Environment;
use super::super::assembler::Assembler;
use super::Instruction;

pub struct Ret;
pub struct Call(String);
pub struct Ext{name: String, val: Value}

impl Instruction for Ret {
	fn new(name: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 0, "Instruction 'ret' takes no arguments.");
		Box::new(
			Ret
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		env.ret();
	}
}

impl Instruction for Call {
	fn new(name: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 1, "Instruction 'call' takes 1 argument.");
		Box::new(
			Call(args[0].to_string())
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		env.call(asm, self.0.as_ref());
	}
}

impl Instruction for Ext {
	fn new(name: &str, args: &[&str]) -> Box<Instruction> {
		assert!(args.len() == 2, "Instruction 'ext' takes 2 arguments.");
		Box::new(
			Ext{
				name: args[0].to_string(),
				val: Value::new(args[1]).expect("Argument 1 is invalid.")
			}
		)
	}

	fn exec(&self, env: &mut Environment, asm: &Assembler) {
		let call = asm.ext_calls.get(&self.name);
		match call {
			Some(f) => f(self.val.get_boolvec(env).as_slice()),
			None => panic!("No such external call of name '{}!'", self.name)
		}
	}
}
