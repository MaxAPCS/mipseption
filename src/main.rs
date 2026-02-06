mod assembler;
mod interpreter;

use crate::{
    assembler::MIPSAssembler as Assembler,
    interpreter::{Interpreter, Program},
};
use std::{env, fs, io::Write};

fn main() -> Result<(), String> {
    let programfile = env::args().nth(1).ok_or("Program file required")?;
    let mut program = fs::read_to_string(&programfile).or(Err("Program file unreadable"))?;

    if programfile.ends_with(".asm") {
        program = Assembler::new().assemble(program.as_str())?;
    }

    let mut machine = Interpreter::new(Program::parse(program.as_str())?);
    let exitcode = loop {
        if let Err(e) = machine.step() {
            break e;
        }
        std::io::stdout().flush().ok();
    };
    println!("\nProcess exited with code {}", exitcode);
    Ok(())
}
