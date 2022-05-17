mod ast;
mod log;
mod machine;
mod machine2;
mod object;
mod parser;
mod program;
mod scu;
mod stringtree;
mod tokenizer;
mod utils;

const HELP_MESSAGE: &str = "\
	Usage:\n\
	\tsflk <filename.sflk> [options]\n\
	\n\
	Options:\n\
	\t-d --debug    Turns on debug mode\n\
	\t-h --help     Prints this help message\n\
	\t-v --version  Prints the interpreter version\n\
	";

const NO_WARRANTY_NOTE: &str = "\
	Please note that there is NO warranty, \
	not even for MERCHANTABILITY or \
	FITNESS FOR A PARTICULAR PURPOSE.";

struct Settings {
	path: String,
	root_filename: Option<String>,
	debug_mode: bool,
	wants_help: bool,
	wants_version: bool,
}

impl Settings {
	fn from_args() -> Settings {
		let mut args = std::env::args();
		let mut settings = Settings {
			path: args.next().unwrap_or_else(|| "sflk".to_string()),
			root_filename: None,
			debug_mode: false,
			wants_help: false,
			wants_version: false,
		};
		for arg in args {
			if arg == "-d" || arg == "--debug" {
				settings.debug_mode = true;
			} else if arg == "-h" || arg == "--help" {
				settings.wants_help = true;
			} else if arg == "-v" || arg == "--version" {
				settings.wants_version = true;
			} else if settings.root_filename.is_none() {
				settings.root_filename = Some(arg);
			} else {
				panic!("unknown command line argument `{}`", arg);
			}
		}
		settings
	}
}

fn main() {
	let settings = Settings::from_args();

	let mut did_something = false;
	if settings.wants_version {
		let version_name = "indev";
		println!(
			"SFLK reference interpreter, version {}.{}.{} ({})",
			0, 1, 0, version_name
		);
		println!("{}", NO_WARRANTY_NOTE);
		did_something = true;
	}
	if settings.wants_help {
		print!("{}", HELP_MESSAGE);
		did_something = true;
	}
	if settings.root_filename.is_none() && !did_something {
		println!(
			"No filename provided, nothing to do. Try `{} --help` for usage.",
			settings.path
		);
	}

	let mut mem = machine::Mem::new(settings.debug_mode);
	mem.exec_file(settings.root_filename.unwrap());
	if let Some(debug_mem) = mem.debug_mem_opt {
		debug_mem.log.print_to_stdout();
	}
}
