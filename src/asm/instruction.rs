use super::value::Value;
use super::environment::Environment;
use super::assembler::Assembler;
use std::ops::Neg;

pub enum Instruction {
	Push( usize ),
	Pop( usize ),
	Mov { to:Value, op:Value },

	Add { to:Value, op1:Value, op2:Value },
	Sub { to:Value, op1:Value, op2:Value },

	Jump { label:String },
	JumpEqual { label:String, op1:Value, op2:Value },
	JumpNotEqual { label:String, op1:Value, op2:Value },
	JumpGreater { label:String, op1:Value, op2:Value },
	JumpGreaterEqual { label:String, op1:Value, op2:Value },
	JumpLess { label:String, op1:Value, op2:Value },
	JumpLessEqual { label:String, op1:Value, op2:Value }
}

impl Instruction {
	pub fn get_num_arguments(name: &str) -> Option<usize> {
		match name {
			//basic
			"push" => Some(1),
			"pop" => Some(1),
			"mov" => Some(2),

			//math
			"add" => Some(3),
			"sub" => Some(3),

			//jumps
			"jmp" => Some(1),
			"je"  => Some(3),
			"jne" => Some(3),
			"jg"  => Some(3),
			"jge" => Some(3),
			"jl"  => Some(3),
			"jle" => Some(3),

			//None
			_ => None
		}
	}

	fn assert_instruction(name: &str) {
		assert!(
			Instruction::get_num_arguments(name) != None,
			format!("Instruction {} does not exist!", name)
		);
	}

	fn assert_arguments(name: &str, num_args: usize) {
		assert!(
			Some(num_args) == Instruction::get_num_arguments(name),
			format!("Instruction {} requires {} operands!", name, num_args)
		);
	}

	pub fn new(name: &str, args: &[&str]) -> Instruction {
		let num_args = args.len();
		Instruction::assert_instruction(name);
		Instruction::assert_arguments(name, num_args);
		match name {
			"push" => {
				Instruction::Push(
					args[0].parse().ok().expect("Argument 0 is not valid.")
				)
			},
			"pop" => {
				Instruction::Pop(
					args[0].parse().ok().expect("Argument 0 is not valid.")
				)
			},
			"mov" => {
				Instruction::Mov{
					to: Value::new(args[0]).expect("Argument 0 is not valid."),
					op: Value::new(args[1]).expect("Argument 1 is not valid.")
				}
			},
			"add" => {
				Instruction::Add{
					to: Value::new(args[0]).expect("Argument 0 is not valid."),
					op1: Value::new(args[1]).expect("Argument 1 is not valid."),
					op2: Value::new(args[2]).expect("Argument 2 is not valid.")
				}
			},
			"sub" => {
				Instruction::Sub{
					to: Value::new(args[0]).expect("Argument 0 is not valid."),
					op1: Value::new(args[1]).expect("Argument 1 is not valid."),
					op2: Value::new(args[2]).expect("Argument 2 is not valid.")
				}
			},
			"jmp" => {
				Instruction::Jump{
					label: args[0].trim().to_string()
				}
			},
			"je" => {
				Instruction::JumpEqual{
					label: args[0].trim().to_string(),
					op1: Value::new(args[1]).expect("Argument 1 is not valid."),
					op2: Value::new(args[2]).expect("Argument 2 is not valid.")
				}
			},
			"jne" => {
				Instruction::JumpNotEqual{
					label: args[0].trim().to_string(),
					op1: Value::new(args[1]).expect("Argument 1 is not valid."),
					op2: Value::new(args[2]).expect("Argument 2 is not valid.")
				}
			},
			"jl" => {
				Instruction::JumpLess{
					label: args[0].trim().to_string(),
					op1: Value::new(args[1]).expect("Argument 1 is not valid."),
					op2: Value::new(args[2]).expect("Argument 2 is not valid.")
				}
			},
			name => {
				panic!("No such instruction {}!", name)
			}
		}
	}

	pub fn exec(&self, env: &mut Environment) -> Option<String> {
		match *self {
			Instruction::Push(bits) => {
				env.push(bits, false);
			},
			Instruction::Pop(bits) => {
				env.pop(bits);
			},
			Instruction::Mov{ref to, ref op} => {
				if !op.can_coerce(to.get_size()) {
					panic!("Argument is bigger than assignment!");
				}
				let pos = to.get_ptr_position(&env);
				let val = op.get_bignum(&env);
				env.set_bits_bignum(&val, pos, to.get_size());
			},
			Instruction::Add{ref to, ref op1, ref op2} => {
				if !op1.can_coerce(to.get_size())
				|| !op2.can_coerce(to.get_size()) {
					panic!("Argument is bigger than assignment!");
				}
				let pos = to.get_ptr_position(&env);
				let val = op1.get_bignum(&env) + op2.get_bignum(&env);
				env.set_bits_bignum(&val, pos, to.get_size());
			},
			Instruction::Sub{ref to, ref op1, ref op2} => {
				if !op1.can_coerce(to.get_size())
				|| !op2.can_coerce(to.get_size()) {
					panic!("Argument is bigger than assignment!");
				}
				let pos = to.get_ptr_position(&env);
				let val = op1.get_bignum(&env) + op2.get_bignum(&env).neg();
				env.set_bits_bignum(&val, pos, to.get_size());
			},
			Instruction::Jump{ref label} => {
				return Some(label.clone());
			},
			Instruction::JumpEqual{ref label, ref op1, ref op2} => {
				if op1.get_bignum(env) == op2.get_bignum(env) {
					return Some(label.clone());
				}
			}
		};
		None
	}
}
