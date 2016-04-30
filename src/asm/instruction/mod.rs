use super::environment::Environment;
use super::assembler::Assembler;
use super::error::*;
mod math;
mod jump;
mod mem;
mod logic;
mod sys;

pub trait Instruction {
	fn new(name: &str, arguments: &[&str], err: &Error) -> Box<Instruction> where Self:Sized;
	fn exec(&self, env: &mut Environment, asm: &Assembler);
}

pub fn create_instruction(name: &str, arguments: &[&str], asm: &Assembler, err: &Error) -> Box<Instruction> {
	if asm.print_parsed {
		println!("{}: {}", name, arguments.join(", "));
	}
	match name {
		"push" => mem::Push::new(name, arguments, err),
		"pop"  => mem::Pop::new(name, arguments, err),
		"mov"  => mem::Mov::new(name, arguments, err),

		"call" => sys::Call::new(name, arguments, err),
		"ret"  => sys::Ret::new(name, arguments, err),
		"ext"  => sys::Ext::new(name, arguments, err),

		"and"  => logic::And::new(name, arguments, err),
		"or"   => logic::Or::new(name, arguments, err),
		"xor"  => logic::Xor::new(name, arguments, err),
		"not"  => logic::Not::new(name, arguments, err),
		"shl"  => logic::LShift::new(name, arguments, err),
		"shr"  => logic::RShift::new(name, arguments, err),

		"add"  => math::Add::new(name, arguments, err),
		"sub"  => math::Sub::new(name, arguments, err),
		"mul"  => math::Mul::new(name, arguments, err),
		"div"  => math::Div::new(name, arguments, err),
		"mod"  => math::Mod::new(name, arguments, err),

		"jmp"  => jump::Jump::new(name, arguments, err),
		"je"   => jump::JumpEqual::new(name, arguments, err),
		"jne"  => jump::JumpNotEqual::new(name, arguments, err),
		"jl"   => jump::JumpLess::new(name, arguments, err),
		"jle"  => jump::JumpLessEqual::new(name, arguments, err),
		"jg"   => jump::JumpGreater::new(name, arguments, err),
		"jge"  => jump::JumpGreaterEqual::new(name, arguments, err),

		n => err.throw(ErrorType::NonExistent{
			typename: "instruction".to_string(),
			value: n.to_string()
		})
	}
}
