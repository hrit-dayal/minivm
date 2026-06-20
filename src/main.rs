use minivm::{assembler, disassembler, vm};

use std::env;
use std::process;

fn usage() -> ! {
    eprintln!(
        "Usage:
  minivm asm <input.tasm> -o <output.tbc>
  minivm run <input.tbc>
  minivm run --trace <input.tbc>
  minivm dis <input.tbc>"
    );

    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    let result = match args[1].as_str() {
        "asm" => assembler::command(&args),
        "run" => vm::command(&args),
        "dis" => disassembler::command(&args),
        _ => {
            usage();
        }
    };

    if let Err(e) = result {
        eprintln!("{e}");
        process::exit(1);
    }
}
