use std::{env, fs};

fn main() -> Result<(), &'static str> {
    let file = fs::read_to_string(env::args().nth(1).ok_or("Program file required")?)
        .or(Err("Program file unreadable"))?;
    let mut machine = Interpreter::new(Program::parse(file.as_str())?);
    let exitcode = loop {
        if let Err(e) = machine.step() {
            break e;
        }
    };
    println!("\nProcess exited with code {}", exitcode);
    Ok(())
}

struct Program {
    text: Vec<u32>,
    data: Vec<u32>,
    stack: Vec<u32>,
    entrypoint: u32,
}

impl Program {
    fn parse(program: &str) -> Result<Self, &'static str> {
        let mut text = Vec::new();
        let mut data = Vec::new();
        let mut entrypoint = None;

        for line in program.lines() {
            if line.starts_with(";") || line.is_empty() {
                continue; // skip comments, empty lines
            }

            let (address, contents) = line.split_once(": ").ok_or("Failed to parse line!")?;
            let index =
                u32::from_str_radix(address, 16).or(Err("Failed to parse line address!"))?;
            let (content, _comment) = contents.split_once(" ; ").unwrap_or((contents, ""));

            if content.starts_with("<") && content.ends_with(">") {
                if content == "<main>" {
                    entrypoint.get_or_insert(index);
                }
                continue; // skip labels
            }

            let content = u32::from_str_radix(content.replace(" ", "").as_str(), 16)
                .or(Err("Failed to parse line content!"))?;

            if index < 0x0040_0000 {
                return Err("Out-of-bounds memory assignment!");
            } else if index <= 0x0FFF_FFFF {
                let real_pc = ((index - 0x0040_0000) >> 2) as usize;
                if real_pc != text.len() {
                    return Err("Discontinuous .text block!");
                }
                text.push(content);
            } else if index <= 0x7FFF_FFFF {
                let real_in = ((index - 0x1001_0000) >> 2) as usize;
                if real_in != data.len() {
                    return Err("Discontinuous .data block!");
                }
                data.push(content);
            }
        }
        Ok(Self {
            text,
            data,
            stack: Vec::new(),
            entrypoint: entrypoint.unwrap_or(0x0040_0000),
        })
    }

    fn index(&mut self, index: u32) -> Result<&u32, i32> {
        if index & 0b0011 != 0 {
            eprintln!("Index must be word-aligned");
            return Err(-1);
        }

        if index < 0x0040_0000 {
            eprintln!("Out-of-bounds memory access.");
            return Err(-1);
        } else if index <= 0x0FFF_FFFF {
            let real_pc = ((index - 0x0040_0000) >> 2) as usize;
            if real_pc < self.text.len() {
                return Ok(&self.text[real_pc]);
            }
        } else if index <= 0x7FFF_FFFF {
            let real_in = ((index - 0x1001_0000) >> 2) as usize;
            if real_in < self.data.len() {
                return Ok(&self.data[real_in]);
            }
            let real_in = ((0x7FFF_FFFF - index) >> 2) as usize;
            if real_in == self.stack.len() {
                self.stack.push(0); // allocate more stack!
            }
            if real_in < self.stack.len() {
                return Ok(&self.stack[real_in]);
            }
        }

        eprintln!("Out-of-bounds memory access.");
        Err(-1)
    }

    fn access(&mut self, index: u32) -> Result<&mut u32, i32> {
        if index & 0b0011 != 0 {
            eprintln!("Index must be word-aligned");
            return Err(-1);
        }

        if index < 0x0040_0000 {
            eprintln!("Out-of-bounds memory access.");
            return Err(-1);
        } else if index <= 0x0FFF_FFFF {
            eprintln!("Tried to modify .text!");
            return Err(-1);
        } else if index <= 0x7FFF_FFFF {
            let real_in = ((index - 0x1001_0000) >> 2) as usize;
            if real_in < self.data.len() {
                return Ok(&mut self.data[real_in]);
            }
            let real_in = ((0x7FFF_FFFF - index) >> 2) as usize;
            if real_in == self.stack.len() {
                self.stack.push(0); // allocate more stack!
            }
            if real_in < self.stack.len() {
                return Ok(&mut self.stack[real_in]);
            }
        }

        eprintln!("Out-of-bounds memory access.");
        Err(-1)
    }
}

struct Interpreter {
    pc: u32,
    reg_lo: u32,
    reg_hi: u32,
    registers: [u32; 32],
    program: Program,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            pc: program.entrypoint,
            reg_lo: 0,
            reg_hi: 0,
            registers: [0b0; 32],
            program: program,
        }
    }

    fn step(&mut self) -> Result<(), i32> {
        let instr = *self.program.index(self.pc)?;
        self.pc += 0b100;
        self.execute(instr)
    }

    fn syscall(&mut self) -> Result<(), i32> {
        match self.registers[2] {
            1 => {
                // Print Integer
                print!("{}", self.registers[4] as i32);
                Ok(())
            }
            4 => {
                // Print String
                if self.registers[5] < self.registers[4] {
                    return Err(-1);
                }
                let mut buf =
                    String::with_capacity((self.registers[5] - self.registers[4]) as usize);
                'outer: for i in (self.registers[4]..=self.registers[5]).step_by(4) {
                    for n in self.program.index(i)?.to_be_bytes() {
                        if n == 0 {
                            break 'outer; // null-termination
                        }
                        buf.push(n as char);
                    }
                }
                print!("{}", buf);
                Ok(())
            }
            5 => {
                // Read integer
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).map_err(|_| -1)?;
                let num = buf.trim().parse::<i32>().map_err(|_| -1)?;
                self.registers[2] = u32::from_ne_bytes(num.to_ne_bytes());
                Ok(())
            }
            8 => {
                // Read string
                if self.registers[5] < self.registers[4] {
                    return Err(-1);
                }
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).map_err(|_| -1)?;
                let buf = buf
                    .trim_end_matches("\n")
                    .chars()
                    .map(|c| c as u8)
                    .collect::<Vec<_>>();
                let buf = buf.as_chunks::<4>();
                let mut chunk_iter = buf.0.into_iter();
                for i in (self.registers[4]..=self.registers[5]).step_by(4) {
                    let n = self.program.access(i)?;
                    if let Some(fourchars) = chunk_iter.next() {
                        *n = u32::from_ne_bytes(*fourchars);
                    } else {
                        let mut padchars = [0; 4];
                        padchars[..buf.1.len()].copy_from_slice(buf.1);
                        *n = u32::from_ne_bytes(padchars);
                        break;
                    }
                }
                Ok(())
            }
            9 => Ok(()),  // Memory Allocation
            10 => Err(0), // Exit
            _ => {
                println!("Unrecognized Syscall");
                Ok(())
            }
        }
    }

    fn execute(&mut self, instr: u32) -> Result<(), i32> {
        self.registers[0] = 0; // $zero is always 0 (easier than doing checks)
        let opcode = instr >> 26;
        match opcode {
            0x00 =>
            self.execute_rtype(
                instr & 0b111111,
                (instr >> 21 & 0b11111) as usize,
                (instr >> 16 & 0b11111) as usize,
                (instr >> 11 & 0b11111) as usize,
                instr >> 6 & 0b11111
            ),
            0x02 // j
            => {
                self.pc = jumpaddr(self.pc, instr);
                Ok(())
            }
            0x03 // jal
            => {
                self.registers[31] = self.pc + 0b100;
                self.pc = jumpaddr(self.pc, instr);
                Ok(())
            }
            _ =>
            self.execute_itype(
                opcode,
                (instr >> 21 & 0b11111) as usize,
                (instr >> 16 & 0b11111) as usize,
                u16::try_from(instr & 0b11111111_11111111).unwrap()
            )
        }
    }

    fn execute_rtype(
        &mut self,
        funct: u32,
        rs: usize,
        rt: usize,
        rd: usize,
        shift: u32,
    ) -> Result<(), i32> {
        match funct {
            0x00 // sll
                => self.registers[rd] = self.registers[rs] << shift,
            0x02 // srl
                => self.registers[rd] = self.registers[rs] >> shift,
            0x03 // sra
                => self.registers[rd] = (self.registers[rs] as i32 >> shift) as u32,
            0x04 // sllv
                => self.registers[rd] = self.registers[rs] << (self.registers[rt] & 0b11111),
            0x06 // srlv
                => self.registers[rd] = self.registers[rs] >> (self.registers[rt] & 0b11111),
            0x07 // srav
                => self.registers[rd] = (self.registers[rs] as i32 >> (self.registers[rt] & 0b11111)) as u32,
            0x08 // jr
                => self.pc = self.registers[rs],
            0x09 // jalr
            => {
                self.registers[rd] = self.pc + 0b100;
                self.pc = self.registers[rs];
            }
            0x0a // movz
                => if self.registers[rt] == 0 {self.registers[rd] = self.registers[rs]},
            0x0b // movn
                => if self.registers[rt] != 0 {self.registers[rd] = self.registers[rs]},
            0x0c // syscall
                => return self.syscall(),
            0x0d // break
                => return Err(-1),
            0x10 // mfhi
                => self.registers[rd] = self.reg_hi,
            0x11 // mthi
                => self.reg_hi = self.registers[rs],
            0x12 // mflo
                => self.registers[rd] = self.reg_lo,
            0x13 // mtlo
                => self.reg_lo = self.registers[rs],
            0x18 // mult
            => {
                let res = (self.registers[rs] as i64 * self.registers[rt] as i64) as u64;
                self.reg_hi = u32::try_from(res >> 32).unwrap();
                self.reg_lo = res as u32;
            }
            0x19 // multu
            => {
                let res = (self.registers[rs] as u64 * self.registers[rt] as u64) as u64;
                self.reg_hi = u32::try_from(res >> 32).unwrap();
                self.reg_lo = res as u32;
            }
            0x1A // div
            => {
                self.reg_hi = (self.registers[rs] as i32 % self.registers[rt] as i32) as u32;
                self.reg_lo = (self.registers[rs] as i32 / self.registers[rt] as i32) as u32;
            }
            0x1B // divu
            => {
                self.reg_hi = self.registers[rs] % self.registers[rt];
                self.reg_lo = self.registers[rs] / self.registers[rt];
            }
            0x20 // add
                => self.registers[rd] = (self.registers[rs] as i32 + self.registers[rt] as i32) as u32,
            0x21 // addu
                => self.registers[rd] = self.registers[rs] + self.registers[rt],
            0x22 // sub
                => self.registers[rd] = (self.registers[rs] as i32 - self.registers[rt] as i32) as u32,
            0x23 // subu
                => self.registers[rd] = self.registers[rs] - self.registers[rt],
            0x24 // and
                => self.registers[rd] = self.registers[rs] & self.registers[rt],
            0x25 // or
                => self.registers[rd] = self.registers[rs] | self.registers[rt],
            0x26 // xor
                => self.registers[rd] = self.registers[rs] ^ self.registers[rt],
            0x27 // nor
                => self.registers[rd] = !(self.registers[rs] | self.registers[rt]),
            0x2a // slt
                => self.registers[rd] = ((self.registers[rs] as i32) < (self.registers[rt] as i32)) as u32,
            0x2b // sltu
                => self.registers[rd] = (self.registers[rs] < self.registers[rt]) as u32,
            _ => {eprintln!("Invalid Instruction"); return Err(-1)}
        }
        Ok(())
    }

    fn execute_itype(
        &mut self,
        opcode: u32,
        rs: usize,
        rd: usize,
        immediate: u16,
    ) -> Result<(), i32> {
        let src = self.registers[rs];
        let dst = &mut self.registers[rd];
        match opcode {
            0x01 if rd == 0 // bltz
                => if (src as i32) <  0 {self.pc += (immediate as u32) << 2},
            0x01 if rd == 1 // bgez
                => if (src as i32) >= 0 {self.pc += (immediate as u32) << 2},
            0x04 // beq
                => if src == *dst {self.pc += (immediate as u32) << 2},
            0x05 // bne
                => if src != *dst {self.pc += (immediate as u32) << 2},
            0x06 // blez
                => if (src as i32) <= 0 {self.pc += (immediate as u32) << 2},
            0x07 // bgtz
                => if (src as i32) >  0 {self.pc += (immediate as u32) << 2},
            0x08 // addi
                => *dst = (src as i32 + immediate as i32) as u32,
            0x09 // addiu
                => *dst = src + immediate as u32,
            0x0a // slti
                => *dst = ((src as i32) < (immediate as i32)) as u32,
            0x0b // sltiu
                => *dst = (src < immediate as u32) as u32,
            0x0c // andi
                => *dst = src & immediate as u32,
            0x0d // ori
                => *dst = src | immediate as u32,
            0x0e // xori
                => *dst = src ^ immediate as u32,
            0x0f // lui
                => *dst = ((immediate as u32) << 16) & 0xffff0000,
            0x23 // lw
                => *dst = *self.program.index(src + immediate as u32)?,
            0x2b // sw
               => *self.program.access(src + immediate as u32)? = *dst,
            _ => {eprintln!("Invalid Instruction"); return Err(-1)}
        }
        Ok(())
    }
}

#[inline]
fn jumpaddr(pc: u32, instr: u32) -> u32 {
    return (pc & 0b11110000_00000000_00000000_00000000)
        | ((instr & 0b00000011_11111111_11111111_11111111) << 2);
}
