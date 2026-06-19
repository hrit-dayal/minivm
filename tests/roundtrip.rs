use std::fs;

use minivm::assembler;
use minivm::disassembler;

#[test]
fn roundtrip() {
    let source = r#"PUSH 7
PUSH 3
ADD
PRINT
HALT
"#;

    fs::write("a.tasm", source).unwrap();

    assembler::assemble("a.tasm", "a.tbc").unwrap();

    let original = fs::read("a.tbc").unwrap();

    let bytes = fs::read("a.tbc").unwrap();

    let code = minivm::bytecode::parse_file(&bytes).unwrap();

    let text = disassembler::disassemble_bytes(&code).unwrap();

    fs::write("b.tasm", text).unwrap();

    assembler::assemble("b.tasm", "b.tbc").unwrap();

    let rebuilt = fs::read("b.tbc").unwrap();

    assert_eq!(original, rebuilt);
}
