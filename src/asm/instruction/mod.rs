use super::environment::Environment;
use super::assembler::Assembler;
mod math;
mod jump;
mod mem;
mod logic;
mod sys;

pub trait Instruction {
	fn new(name: &str, arguments: &[&str]) -> Box<Instruction> where Self:Sized;
	fn exec(&self, env: &mut Environment, asm: &Assembler);
}

pub fn create_instruction(name: &str, arguments: &[&str]) -> Box<Instruction> {
	// println!("{}: {}", name, arguments.join(", "));
	match name {
		"push" => mem::Push::new(name, arguments),
		"pop"  => mem::Pop::new(name, arguments),
		"mov"  => mem::Mov::new(name, arguments),

		"call" => sys::Call::new(name, arguments),
		"ret"  => sys::Ret::new(name, arguments),
		"ext"  => sys::Ext::new(name, arguments),

		"and"  => logic::And::new(name, arguments),
		"or"   => logic::Or::new(name, arguments),
		"xor"  => logic::Xor::new(name, arguments),
		"not"  => logic::Not::new(name, arguments),
		"shl"  => logic::LShift::new(name, arguments),
		"shr"  => logic::RShift::new(name, arguments),

		"add"  => math::Add::new(name, arguments),
		"sub"  => math::Sub::new(name, arguments),
		"mul"  => math::Mul::new(name, arguments),
		"div"  => math::Div::new(name, arguments),
		"mod"  => math::Mod::new(name, arguments),

		"jmp"  => jump::Jump::new(name, arguments),
		"je"   => jump::JumpEqual::new(name, arguments),
		"jne"  => jump::JumpNotEqual::new(name, arguments),
		"jl"   => jump::JumpLess::new(name, arguments),
		"jle"  => jump::JumpLessEqual::new(name, arguments),
		"jg"   => jump::JumpGreater::new(name, arguments),
		"jge"  => jump::JumpGreaterEqual::new(name, arguments),

		n => panic!("No such instruction {}!", n)
	}
}
