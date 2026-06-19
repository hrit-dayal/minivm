use crate::errors::*;
use crate::isa::{MAGIC, VERSION};

pub fn build_file(code: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(&MAGIC);
    out.push(VERSION);

    out.extend_from_slice(&(code.len() as u32).to_le_bytes());

    out.extend_from_slice(code);

    out
}

pub fn parse_file(bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() < 9 {
        return Err(MiniVmError::Validation("file too small".into()));
    }

    if bytes[0..4] != MAGIC {
        return Err(MiniVmError::Validation("invalid magic".into()));
    }

    if bytes[4] != VERSION {
        return Err(MiniVmError::Validation(format!(
            "unsupported version {}",
            bytes[4]
        )));
    }

    let len = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]) as usize;

    let actual = bytes.len() - 9;

    if len != actual {
        return Err(MiniVmError::Validation(format!(
            "length mismatch: header={}, actual={}",
            len, actual
        )));
    }

    Ok(bytes[9..].to_vec())
}
