fn main() {
    let mut machine = Interpreter::new();
    // machine.load(vec![
    //     0x20020010,
    //     0x0000000c
    // ]);
    machine.load(vec![
        0x240400c0,
        0x00042400,
        0x24840000,
        0x20020004,
        0x0000000c,
        0x20020005,
        0x0000000c,
        0x00026020,
        0x218c0001,
        0x200a0001,
        0x000a2020,
        0x000a1020,
        0x0c100027,
        0x00025820,
        0x240400c0,
        0x00042400,
        0x248400d4,
        0x20020004,
        0x0000000c,
        0x000a2020,
        0x20020001,
        0x0000000c,
        0x240400c0,
        0x00042400,
        0x248400d8,
        0x20020004,
        0x0000000c,
        0x000b2020,
        0x20020001,
        0x0000000c,
        0x240400c0,
        0x00042400,
        0x248400dc,
        0x20020004,
        0x0000000c,
        0x214a0001,
        0x154cffe5,
        0x2002000a,
        0x0000000c,
        0x10800015,
        0x1000ffd7,
        0x00000022,
        0xafbf0000,
        0x00000022,
        0x0c100027,
        0x00000020,
        0x8fbf0000,
        0x00000020,
        0x00000022,
        0xafa20000,
        0x00000022,
        0xafbf0000,
        0x00000022,
        0x0c100027,
        0x00000020,
        0x8fbf0000,
        0x00000020,
        0x8fb70000,
        0x00000020,
        0x00571020,
        0x03e00008,
        0x20020000,
        0x03e00008,
        0x20020001,
        0x03e00008,
    ]);
    let exitcode = loop {
        if let Err(e) = machine.step() {
            break e;
        }
    };
    println!("Process exited with code {}", exitcode);
    std::process::exit(exitcode)
}

struct Interpreter {
    pc: u32,
    reg_lo: u32,
    reg_hi: u32,
    registers: [u32; 32],
    text: Vec<u32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            pc: 0b0,
            reg_lo: 0,
            reg_hi: 0,
            registers: [0b0; 32],
            text: Vec::new(),
        }
    }

    fn load(&mut self, program: Vec<u32>) {
        self.pc = 0b0;
        self.text = program;
    }

    fn step(&mut self) -> Result<(), i32> {
        if self.pc & 0b0011 != 0 {
            eprintln!("Program Counter must be word-aligned");
            return Err(-1)
        }

        let real_pc = (self.pc >> 2) as usize;

        if real_pc >= self.text.len() {
            eprintln!("Out-of-bounds memory access.");
            return Err(-1)
        }

        let instr = self.text[real_pc];
        self.pc += 0b100;
        self.execute(instr)
    }

    fn syscall(&self) -> Result<(), i32> {
        match self.registers[2] {
            1 => {
                print!("{}", self.registers[4] as i32);
                Ok(())
            }
            2 => {
                print!("{}", f32::from_be_bytes(self.registers[4].to_be_bytes()));
                Ok(())
            }
            4 => {
                // get the chars between self.registers[4] and self.registers[5]
                print!("{}", "Hello World!\n");
                Ok(())
            }
            //   5 =>
            //     {
            //   static char str [256];

            //   read_input (str, 256);
            //   R[REG_RES] = atol (str);
            //   break;
            //     }

            //   6 =>
            //     {
            //   static char str [256];

            //   read_input (str, 256);
            //   FPR_S (REG_FRES) = (float) atof (str);
            //   break;
            //     }

            //   7 =>
            //     {
            //   static char str [256];

            //   read_input (str, 256);
            //   FPR [REG_FRES] = atof (str);
            //   break;
            //     }

            //   case READ_STRING_SYSCALL:
            //     {
            //   read_input ( (char *) mem_reference (R[REG_A0]), R[REG_A1]);
            //   data_modified = true;
            //   break;
            //     }

            //   case SBRK_SYSCALL:
            //     {
            //   mem_addr x = data_top;
            //   expand_data (R[REG_A0]);
            //   R[REG_RES] = x;
            //   data_modified = true;
            //   break;
            //     }

            //   case PRINT_CHARACTER_SYSCALL:
            //     write_output (console_out, "%c", R[REG_A0]);
            //     break;

            //   case READ_CHARACTER_SYSCALL:
            //     {
            //   static char str [2];

            //   read_input (str, 2);
            //   if (*str == '\0') *str = '\n';      /* makes xspim = spim */
            //   R[REG_RES] = (long) str[0];
            //   break;
            //     }
            16 => Err(0),
            17 => Err(self.registers[4] as i32),
            _ => {
                println!("Unrecognized Syscall");
                Ok(())
            }
        }
    }

    fn segfault(&self) -> Result<(), i32> {
        eprintln!("Segmentation Fault: core not dumped why would i dump that");
        Err(-1)
    }

    fn execute(&mut self, instr: u32) -> Result<(), i32> {
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

    fn execute_rtype(&mut self, funct: u32, rs: usize, rt: usize, rd: usize, shift: u32) -> Result<(), i32> {
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
            _ => return self.segfault()
        }
        Ok(())
    }

    fn execute_itype(&mut self, opcode: u32, rs: usize, rd: usize, immediate: u16) -> Result<(), i32> {
        let src = self.registers[rs];
        let dst = &mut self.registers[rd];
        match opcode {
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
            // 0x23 // lw
            //    => 
            // 0x2b // sw
            //    => 
            _ => return self.segfault(),
        }
        Ok(())
    }
}

fn jumpaddr(pc: u32, instr: u32) -> u32 {
    return (pc & 0b11110000_00000000_00000000_00000000)
        | ((instr & 0b00000011_11111111_11111111_11111111) << 2);
}

// https://www.yumpu.com/en/document/read/33742169/mips32-instruction-set-quick-reference-mips-technologies-inc