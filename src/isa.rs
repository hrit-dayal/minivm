use crate::errors::{MiniVmError, Result};

pub const MAGIC: [u8; 4] = *b"MVM\0";
pub const VERSION: u8 = 1;

pub const STACK_LIMIT: usize = 1024;
pub const GLOBAL_SLOTS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Push(i64),

    Pop,
    Dup,
    Swap,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    Load(u8),
    Store(u8),

    Print,

    Halt,
}

impl Op {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Op::Push(_) => "PUSH",

            Op::Pop => "POP",
            Op::Dup => "DUP",
            Op::Swap => "SWAP",

            Op::Add => "ADD",
            Op::Sub => "SUB",
            Op::Mul => "MUL",
            Op::Div => "DIV",
            Op::Mod => "MOD",
            Op::Neg => "NEG",

            Op::Load(_) => "LOAD",
            Op::Store(_) => "STORE",

            Op::Print => "PRINT",

            Op::Halt => "HALT",
        }
    }

    pub fn encode(&self, out: &mut Vec<u8>) {
        match *self {
            Op::Push(v) => {
                out.push(0x01);
                out.extend_from_slice(&v.to_le_bytes());
            }

            Op::Pop => out.push(0x02),
            Op::Dup => out.push(0x03),
            Op::Swap => out.push(0x04),

            Op::Add => out.push(0x10),
            Op::Sub => out.push(0x11),
            Op::Mul => out.push(0x12),
            Op::Div => out.push(0x13),
            Op::Mod => out.push(0x14),
            Op::Neg => out.push(0x15),

            Op::Load(slot) => {
                out.push(0x40);
                out.push(slot);
            }

            Op::Store(slot) => {
                out.push(0x41);
                out.push(slot);
            }

            Op::Print => out.push(0x60),

            Op::Halt => out.push(0xFF),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Op::Push(_) => 9,
            Op::Load(_) | Op::Store(_) => 2,
            _ => 1,
        }
    }
}

pub fn decode(code: &[u8], ip: &mut usize) -> Result<Op> {
    let start = *ip;

    if start >= code.len() {
        return Err(MiniVmError::Trap(format!(
            "trap at ip=0x{:04X}: ip past end without HALT",
            start
        )));
    }

    let opcode = code[*ip];
    *ip += 1;

    match opcode {
        0x01 => {
            if *ip + 8 > code.len() {
                return Err(MiniVmError::Trap(format!(
                    "trap at ip=0x{:04X}: truncated instruction",
                    start
                )));
            }

            let mut buf = [0u8; 8];

            buf.copy_from_slice(&code[*ip..*ip + 8]);

            *ip += 8;

            Ok(Op::Push(i64::from_le_bytes(buf)))
        }

        0x02 => Ok(Op::Pop),

        0x03 => Ok(Op::Dup),

        0x04 => Ok(Op::Swap),

        0x10 => Ok(Op::Add),

        0x11 => Ok(Op::Sub),

        0x12 => Ok(Op::Mul),

        0x13 => Ok(Op::Div),

        0x14 => Ok(Op::Mod),

        0x15 => Ok(Op::Neg),

        0x40 => {
            if *ip >= code.len() {
                return Err(MiniVmError::Trap(format!(
                    "trap at ip=0x{:04X}: truncated instruction",
                    start
                )));
            }

            let slot = code[*ip];
            *ip += 1;

            Ok(Op::Load(slot))
        }

        0x41 => {
            if *ip >= code.len() {
                return Err(MiniVmError::Trap(format!(
                    "trap at ip=0x{:04X}: truncated instruction",
                    start
                )));
            }

            let slot = code[*ip];
            *ip += 1;

            Ok(Op::Store(slot))
        }

        0x60 => Ok(Op::Print),

        0xFF => Ok(Op::Halt),

        _ => Err(MiniVmError::Trap(format!(
            "trap at ip=0x{:04X}: unknown opcode 0x{:02X}",
            start, opcode
        ))),
    }
}

pub fn to_asm(op: &Op) -> String {
    match *op {
        Op::Push(v) => format!("PUSH {}", v),

        Op::Load(s) => format!("LOAD {}", s),

        Op::Store(s) => format!("STORE {}", s),

        Op::Pop => "POP".into(),
        Op::Dup => "DUP".into(),
        Op::Swap => "SWAP".into(),

        Op::Add => "ADD".into(),
        Op::Sub => "SUB".into(),
        Op::Mul => "MUL".into(),
        Op::Div => "DIV".into(),
        Op::Mod => "MOD".into(),
        Op::Neg => "NEG".into(),

        Op::Print => "PRINT".into(),

        Op::Halt => "HALT".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(op: Op) {
        let mut bytes = Vec::new();

        op.encode(&mut bytes);

        let mut ip = 0;

        let decoded = decode(&bytes, &mut ip).unwrap();

        assert_eq!(op, decoded);
    }

    #[test]
    fn push_roundtrip() {
        roundtrip(Op::Push(12345));
    }

    #[test]
    fn load_roundtrip() {
        roundtrip(Op::Load(200));
    }

    #[test]
    fn store_roundtrip() {
        roundtrip(Op::Store(17));
    }

    #[test]
    fn simple_roundtrip() {
        roundtrip(Op::Add);
        roundtrip(Op::Mul);
        roundtrip(Op::Print);
        roundtrip(Op::Halt);
    }
}
