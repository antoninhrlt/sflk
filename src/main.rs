
struct Settings {
	src_filename: String,
	debug_mode: bool,
}

impl Settings {
	fn from_args() -> Settings {
		let mut args = std::env::args();
		args.next();
		let src_filename = args.next().expect("no source file provided");
		let mut debug_mode = false;
		for arg in args {
			if arg == "-d" {
				debug_mode = true;
			} else {
				panic!("unknown command line argument `{}`", arg);
			}
		}
		Settings {
			src_filename,
			debug_mode,
		}
	}
}

use std::rc::Rc;
mod tokenizer;
mod parser;
mod machine;
mod stringtree;

fn main() {
	let settings = Settings::from_args();

	let scu = Rc::new(tokenizer::SourceCodeUnit::from_filename(
		&settings.src_filename));

	let mut prh = parser::ProgReadingHead::from(
		tokenizer::TokReadingHead::from_scu(scu));
	let prog = match prh.parse_prog() {
		Ok((prog, _)) => prog,
		Err(parsing_error) => {
			println!("\x1b[91m\x1b[1mParsing error:\x1b[22m\x1b[39m {}", parsing_error);
			return;
		},
	};
	if settings.debug_mode {
		stringtree::StringTree::from(&prog).print();
	}

	let mut mem = machine::Mem::new();
	mem.exec_prog(&prog);
}
