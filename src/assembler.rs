/**
 * AUTHORSHIP: This assembler is almost completely generated with AI.
 * I have modified it such that it is now compatible with my interpreter.
 * Because the assembler is not part of the assignment, this should be okay.
 * In other words, I have generated and fixed a convenience tool to test
 * my intepreter with. I cannot make any guarantees it is correct, but it
 * seems good enough for now.
 *
 * Enjoy!
 */
use std::collections::HashMap;
use std::str::FromStr;

// ===========================
// Data Structures
// ===========================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Register {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$zero" | "$0" => Ok(Register::Zero),
            "$at" | "$1" => Ok(Register::At),
            "$v0" | "$2" => Ok(Register::V0),
            "$v1" | "$3" => Ok(Register::V1),
            "$a0" | "$4" => Ok(Register::A0),
            "$a1" | "$5" => Ok(Register::A1),
            "$a2" | "$6" => Ok(Register::A2),
            "$a3" | "$7" => Ok(Register::A3),
            "$t0" | "$8" => Ok(Register::T0),
            "$t1" | "$9" => Ok(Register::T1),
            "$t2" | "$10" => Ok(Register::T2),
            "$t3" | "$11" => Ok(Register::T3),
            "$t4" | "$12" => Ok(Register::T4),
            "$t5" | "$13" => Ok(Register::T5),
            "$t6" | "$14" => Ok(Register::T6),
            "$t7" | "$15" => Ok(Register::T7),
            "$s0" | "$16" => Ok(Register::S0),
            "$s1" | "$17" => Ok(Register::S1),
            "$s2" | "$18" => Ok(Register::S2),
            "$s3" | "$19" => Ok(Register::S3),
            "$s4" | "$20" => Ok(Register::S4),
            "$s5" | "$21" => Ok(Register::S5),
            "$s6" | "$22" => Ok(Register::S6),
            "$s7" | "$23" => Ok(Register::S7),
            "$t8" | "$24" => Ok(Register::T8),
            "$t9" | "$25" => Ok(Register::T9),
            "$k0" | "$26" => Ok(Register::K0),
            "$k1" | "$27" => Ok(Register::K1),
            "$gp" | "$28" => Ok(Register::Gp),
            "$sp" | "$29" => Ok(Register::Sp),
            "$fp" | "$30" => Ok(Register::Fp),
            "$ra" | "$31" => Ok(Register::Ra),
            _ => Err(format!("Invalid register: {}", s)),
        }
    }
}

impl Register {
    fn to_num(&self) -> u32 {
        match self {
            Register::Zero => 0,
            Register::At => 1,
            Register::V0 => 2,
            Register::V1 => 3,
            Register::A0 => 4,
            Register::A1 => 5,
            Register::A2 => 6,
            Register::A3 => 7,
            Register::T0 => 8,
            Register::T1 => 9,
            Register::T2 => 10,
            Register::T3 => 11,
            Register::T4 => 12,
            Register::T5 => 13,
            Register::T6 => 14,
            Register::T7 => 15,
            Register::S0 => 16,
            Register::S1 => 17,
            Register::S2 => 18,
            Register::S3 => 19,
            Register::S4 => 20,
            Register::S5 => 21,
            Register::S6 => 22,
            Register::S7 => 23,
            Register::T8 => 24,
            Register::T9 => 25,
            Register::K0 => 26,
            Register::K1 => 27,
            Register::Gp => 28,
            Register::Sp => 29,
            Register::Fp => 30,
            Register::Ra => 31,
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Register(Register),
    Immediate(i32),
    Label(String),
    LabelOffset(String, i32), // Label with offset: label(offset)
    #[allow(unused)]
    LabelOnly(String), // Just a label without offset
}

#[derive(Debug, Clone)]
enum Instruction {
    // R-Type instructions
    Add {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Addu {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Sub {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Subu {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    And {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Or {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Nor {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Slt {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Sltu {
        rd: Register,
        rs: Register,
        rt: Register,
    },
    Sll {
        rd: Register,
        rt: Register,
        shamt: u32,
    },
    Srl {
        rd: Register,
        rt: Register,
        shamt: u32,
    },
    Sra {
        rd: Register,
        rt: Register,
        shamt: u32,
    },
    Sllv {
        rd: Register,
        rt: Register,
        rs: Register,
    },
    Srlv {
        rd: Register,
        rt: Register,
        rs: Register,
    },
    Srav {
        rd: Register,
        rt: Register,
        rs: Register,
    },
    Jr {
        rs: Register,
    },
    Jalr {
        rd: Register,
        rs: Register,
    },
    Syscall,

    // I-Type instructions
    Addi {
        rt: Register,
        rs: Register,
        imm: i32,
    },
    Addiu {
        rt: Register,
        rs: Register,
        imm: i32,
    },
    Lui {
        rt: Register,
        imm: i32,
    },
    Lw {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Sw {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Lh {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Lhu {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Lb {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Lbu {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Sh {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Sb {
        rt: Register,
        offset: i32,
        base: Register,
    },
    Beq {
        rs: Register,
        rt: Register,
        label: String,
    },
    Bne {
        rs: Register,
        rt: Register,
        label: String,
    },
    Blez {
        rs: Register,
        label: String,
    },
    Bgtz {
        rs: Register,
        label: String,
    },
    Bltz {
        rs: Register,
        label: String,
    },
    Bgez {
        rs: Register,
        label: String,
    },

    // J-Type instructions
    J {
        target: String,
    },
    Jal {
        target: String,
    },

    // Pseudo-instructions
    Li {
        rt: Register,
        imm: i32,
    },
    La {
        rt: Register,
        label: String,
    },
    Move {
        rd: Register,
        rs: Register,
    },
    Blt {
        rs: Register,
        rt: Register,
        label: String,
    },
    Bgt {
        rs: Register,
        rt: Register,
        label: String,
    },
    Ble {
        rs: Register,
        rt: Register,
        label: String,
    },
    Bge {
        rs: Register,
        rt: Register,
        label: String,
    },
    Nop,

    // Special handling for lw/sw with labels
    #[allow(unused)]
    LwLabel {
        rt: Register,
        label: String,
        offset: i32,
    },
    #[allow(unused)]
    SwLabel {
        rt: Register,
        label: String,
        offset: i32,
    },
}

// ===========================
// Parser
// ===========================

struct Parser {
    text_start: u32,
    data_start: u32,
    current_address: u32,
    in_text_section: bool,
    in_data_section: bool,
    labels: HashMap<String, u32>,
    data_labels: HashMap<String, u32>,
    instructions: Vec<(u32, Instruction, String)>,
    data: Vec<(u32, Vec<u8>, String)>, // (address, bytes, original_line)
}

impl Parser {
    fn new() -> Self {
        Self {
            text_start: 0x00400000,
            data_start: 0x10010000,
            current_address: 0x00400000,
            in_text_section: false,
            in_data_section: false,
            labels: HashMap::new(),
            data_labels: HashMap::new(),
            instructions: Vec::new(),
            data: Vec::new(),
        }
    }

    fn parse_line(&mut self, line: &str) -> Result<(), String> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        // Remove inline comments
        let mut line = line.split('#').next().unwrap().trim();

        // Check for directives
        if line.starts_with('.') {
            return self.parse_directive(line);
        }

        // Check for label
        if let Some(spl) = line.split_once(":")
            && !spl.0.contains(" ")
        {
            let label = spl.0.trim().to_string();
            if self.in_text_section {
                self.labels.insert(label, self.current_address);
            } else if self.in_data_section {
                self.data_labels.insert(label, self.current_address);
            }
            line = spl.1.trim();
            if line.is_empty() {
                return Ok(());
            }
        }

        if self.in_text_section {
            self.parse_instruction(line)
        } else if self.in_data_section {
            self.parse_data(line)
        } else {
            Err("Instruction/data outside of .text or .data section".to_string())
        }
    }

    fn parse_directive(&mut self, line: &str) -> Result<(), String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            ".text" => {
                self.in_text_section = true;
                self.in_data_section = false;
                self.current_address = self.text_start;
                Ok(())
            }
            ".data" => {
                self.in_data_section = true;
                self.in_text_section = false;
                self.current_address = self.data_start;
                Ok(())
            }
            ".word" => {
                if !self.in_data_section {
                    return Err(".word outside .data section".to_string());
                }
                self.parse_word_data(&parts[1..], line)
            }
            ".half" => {
                if !self.in_data_section {
                    return Err(".half outside .data section".to_string());
                }
                self.parse_half_data(&parts[1..], line)
            }
            ".byte" => {
                if !self.in_data_section {
                    return Err(".byte outside .data section".to_string());
                }
                self.parse_byte_data(&parts[1..], line)
            }
            ".asciiz" => {
                if !self.in_data_section {
                    return Err(".asciiz outside .data section".to_string());
                }
                self.parse_asciiz_data(&parts[1..], line)
            }
            ".space" => {
                if !self.in_data_section {
                    return Err(".space outside .data section".to_string());
                }
                self.parse_space_data(&parts[1..], line)
            }
            ".ascii" => {
                if !self.in_data_section {
                    return Err(".ascii outside .data section".to_string());
                }
                self.parse_ascii_data(&parts[1..], line)
            }
            ".align" => {
                if !self.in_data_section {
                    return Err(".align outside .data section".to_string());
                }
                self.parse_align_data(&parts[1..], line)
            }
            _ => Err(format!("Unknown directive: {}", parts[0])),
        }
    }

    fn parse_word_data(&mut self, values: &[&str], original: &str) -> Result<(), String> {
        for val in values {
            let val = val.trim_end_matches(',');
            let num = parse_immediate(val)?;
            let bytes = num.to_le_bytes(); // MIPS is little-endian
            self.data
                .push((self.current_address, bytes.to_vec(), original.to_string()));
            self.current_address += 4;
        }
        Ok(())
    }

    fn parse_half_data(&mut self, values: &[&str], original: &str) -> Result<(), String> {
        for val in values {
            let val = val.trim_end_matches(',');
            let num = parse_immediate(val)? as i16;
            let bytes = num.to_le_bytes();
            self.data
                .push((self.current_address, bytes.to_vec(), original.to_string()));
            self.current_address += 2;
        }
        Ok(())
    }

    fn parse_byte_data(&mut self, values: &[&str], original: &str) -> Result<(), String> {
        for val in values {
            let val = val.trim_end_matches(',');
            let num = parse_immediate(val)? as i8;
            let bytes = vec![num as u8];
            self.data
                .push((self.current_address, bytes, original.to_string()));
            self.current_address += 1;
        }
        Ok(())
    }

    fn parse_asciiz_data(&mut self, parts: &[&str], original: &str) -> Result<(), String> {
        if parts.is_empty() {
            return Err("Expected string literal for .asciiz".to_string());
        }

        let mut s = String::new();
        for part in parts {
            s.push_str(part);
            s.push(' ');
        }
        s = s.trim().to_string();

        if !s.starts_with('"') || !s.ends_with('"') {
            return Err("String literal must be in quotes".to_string());
        }

        let s = &s[1..s.len() - 1];
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0); // Null terminator

        let bl = bytes.len() as u32;
        self.data
            .push((self.current_address, bytes, original.to_string()));
        self.current_address += bl;
        Ok(())
    }

    fn parse_ascii_data(&mut self, parts: &[&str], original: &str) -> Result<(), String> {
        if parts.is_empty() {
            return Err("Expected string literal for .ascii".to_string());
        }

        let mut s = String::new();
        for part in parts {
            s.push_str(part);
            s.push(' ');
        }
        s = s.trim().to_string();

        if !s.starts_with('"') || !s.ends_with('"') {
            return Err("String literal must be in quotes".to_string());
        }

        let s = &s[1..s.len() - 1];
        let bytes = s.as_bytes().to_vec();

        let bl = bytes.len() as u32;
        self.data
            .push((self.current_address, bytes, original.to_string()));
        self.current_address += bl;
        Ok(())
    }

    fn parse_space_data(&mut self, parts: &[&str], original: &str) -> Result<(), String> {
        if parts.is_empty() {
            return Err("Expected size for .space".to_string());
        }

        let size = parse_immediate(parts[0])? as u32;
        let bytes = vec![0u8; size as usize];
        self.data
            .push((self.current_address, bytes, original.to_string()));
        self.current_address += size;
        Ok(())
    }

    fn parse_align_data(&mut self, parts: &[&str], original: &str) -> Result<(), String> {
        if parts.is_empty() {
            return Err("Expected alignment for .align".to_string());
        }

        let align = parse_immediate(parts[0])? as u32;
        let mask = (1 << align) - 1;
        if (self.current_address & mask) != 0 {
            let padding = (1 << align) - (self.current_address & mask);
            let bytes = vec![0u8; padding as usize];
            self.data
                .push((self.current_address, bytes, format!("; {}", original)));
            self.current_address += padding;
        }
        Ok(())
    }

    fn parse_data(&mut self, line: &str) -> Result<(), String> {
        // Check if the second part is a directive
        if line.starts_with('.') {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            // Parse the directive with the label
            match parts[0] {
                ".word" => self.parse_word_data(&parts[1..], line),
                ".half" => self.parse_half_data(&parts[1..], line),
                ".byte" => self.parse_byte_data(&parts[1..], line),
                ".asciiz" => self.parse_asciiz_data(&parts[1..], line),
                ".ascii" => self.parse_ascii_data(&parts[1..], line),
                ".space" => self.parse_space_data(&parts[1..], line),
                _ => Err(format!("Unknown directive: {}", parts[0])),
            }
        } else {
            Err("Expected directive after label in data section".to_string())
        }
    }

    fn parse_instruction(&mut self, line: &str) -> Result<(), String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let mnemonic = parts[0];
        let operands_str = if parts.len() > 1 {
            parts[1..].join(" ")
        } else {
            String::new()
        };

        let instruction = match mnemonic.to_lowercase().as_str() {
            "add" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Add { rd, rs, rt }),
            "addu" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Addu { rd, rs, rt }),
            "sub" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Sub { rd, rs, rt }),
            "subu" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Subu { rd, rs, rt }),
            "and" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::And { rd, rs, rt }),
            "or" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Or { rd, rs, rt }),
            "nor" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Nor { rd, rs, rt }),
            "slt" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Slt { rd, rs, rt }),
            "sltu" => self.parse_r3(&operands_str, |rd, rs, rt| Instruction::Sltu { rd, rs, rt }),
            "sll" => self.parse_shift(&operands_str, |rd, rt, shamt| Instruction::Sll {
                rd,
                rt,
                shamt,
            }),
            "srl" => self.parse_shift(&operands_str, |rd, rt, shamt| Instruction::Srl {
                rd,
                rt,
                shamt,
            }),
            "sra" => self.parse_shift(&operands_str, |rd, rt, shamt| Instruction::Sra {
                rd,
                rt,
                shamt,
            }),
            "sllv" => self.parse_r3(&operands_str, |rd, rt, rs| Instruction::Sllv { rd, rt, rs }),
            "srlv" => self.parse_r3(&operands_str, |rd, rt, rs| Instruction::Srlv { rd, rt, rs }),
            "srav" => self.parse_r3(&operands_str, |rd, rt, rs| Instruction::Srav { rd, rt, rs }),
            "jr" => self.parse_r1(&operands_str, |rs| Instruction::Jr { rs }),
            "jalr" => self.parse_r2(&operands_str, |rd, rs| Instruction::Jalr { rd, rs }),
            "syscall" => Ok(Instruction::Syscall),

            "addi" => self.parse_i(&operands_str, |rt, rs, imm| Instruction::Addi {
                rt,
                rs,
                imm,
            }),
            "addiu" => self.parse_i(&operands_str, |rt, rs, imm| Instruction::Addiu {
                rt,
                rs,
                imm,
            }),
            "lui" => self.parse_i_rt_imm(&operands_str, |rt, imm| Instruction::Lui { rt, imm }),
            "lw" => self.parse_mem(&operands_str, |rt, offset, base| {
                if offset == 0 && base == Register::Zero {
                    // This is lw with just a label - we'll handle it specially
                    Err("Use lw with explicit offset and register".to_string())
                } else {
                    Ok(Instruction::Lw { rt, offset, base })
                }
            }),
            "sw" => self.parse_mem(&operands_str, |rt, offset, base| {
                if offset == 0 && base == Register::Zero {
                    // This is sw with just a label
                    Err("Use sw with explicit offset and register".to_string())
                } else {
                    Ok(Instruction::Sw { rt, offset, base })
                }
            }),
            "lh" => self.parse_mem(&operands_str, |rt, offset, base| {
                Ok(Instruction::Lh { rt, offset, base })
            }),
            "lhu" => self.parse_mem(&operands_str, |rt, offset, base| {
                Ok(Instruction::Lhu { rt, offset, base })
            }),
            "lb" => self.parse_mem(&operands_str, |rt, offset, base| {
                Ok(Instruction::Lb { rt, offset, base })
            }),
            "lbu" => self.parse_mem(&operands_str, |rt, offset, base| {
                Ok(Instruction::Lbu { rt, offset, base })
            }),
            "sh" => self.parse_mem(&operands_str, |rt, offset, base| {
                Ok(Instruction::Sh { rt, offset, base })
            }),
            "sb" => self.parse_mem(&operands_str, |rt, offset, base| {
                Ok(Instruction::Sb { rt, offset, base })
            }),
            "beq" => self.parse_branch(&operands_str, |rs, rt, label| Instruction::Beq {
                rs,
                rt,
                label,
            }),
            "bne" => self.parse_branch(&operands_str, |rs, rt, label| Instruction::Bne {
                rs,
                rt,
                label,
            }),
            "blez" => {
                self.parse_branch1(&operands_str, |rs, label| Instruction::Blez { rs, label })
            }
            "bgtz" => {
                self.parse_branch1(&operands_str, |rs, label| Instruction::Bgtz { rs, label })
            }
            "bltz" => {
                self.parse_branch1(&operands_str, |rs, label| Instruction::Bltz { rs, label })
            }
            "bgez" => {
                self.parse_branch1(&operands_str, |rs, label| Instruction::Bgez { rs, label })
            }

            "j" => self.parse_j(&operands_str, |target| Instruction::J { target }),
            "jal" => self.parse_j(&operands_str, |target| Instruction::Jal { target }),

            // Pseudo-instructions
            "li" => self.parse_li(&operands_str),
            "la" => self.parse_la(&operands_str),
            "move" => self.parse_r2(&operands_str, |rd, rs| Instruction::Move { rd, rs }),
            "blt" => self.parse_branch(&operands_str, |rs, rt, label| Instruction::Blt {
                rs,
                rt,
                label,
            }),
            "bgt" => self.parse_branch(&operands_str, |rs, rt, label| Instruction::Bgt {
                rs,
                rt,
                label,
            }),
            "ble" => self.parse_branch(&operands_str, |rs, rt, label| Instruction::Ble {
                rs,
                rt,
                label,
            }),
            "bge" => self.parse_branch(&operands_str, |rs, rt, label| Instruction::Bge {
                rs,
                rt,
                label,
            }),
            "nop" => Ok(Instruction::Nop),

            _ => Err(format!("Unknown instruction: {}", mnemonic)),
        }?;

        self.instructions
            .push((self.current_address, instruction, line.to_string()));
        self.current_address += 4;
        Ok(())
    }

    fn parse_operands(&self, s: &str) -> Result<Vec<Operand>, String> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Vec::new());
        }

        let mut operands = Vec::new();
        let mut current = String::new();
        let mut in_paren = false;
        let mut in_string = false;

        for c in s.chars() {
            if c == '"' {
                in_string = !in_string;
                current.push(c);
            } else if in_string {
                current.push(c);
            } else if c == '(' && !in_paren {
                in_paren = true;
                if !current.trim().is_empty() {
                    operands.push(self.parse_single_operand(&current.trim())?);
                    current.clear();
                }
                current.push(c);
            } else if c == ')' && in_paren {
                in_paren = false;
                current.push(c);
                operands.push(self.parse_single_operand(&current)?);
                current.clear();
            } else if c == ',' && !in_paren {
                if !current.trim().is_empty() {
                    operands.push(self.parse_single_operand(&current.trim())?);
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }

        if !current.trim().is_empty() {
            operands.push(self.parse_single_operand(&current.trim())?);
        }

        Ok(operands)
    }

    fn parse_single_operand(&self, s: &str) -> Result<Operand, String> {
        let s = s.trim();

        // Check for register
        if s.starts_with('$') {
            return Ok(Operand::Register(s.parse()?));
        }

        // Check for label with offset: label(offset)
        if s.contains('(') && s.ends_with(')') {
            let open_paren = s.find('(').unwrap();
            let label = s[..open_paren].trim().to_string();
            let offset_str = s[open_paren + 1..s.len() - 1].trim();
            let offset = parse_immediate(offset_str)?;
            return Ok(Operand::LabelOffset(label, offset));
        }

        // Check for immediate
        if let Ok(imm) = parse_immediate(s) {
            return Ok(Operand::Immediate(imm));
        }

        // Otherwise it's a label
        Ok(Operand::Label(s.to_string()))
    }

    fn parse_r3<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, Register, Register) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 3 {
            return Err(format!("Expected 3 operands, got {}", ops.len()));
        }

        let rd = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let rs = match ops[1] {
            Operand::Register(r) => r,
            _ => return Err("Second operand must be a register".to_string()),
        };

        let rt = match ops[2] {
            Operand::Register(r) => r,
            _ => return Err("Third operand must be a register".to_string()),
        };

        Ok(f(rd, rs, rt))
    }

    fn parse_r2<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, Register) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 2 {
            return Err(format!("Expected 2 operands, got {}", ops.len()));
        }

        let rd = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let rs = match ops[1] {
            Operand::Register(r) => r,
            _ => return Err("Second operand must be a register".to_string()),
        };

        Ok(f(rd, rs))
    }

    fn parse_r1<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 1 {
            return Err(format!("Expected 1 operand, got {}", ops.len()));
        }

        let rs = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("Operand must be a register".to_string()),
        };

        Ok(f(rs))
    }

    fn parse_shift<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, Register, u32) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 3 {
            return Err(format!("Expected 3 operands, got {}", ops.len()));
        }

        let rd = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let rt = match ops[1] {
            Operand::Register(r) => r,
            _ => return Err("Second operand must be a register".to_string()),
        };

        let shamt = match ops[2] {
            Operand::Immediate(imm) => {
                if imm < 0 || imm > 31 {
                    return Err("Shift amount must be between 0 and 31".to_string());
                }
                imm as u32
            }
            _ => return Err("Third operand must be an immediate".to_string()),
        };

        Ok(f(rd, rt, shamt))
    }

    fn parse_i<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, Register, i32) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 3 {
            return Err(format!("Expected 3 operands, got {}", ops.len()));
        }

        let rt = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let rs = match ops[1] {
            Operand::Register(r) => r,
            _ => return Err("Second operand must be a register".to_string()),
        };

        let imm = match ops[2] {
            Operand::Immediate(imm) => imm,
            _ => return Err("Third operand must be an immediate".to_string()),
        };

        Ok(f(rt, rs, imm))
    }

    fn parse_i_rt_imm<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, i32) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 2 {
            return Err(format!("Expected 2 operands, got {}", ops.len()));
        }

        let rt = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let imm = match ops[1] {
            Operand::Immediate(imm) => imm,
            _ => return Err("Second operand must be an immediate".to_string()),
        };

        Ok(f(rt, imm))
    }

    fn parse_mem<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, i32, Register) -> Result<Instruction, String>,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 2 {
            return Err(format!("Expected 2 operands, got {}", ops.len()));
        }

        let rt = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        match &ops[1] {
            Operand::LabelOffset(_label, offset) => {
                // lw $t0, label(offset)
                // We need to convert this to: la $at, label + offset; lw $t0, 0($at)
                // But we'll handle this in the encoder
                f(rt, *offset, Register::Zero)
            }
            Operand::Immediate(offset) => {
                // lw $t0, offset - assume base is $zero
                f(rt, *offset, Register::Zero)
            }
            Operand::Label(_label) => {
                // lw $t0, label - equivalent to lw $t0, 0(label)
                f(rt, 0, Register::Zero)
            }
            _ => Err("Second operand must be in offset(base) or label format".to_string()),
        }
    }

    fn parse_branch<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, Register, String) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 3 {
            return Err(format!("Expected 3 operands, got {}", ops.len()));
        }

        let rs = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let rt = match ops[1] {
            Operand::Register(r) => r,
            _ => return Err("Second operand must be a register".to_string()),
        };

        let label = match ops[2] {
            Operand::Label(ref l) => l.clone(),
            _ => return Err("Third operand must be a label".to_string()),
        };

        Ok(f(rs, rt, label))
    }

    fn parse_branch1<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(Register, String) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 2 {
            return Err(format!("Expected 2 operands, got {}", ops.len()));
        }

        let rs = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let label = match ops[1] {
            Operand::Label(ref l) => l.clone(),
            _ => return Err("Second operand must be a label".to_string()),
        };

        Ok(f(rs, label))
    }

    fn parse_j<F>(&self, s: &str, f: F) -> Result<Instruction, String>
    where
        F: FnOnce(String) -> Instruction,
    {
        let ops = self.parse_operands(s)?;
        if ops.len() != 1 {
            return Err(format!("Expected 1 operand, got {}", ops.len()));
        }

        let target = match ops[0] {
            Operand::Label(ref l) => l.clone(),
            _ => return Err("Operand must be a label".to_string()),
        };

        Ok(f(target))
    }

    fn parse_li(&self, s: &str) -> Result<Instruction, String> {
        let ops = self.parse_operands(s)?;
        if ops.len() != 2 {
            return Err(format!("Expected 2 operands, got {}", ops.len()));
        }

        let rt = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let imm = match ops[1] {
            Operand::Immediate(imm) => imm,
            _ => return Err("Second operand must be an immediate".to_string()),
        };

        Ok(Instruction::Li { rt, imm })
    }

    fn parse_la(&self, s: &str) -> Result<Instruction, String> {
        let ops = self.parse_operands(s)?;
        if ops.len() != 2 {
            return Err(format!("Expected 2 operands, got {}", ops.len()));
        }

        let rt = match ops[0] {
            Operand::Register(r) => r,
            _ => return Err("First operand must be a register".to_string()),
        };

        let label = match ops[1] {
            Operand::Label(ref l) => l.clone(),
            _ => return Err("Second operand must be a label".to_string()),
        };

        Ok(Instruction::La { rt, label })
    }
}

// ===========================
// Encoder
// ===========================

struct Encoder {
    labels: HashMap<String, u32>,
    data_labels: HashMap<String, u32>,
}

impl Encoder {
    fn new(labels: HashMap<String, u32>, data_labels: HashMap<String, u32>) -> Self {
        Self {
            labels,
            data_labels,
        }
    }

    fn encode(&self, instr: &Instruction, address: u32) -> Result<Vec<u32>, String> {
        match instr {
            // R-Type instructions
            Instruction::Add { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x20,
            )]),
            Instruction::Addu { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x21,
            )]),
            Instruction::Sub { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x22,
            )]),
            Instruction::Subu { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x23,
            )]),
            Instruction::And { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x24,
            )]),
            Instruction::Or { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x25,
            )]),
            Instruction::Nor { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x27,
            )]),
            Instruction::Slt { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x2a,
            )]),
            Instruction::Sltu { rd, rs, rt } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x2b,
            )]),
            Instruction::Sll { rd, rt, shamt } => Ok(vec![self.encode_r(
                0x00,
                0,
                rt.to_num(),
                rd.to_num(),
                *shamt,
                0x00,
            )]),
            Instruction::Srl { rd, rt, shamt } => Ok(vec![self.encode_r(
                0x00,
                0,
                rt.to_num(),
                rd.to_num(),
                *shamt,
                0x02,
            )]),
            Instruction::Sra { rd, rt, shamt } => Ok(vec![self.encode_r(
                0x00,
                0,
                rt.to_num(),
                rd.to_num(),
                *shamt,
                0x03,
            )]),
            Instruction::Sllv { rd, rt, rs } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x04,
            )]),
            Instruction::Srlv { rd, rt, rs } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x06,
            )]),
            Instruction::Srav { rd, rt, rs } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                rt.to_num(),
                rd.to_num(),
                0,
                0x07,
            )]),
            Instruction::Jr { rs } => Ok(vec![self.encode_r(0x00, rs.to_num(), 0, 0, 0, 0x08)]),
            Instruction::Jalr { rd, rs } => Ok(vec![self.encode_r(
                0x00,
                rs.to_num(),
                0,
                rd.to_num(),
                0,
                0x09,
            )]),
            Instruction::Syscall => Ok(vec![0x0000000c]),

            // I-Type instructions
            Instruction::Addi { rt, rs, imm } => {
                Ok(vec![self.encode_i(0x08, rs.to_num(), rt.to_num(), *imm)])
            }
            Instruction::Addiu { rt, rs, imm } => {
                Ok(vec![self.encode_i(0x09, rs.to_num(), rt.to_num(), *imm)])
            }
            Instruction::Lui { rt, imm } => Ok(vec![self.encode_i(0x0f, 0, rt.to_num(), *imm)]),
            Instruction::Lw { rt, offset, base } => Ok(vec![self.encode_i(
                0x23,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Sw { rt, offset, base } => Ok(vec![self.encode_i(
                0x2b,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Lh { rt, offset, base } => Ok(vec![self.encode_i(
                0x21,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Lhu { rt, offset, base } => Ok(vec![self.encode_i(
                0x25,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Lb { rt, offset, base } => Ok(vec![self.encode_i(
                0x20,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Lbu { rt, offset, base } => Ok(vec![self.encode_i(
                0x24,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Sh { rt, offset, base } => Ok(vec![self.encode_i(
                0x29,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Sb { rt, offset, base } => Ok(vec![self.encode_i(
                0x28,
                base.to_num(),
                rt.to_num(),
                *offset,
            )]),
            Instruction::Beq { rs, rt, label } => {
                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 4)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                Ok(vec![self.encode_i(0x04, rs.to_num(), rt.to_num(), offset)])
            }
            Instruction::Bne { rs, rt, label } => {
                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 4)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                Ok(vec![self.encode_i(0x05, rs.to_num(), rt.to_num(), offset)])
            }
            Instruction::Blez { rs, label } => {
                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 4)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                Ok(vec![self.encode_i(0x06, rs.to_num(), 0, offset)])
            }
            Instruction::Bgtz { rs, label } => {
                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 4)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                Ok(vec![self.encode_i(0x07, rs.to_num(), 0, offset)])
            }
            Instruction::Bltz { rs, label } => {
                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 4)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                Ok(vec![self.encode_i(0x01, rs.to_num(), 0, offset)])
            }
            Instruction::Bgez { rs, label } => {
                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 4)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                Ok(vec![self.encode_i(0x01, rs.to_num(), 1, offset)])
            }

            // J-Type instructions
            Instruction::J { target } => {
                let target_addr = *self
                    .labels
                    .get(target)
                    .or_else(|| self.data_labels.get(target))
                    .ok_or_else(|| format!("Undefined label: {}", target))?;
                let target = (target_addr & 0x0FFFFFFC) >> 2;
                Ok(vec![self.encode_j(0x02, target)])
            }
            Instruction::Jal { target } => {
                let target_addr = *self
                    .labels
                    .get(target)
                    .or_else(|| self.data_labels.get(target))
                    .ok_or_else(|| format!("Undefined label: {}", target))?;
                let target = (target_addr & 0x0FFFFFFC) >> 2;
                Ok(vec![self.encode_j(0x03, target)])
            }

            // Pseudo-instructions
            Instruction::Li { rt, imm } => {
                if *imm >= -32768 && *imm <= 32767 {
                    // Use addiu with $zero
                    Ok(vec![self.encode_i(0x09, 0, rt.to_num(), *imm)])
                } else {
                    // Use lui + ori
                    let upper = ((*imm as u32) >> 16) & 0xFFFF;
                    let lower = (*imm as u32) & 0xFFFF;
                    let mut result = Vec::new();

                    if upper != 0 {
                        result.push(self.encode_i(0x0f, 0, rt.to_num(), upper as i32));
                        if lower != 0 {
                            result.push(self.encode_i(
                                0x0d,
                                rt.to_num(),
                                rt.to_num(),
                                lower as i32,
                            ));
                        }
                    } else {
                        result.push(self.encode_i(0x09, 0, rt.to_num(), lower as i32));
                    }
                    Ok(result)
                }
            }
            Instruction::La { rt, label } => {
                let addr = *self
                    .data_labels
                    .get(label)
                    .or_else(|| self.labels.get(label))
                    .ok_or_else(|| format!("Undefined label: {}", label))?;

                let upper = (addr >> 16) & 0xFFFF;
                let lower = addr & 0xFFFF;
                let mut result = Vec::new();

                if upper != 0 {
                    result.push(self.encode_i(0x0f, 0, rt.to_num(), upper as i32));
                    if lower != 0 {
                        result.push(self.encode_i(0x0d, rt.to_num(), rt.to_num(), lower as i32));
                    }
                } else {
                    result.push(self.encode_i(0x09, 0, rt.to_num(), lower as i32));
                }
                Ok(result)
            }
            Instruction::Move { rd, rs } => {
                // move $rd, $rs -> addu $rd, $rs, $zero
                Ok(vec![self.encode_r(
                    0x00,
                    rs.to_num(),
                    0,
                    rd.to_num(),
                    0,
                    0x21,
                )])
            }
            Instruction::Blt { rs, rt, label } => {
                // blt $rs, $rt, label -> slt $at, $rs, $rt + bne $at, $zero, label
                let mut result = Vec::new();
                result.push(self.encode_r(0x00, rs.to_num(), rt.to_num(), 1, 0, 0x2a)); // slt $at

                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 8)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                result.push(self.encode_i(0x05, 1, 0, offset)); // bne $at, $zero
                Ok(result)
            }
            Instruction::Bgt { rs, rt, label } => {
                // bgt $rs, $rt, label -> slt $at, $rt, $rs + bne $at, $zero, label
                let mut result = Vec::new();
                result.push(self.encode_r(0x00, rt.to_num(), rs.to_num(), 1, 0, 0x2a)); // slt $at

                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 8)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                result.push(self.encode_i(0x05, 1, 0, offset)); // bne $at, $zero
                Ok(result)
            }
            Instruction::Ble { rs, rt, label } => {
                // ble $rs, $rt, label -> slt $at, $rt, $rs + beq $at, $zero, label
                let mut result = Vec::new();
                result.push(self.encode_r(0x00, rt.to_num(), rs.to_num(), 1, 0, 0x2a)); // slt $at

                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 8)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                result.push(self.encode_i(0x04, 1, 0, offset)); // beq $at, $zero
                Ok(result)
            }
            Instruction::Bge { rs, rt, label } => {
                // bge $rs, $rt, label -> slt $at, $rs, $rt + beq $at, $zero, label
                let mut result = Vec::new();
                result.push(self.encode_r(0x00, rs.to_num(), rt.to_num(), 1, 0, 0x2a)); // slt $at

                let target_addr = *self
                    .labels
                    .get(label)
                    .ok_or_else(|| format!("Undefined label: {}", label))?;
                let offset = ((target_addr as i64 - (address as i64 + 8)) / 4) as i32;
                if offset < -32768 || offset > 32767 {
                    return Err(format!("Branch offset too large for label: {}", label));
                }
                result.push(self.encode_i(0x04, 1, 0, offset)); // beq $at, $zero
                Ok(result)
            }
            Instruction::Nop => {
                // nop -> sll $zero, $zero, 0
                Ok(vec![self.encode_r(0x00, 0, 0, 0, 0, 0x00)])
            }

            _ => Err("Unsupported instruction variant".to_string()),
        }
    }

    fn encode_r(&self, opcode: u32, rs: u32, rt: u32, rd: u32, shamt: u32, funct: u32) -> u32 {
        (opcode << 26) | (rs << 21) | (rt << 16) | (rd << 11) | (shamt << 6) | funct
    }

    fn encode_i(&self, opcode: u32, rs: u32, rt: u32, imm: i32) -> u32 {
        (opcode << 26) | (rs << 21) | (rt << 16) | (imm as u32 & 0xFFFF)
    }

    fn encode_j(&self, opcode: u32, target: u32) -> u32 {
        (opcode << 26) | (target & 0x03FFFFFF)
    }
}

// ===========================
// Main Assembler
// ===========================

pub struct MIPSAssembler {
    parser: Parser,
}

impl MIPSAssembler {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    pub fn assemble(&mut self, input: &str) -> Result<String, String> {
        // First pass: parse everything
        for line in input.lines() {
            self.parser.parse_line(line)?;
        }

        // Second pass: encode instructions
        let encoder = Encoder::new(self.parser.labels.clone(), self.parser.data_labels.clone());

        let mut output = String::new();
        let mut current_address = self.parser.text_start;

        // Output text section
        for (addr, instr, comment) in &self.parser.instructions {
            let encoded = encoder.encode(instr, *addr)?;

            for word in encoded {
                output.push_str(&format!(
                    "{:08x}: {:08x} ; {}\n",
                    current_address, word, comment
                ));
                current_address += 4;
            }
        }

        // Output data section
        // Group data into 4-byte words
        let mut data_words = Vec::new();
        let mut current_addr = self.parser.data_start;

        for (addr, bytes, comment) in &self.parser.data {
            // Align to current address if needed
            if current_addr < *addr {
                let padding = (*addr - current_addr) as usize;
                for _ in 0..padding {
                    data_words.push((current_addr, 0u8, String::new()));
                    current_addr += 1;
                }
            }

            for byte in bytes {
                data_words.push((current_addr, *byte, comment.clone()));
                current_addr += 1;
            }
        }

        // Group bytes into 4-byte words
        let mut word_output = Vec::new();
        let mut current_word_addr = 0;
        let mut current_word = 0u32;
        let mut current_comment = String::new();
        let mut byte_count = 0;

        for (addr, byte, comment) in &data_words {
            if byte_count == 0 {
                current_word_addr = *addr;
                current_word = 0;
                current_comment = comment.clone();
            }

            current_word |= (*byte as u32) << (byte_count * 8);
            byte_count += 1;

            if byte_count == 4 || addr + 1 == current_addr {
                word_output.push((current_word_addr, current_word, current_comment.clone()));
                byte_count = 0;
            }
        }

        // Output data words
        for (addr, word, comment) in word_output {
            let mut full_comment = String::new();
            full_comment.push_str(&comment);
            output.push_str(&format!(
                "{:08x}: {:08x} ; {}\n",
                addr,
                word.to_be(),
                full_comment
            ));
        }

        Ok(output)
    }
}

// ===========================
// Helper Functions
// ===========================

fn parse_immediate(s: &str) -> Result<i32, String> {
    let s = s.trim();

    if s.starts_with("0x") || s.starts_with("0X") {
        i32::from_str_radix(&s[2..], 16).map_err(|e| format!("Invalid hex number: {}", e))
    } else if s.starts_with("0b") || s.starts_with("0B") {
        i32::from_str_radix(&s[2..], 2).map_err(|e| format!("Invalid binary number: {}", e))
    } else if s.starts_with("0") && s.len() > 1 {
        i32::from_str_radix(&s[1..], 8).map_err(|e| format!("Invalid octal number: {}", e))
    } else {
        s.parse::<i32>()
            .map_err(|e| format!("Invalid decimal number: {}", e))
    }
}
