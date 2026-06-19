use std::fs;

use crate::bytecode::build_file;
use crate::errors::{MiniVmError, Result};
use crate::isa::Op;

pub fn command(args: &[String]) -> Result<()> {
    if args.len() != 5 {
        return Err(MiniVmError::Parse(
            "usage: minivm asm input.tasm -o output.tbc".into(),
        ));
    }

    if args[3] != "-o" {
        return Err(MiniVmError::Parse("expected -o".into()));
    }

    assemble(&args[2], &args[4])
}

pub fn assemble(input: &str, output: &str) -> Result<()> {
    let source = fs::read_to_string(input)?;

    let mut code = Vec::new();

    let mut last_was_halt = false;

    for (line_no, raw) in source.lines().enumerate() {
        let line_no = line_no + 1;

        let line = raw.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        let op = parse_instruction(line, line_no)?;

        last_was_halt = matches!(op, Op::Halt);

        op.encode(&mut code);
    }

    if !last_was_halt {
        eprintln!("warning: program does not end with HALT");
    }

    let file = build_file(&code);

    fs::write(output, file)?;

    Ok(())
}

fn parse_instruction(line: &str, line_no: usize) -> Result<Op> {
    let parts: Vec<_> = line.split_whitespace().collect();

    if parts.is_empty() {
        return Err(MiniVmError::Parse(format!(
            "line {}: empty instruction",
            line_no
        )));
    }

    let mnemonic = parts[0].to_ascii_uppercase();

    match mnemonic.as_str() {
        "PUSH" => {
            expect_count(line_no, &parts, 2)?;

            let value = parse_i64(line_no, parts[1])?;

            Ok(Op::Push(value))
        }

        "POP" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Pop)
        }

        "DUP" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Dup)
        }

        "SWAP" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Swap)
        }

        "ADD" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Add)
        }

        "SUB" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Sub)
        }

        "MUL" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Mul)
        }

        "DIV" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Div)
        }

        "MOD" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Mod)
        }

        "NEG" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Neg)
        }

        "LOAD" => {
            expect_count(line_no, &parts, 2)?;

            let slot = parse_u8(line_no, parts[1])?;

            Ok(Op::Load(slot))
        }

        "STORE" => {
            expect_count(line_no, &parts, 2)?;

            let slot = parse_u8(line_no, parts[1])?;

            Ok(Op::Store(slot))
        }

        "PRINT" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Print)
        }

        "HALT" => {
            expect_count(line_no, &parts, 1)?;
            Ok(Op::Halt)
        }

        _ => Err(MiniVmError::Parse(format!(
            "line {}: unknown instruction '{}'",
            line_no, mnemonic
        ))),
    }
}

fn expect_count(line: usize, parts: &[&str], expected: usize) -> Result<()> {
    if parts.len() != expected {
        return Err(MiniVmError::Parse(format!(
            "line {}: expected {} token(s), got {}",
            line,
            expected,
            parts.len()
        )));
    }

    Ok(())
}

fn parse_i64(line: usize, text: &str) -> Result<i64> {
    text.parse::<i64>()
        .map_err(|_| MiniVmError::Parse(format!("line {}: invalid integer '{}'", line, text)))
}

fn parse_u8(line: usize, text: &str) -> Result<u8> {
    text.parse::<u8>()
        .map_err(|_| MiniVmError::Parse(format!("line {}: invalid slot '{}'", line, text)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push_parse() {
        let op = parse_instruction("PUSH 42", 1).unwrap();

        assert_eq!(op, Op::Push(42));
    }

    #[test]
    fn case_insensitive() {
        let op = parse_instruction("aDd", 1).unwrap();

        assert_eq!(op, Op::Add);
    }

    #[test]
    fn load_parse() {
        let op = parse_instruction("LOAD 255", 1).unwrap();

        assert_eq!(op, Op::Load(255));
    }
}
