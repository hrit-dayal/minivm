use std::fs;

#[test]
fn unknown_opcode_trap() {
    let mut file = Vec::new();

    // MVM\0
    file.extend_from_slice(b"MVM\0");

    // version
    file.push(1);

    // code length = 1
    file.extend_from_slice(&1u32.to_le_bytes());

    // invalid opcode
    file.push(0x99);

    fs::write("bad_opcode.tbc", file).unwrap();

    let result = minivm::vm::run_file("bad_opcode.tbc", false);

    match result {
        Err(e) => {
            let msg = e.to_string();

            assert!(msg.contains("unknown opcode"));

            assert!(msg.contains("0x0000"));
        }

        Ok(_) => panic!("expected unknown opcode trap"),
    }
}
