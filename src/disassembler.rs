use std::fs;

use crate::bytecode::parse_file;
use crate::errors::{MiniVmError, Result};
use crate::isa::{Op, decode, to_asm};

pub fn command(args: &[String]) -> Result<()> {
    if args.len() != 3 {
        return Err(MiniVmError::Parse("usage: minivm dis file.tbc".into()));
    }

    disassemble(&args[2])
}

pub fn disassemble(file: &str) -> Result<()> {
    let bytes = fs::read(file)?;

    let code = parse_file(&bytes)?;

    let asm = decode_program(&code)?;

    print!("{}", asm);

    Ok(())
}

fn decode_program(code: &[u8]) -> Result<String> {
    let mut ip = 0;

    let mut output = String::new();

    while ip < code.len() {
        let op = decode(code, &mut ip)?;

        output.push_str(&to_asm(&op));

        output.push('\n');

        if matches!(op, Op::Halt) {
            break;
        }
    }

    Ok(output)
}

pub fn disassemble_bytes(code: &[u8]) -> Result<String> {
    decode_program(code)
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::isa::Op;

    #[test]
    fn simple_program() {
        let mut code = Vec::new();

        Op::Push(7).encode(&mut code);

        Op::Push(3).encode(&mut code);

        Op::Add.encode(&mut code);

        Op::Halt.encode(&mut code);

        let text = disassemble_bytes(&code).unwrap();

        assert_eq!(text, "PUSH 7\nPUSH 3\nADD\nHALT\n");
    }
}
