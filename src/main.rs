extern crate gmp;
extern crate time;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use asm::assembler::Assembler;
use asm::environment::Environment;
use std::env;
use std::collections::HashMap;

mod asm;

fn load_text(asm: &mut Assembler, code: &str) {
	let mut linenum = 0;
	for line in code.lines() {
		for s in line.split(';') {
			linenum += 1;
			asm.parse_line(&s.to_string(), linenum, None);
		}
	}
}

fn load_file(asm: &mut Assembler, file_name:&str) {
	let file = File::open(file_name).unwrap();
	let buffer = BufReader::new(&file);
	let mut linenum = 0;
	for line in buffer.lines() {
		let l:String = line.unwrap();
		linenum += 1;
		asm.parse_line(&l, linenum, Some(file_name.to_string()));
	}
}

enum Req {
	Yes,
	Maybe,
	No
}

struct ArgType {
	name:String,
	short:Option<String>,
	arg:Req
}

fn main() {
	let valid_args = vec![
		ArgType{name:"help".to_string(), short:Some("h".to_string()), arg:Req::No},
		ArgType{name:"file".to_string(), short:Some("h".to_string()), arg:Req::Yes},
		ArgType{name:"text".to_string(), short:Some("h".to_string()), arg:Req::Yes},
		ArgType{name:"print-stack".to_string(),  short:Some("s".to_string()), arg:Req::Maybe},
		ArgType{name:"print-parsed".to_string(), short:Some("p".to_string()), arg:Req::No},
	];

	let mut args:HashMap<String, String> = HashMap::new();

	let mut current_arg_name:String = "file".to_string();

	let mut is_first = true;
	for element in env::args().collect::<Vec<String>>() {
		if is_first {
			is_first = false;
			continue;
		}
		//argument
		if element.len() >= 2 && &element[0..2] == "--" {
			let element = &element[2..];
			let does_contain = valid_args.iter().filter(
				|a| a.name == element
			).count() != 0;

			if !does_contain {
				panic!("No such argument of name '{}'!", element);
			}

			current_arg_name = element.to_string();

			args.insert(current_arg_name.clone(), "".to_string());

		//shortened argument
		} else if element.len() >= 1 && &element[0..1] == "-" {
			let element = &element[1..];
			let mut arg_name = "".to_string();
			for arg in &valid_args {
				if arg.short == Some(element.to_string()) {
					arg_name = arg.name.clone();
					break;
				}
			}

			if arg_name == "" {
				panic!("No such argument of name '{}'!", element);
			} else {
				current_arg_name = arg_name;
			}

			args.insert(current_arg_name.clone(), "".to_string());

		//not an argument
		} else {
			println!("{}", element);
			let arg = args.get_mut(&current_arg_name).expect("No argument");
			arg.push_str(element.as_ref());
		}
	}

	let do_print_parsed = args.contains_key("print-parsed");
	let do_stack_print = args.contains_key("print-stack");

	let mut env = Environment::new();
	let mut asm = Assembler::new(do_print_parsed);
	let mut do_run = false;

	if args.contains_key("help") {
		println!("{}",
"Welcome to Bit Assembly!
For help on how to write Bit Assembly, refer to the 'doc.md' file.

Usage:
    bit-asm --file {file name}.asm
    bit-asm --text {assembly}

Options:
    --print-stack {bits}    prints stack as a sequence of bytes
    --print-parsed          prints each line as they are parsed");

	} else if args.contains_key("file") {
		load_file(&mut asm, args.get("file").expect("This shouldnt happen"));
		do_run = true;

	} else if args.contains_key("text") {
		load_text(&mut asm, args.get("text").expect("This shouldnt happen"));
		do_run = true;

	} else {
		println!("type 'bit-asm --help' for help on how to use bit assembly");
	}
	if do_run {
		asm.run(&mut env);
		if do_stack_print {
			let bits = usize::from_str(
				match args.get("print-stack").unwrap_or(&"64".to_string()).as_ref() {
					"" => "64",
					other => other
				}
			).expect("print-stack argument is not valid!");
			env.print_bytes(bits);
		}
	}
}
