use std::process::Command;

#[test]
fn arith_program_runs() {
    let status = Command::new("cargo")
        .args(["run", "--", "asm", "programs/arith.tasm", "-o", "arith.tbc"])
        .status()
        .unwrap();

    assert!(status.success());
}

#[test]
fn celcilus_program_runs() {
    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "asm",
            "programs/celcius.tasm",
            "-o",
            "celcius.tbc",
        ])
        .status()
        .unwrap();

    assert!(status.success());
}

#[test]
fn digits_program_runs() {
    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "asm",
            "programs/digits.tasm",
            "-o",
            "digits.tbc",
        ])
        .status()
        .unwrap();

    assert!(status.success());
}

#[test]
fn horner_program_runs() {
    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "asm",
            "programs/horner.tasm",
            "-o",
            "horner.tbc",
        ])
        .status()
        .unwrap();

    assert!(status.success());
}

#[test]
fn stackplay_program_runs() {
    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "asm",
            "programs/stackplay.tasm",
            "-o",
            "stackplay.tbc",
        ])
        .status()
        .unwrap();

    assert!(status.success());
}
