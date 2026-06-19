use std::fs;

use crate::bytecode::parse_file;
use crate::errors::{MiniVmError, Result};
use crate::isa::{GLOBAL_SLOTS, Op, STACK_LIMIT, decode};

pub struct VM {
    stack: Vec<i64>,
    globals: [i64; GLOBAL_SLOTS],
    trace: bool,
}

impl VM {
    pub fn new(trace: bool) -> Self {
        Self {
            stack: Vec::new(),
            globals: [0; GLOBAL_SLOTS],
            trace,
        }
    }
}

pub fn command(args: &[String]) -> Result<()> {
    match args.len() {
        3 => run_file(&args[2], false),

        4 => {
            if args[2] != "--trace" {
                return Err(MiniVmError::Parse(
                    "usage: minivm run [--trace] file.tbc".into(),
                ));
            }

            run_file(&args[3], true)
        }

        _ => Err(MiniVmError::Parse(
            "usage: minivm run [--trace] file.tbc".into(),
        )),
    }
}

pub fn run_file(filename: &str, trace: bool) -> Result<()> {
    let bytes = fs::read(filename)?;

    let code = parse_file(&bytes)?;

    let mut vm = VM::new(trace);

    vm.run(&code)
}

impl VM {
    fn push(&mut self, value: i64, ip: usize) -> Result<()> {
        if self.stack.len() >= STACK_LIMIT {
            return Err(MiniVmError::Trap(format!(
                "trap at ip=0x{:04X}: stack overflow",
                ip
            )));
        }

        self.stack.push(value);

        Ok(())
    }
}

impl VM {
    fn pop(&mut self, ip: usize, context: &str) -> Result<i64> {
        self.stack.pop().ok_or_else(|| {
            MiniVmError::Trap(format!(
                "trap at ip=0x{:04X}: stack underflow ({})",
                ip, context
            ))
        })
    }
}

impl VM {
    fn trace(&self, ip: usize, op: Op) {
        if !self.trace {
            return;
        }

        println!("ip=0x{:04X} {:?} stack={:?}", ip, op, self.stack);
    }
}

impl VM {
    pub fn run(&mut self, code: &[u8]) -> Result<()> {
        let mut ip = 0;

        loop {
            let current_ip = ip;

            let op = decode(code, &mut ip)?;

            self.trace(current_ip, op);

            match op {
                Op::Push(v) => {
                    self.push(v, current_ip)?;
                }

                Op::Pop => {
                    self.pop(current_ip, "POP on empty stack")?;
                }

                Op::Dup => {
                    let v = *self.stack.last().ok_or_else(|| {
                        MiniVmError::Trap(format!(
                            "trap at ip=0x{:04X}: stack underflow (DUP on empty stack)",
                            current_ip
                        ))
                    })?;

                    self.push(v, current_ip)?;
                }

                Op::Swap => {
                    if self.stack.len() < 2 {
                        return Err(MiniVmError::Trap(format!(
                            "trap at ip=0x{:04X}: stack underflow (SWAP requires 2 values)",
                            current_ip
                        )));
                    }

                    let n = self.stack.len();

                    self.stack.swap(n - 1, n - 2);
                }

                Op::Add => {
                    let b = self.pop(current_ip, "ADD")?;

                    let a = self.pop(current_ip, "ADD")?;

                    self.push(a + b, current_ip)?;
                }

                Op::Sub => {
                    let b = self.pop(current_ip, "SUB")?;

                    let a = self.pop(current_ip, "SUB")?;

                    self.push(a - b, current_ip)?;
                }

                Op::Mul => {
                    let b = self.pop(current_ip, "MUL")?;

                    let a = self.pop(current_ip, "MUL")?;

                    self.push(a * b, current_ip)?;
                }

                Op::Div => {
                    let b = self.pop(current_ip, "DIV")?;

                    if b == 0 {
                        return Err(MiniVmError::Trap(format!(
                            "trap at ip=0x{:04X}: division by zero",
                            current_ip
                        )));
                    }

                    let a = self.pop(current_ip, "DIV")?;

                    self.push(a / b, current_ip)?;
                }

                Op::Mod => {
                    let b = self.pop(current_ip, "MOD")?;

                    if b == 0 {
                        return Err(MiniVmError::Trap(format!(
                            "trap at ip=0x{:04X}: modulo by zero",
                            current_ip
                        )));
                    }

                    let a = self.pop(current_ip, "MOD")?;

                    self.push(a % b, current_ip)?;
                }

                Op::Neg => {
                    let a = self.pop(current_ip, "NEG")?;

                    self.push(-a, current_ip)?;
                }

                Op::Load(slot) => {
                    self.push(self.globals[slot as usize], current_ip)?;
                }

                Op::Store(slot) => {
                    let value = self.pop(current_ip, "STORE")?;

                    self.globals[slot as usize] = value;
                }

                Op::Print => {
                    let value = self.pop(current_ip, "PRINT")?;

                    println!("{}", value);
                }

                Op::Halt => {
                    return Ok(());
                }
            }
        }
    }
}
