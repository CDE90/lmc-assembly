use std::{
    io::{self, Write},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Instruction {
    LDA(Operand),
    STA(Operand),
    ADD(Operand),
    SUB(Operand),
    INP,
    OUT,
    OTC,
    HLT,
    BRZ(Operand),
    BRP(Operand),
    BRA(Operand),
    DAT(Operand),
}

impl Instruction {
    pub fn from_string(opcode: &str, operand: Option<Operand>) -> Option<Self> {
        match opcode.to_uppercase().as_str() {
            "LDA" => Some(Instruction::LDA(operand.expect("LDA requires an operand"))),
            "STA" => Some(Instruction::STA(operand.expect("STA requires an operand"))),
            "ADD" => Some(Instruction::ADD(operand.expect("ADD requires an operand"))),
            "SUB" => Some(Instruction::SUB(operand.expect("SUB requires an operand"))),
            "INP" => Some(Instruction::INP),
            "OUT" => Some(Instruction::OUT),
            "OTC" => Some(Instruction::OTC),
            "HLT" => Some(Instruction::HLT),
            "BRZ" => Some(Instruction::BRZ(operand.expect("BRZ requires an operand"))),
            "BRP" => Some(Instruction::BRP(operand.expect("BRP requires an operand"))),
            "BRA" => Some(Instruction::BRA(operand.expect("BRA requires an operand"))),
            "DAT" => Some(Instruction::DAT(operand.unwrap_or(Operand::Value(0)))), // DAT can have an operand, but doesn't have to
            _ => None,
        }
    }
    fn get_base(&self) -> i16 {
        match self {
            Self::LDA(_) => 500,
            Self::STA(_) => 300,
            Self::ADD(_) => 100,
            Self::SUB(_) => 200,
            Self::INP => 901,
            Self::OUT => 902,
            Self::OTC => 922,
            Self::HLT => 0,
            Self::BRZ(_) => 700,
            Self::BRP(_) => 800,
            Self::BRA(_) => 600,
            Self::DAT(_) => 0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Operand {
    Value(i16),
    Label(String),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i16>() {
            Ok(val) => Ok(Operand::Value(val)),
            Err(_) => Ok(Operand::Label(s.to_string())),
        }
    }
}

impl Operand {
    fn get_value(&self, program: &Program) -> Result<i16, String> {
        match self {
            Operand::Value(val) => Ok(*val),
            Operand::Label(lbl) => {
                let mut pos = 0;
                for (label, _) in program {
                    if label == &Label::LBL(lbl.to_string()) {
                        return Ok(pos);
                    }
                    pos += 1;
                }
                Err(format!("Invalid label... {}", lbl))
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Label {
    LBL(String),
    None,
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Label::LBL(a), Label::LBL(b)) => a == b,
            (Label::None, Label::None) => true,
            _ => false,
        }
    }
}

pub type Program = Vec<(Label, Instruction)>;

pub fn parse(code: &str, debug_mode: bool) -> Result<Program, String> {
    if debug_mode {
        println!("Parsing code...");
    }

    let mut program: Program = vec![];

    for line in code.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if debug_mode {
            println!("{:?}", tokens);
        }

        if !tokens.is_empty() && tokens[0].starts_with("//") {
            continue;
        }

        match tokens.len() {
            0 => continue,
            1 => {
                let instruction = Instruction::from_string(tokens[0], None)
                    .ok_or_else(|| format!("Invalid opcode... {}", tokens[0]))?;

                program.push((Label::None, instruction));
            }
            2 => {
                let operand = tokens[1].parse::<Operand>()?;

                match Instruction::from_string(tokens[0], Some(operand)) {
                    Some(val) => program.push((Label::None, val)),
                    None => {
                        let instruction = Instruction::from_string(tokens[1], None)
                            .ok_or_else(|| format!("Invalid opcode... {}", tokens[1]))?;

                        program.push((Label::LBL(tokens[0].to_string()), instruction));
                    }
                }
            }
            3 => {
                let operand = tokens[2].parse::<Operand>()?;

                let instruction = Instruction::from_string(tokens[1], Some(operand))
                    .ok_or_else(|| format!("Invalid opcode... {}", tokens[1]))?;

                program.push((Label::LBL(tokens[0].to_string()), instruction));
            }
            _ => return Err(format!("Error while reading line: {}", line)),
        }
    }

    if debug_mode {
        println!();
    }

    Ok(program)
}

pub fn assemble(program: Program) -> Result<[i16; 100], String> {
    let mut ram = [0; 100];

    for (i, (_, instruction)) in program.iter().enumerate() {
        ram[i] = match instruction {
            Instruction::BRZ(operand) | Instruction::BRP(operand) | Instruction::BRA(operand) => {
                instruction.get_base() + operand.get_value(&program)?
            }
            Instruction::DAT(operand) => operand.get_value(&program)?,
            Instruction::LDA(operand)
            | Instruction::STA(operand)
            | Instruction::ADD(operand)
            | Instruction::SUB(operand) => instruction.get_base() + operand.get_value(&program)?,
            Instruction::INP | Instruction::OUT | Instruction::OTC | Instruction::HLT => {
                instruction.get_base()
            }
        }
    }

    Ok(ram)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct ExecutionState {
    pub pc: i16,
    pub cir: i16,
    pub mar: i16,
    pub mdr: i16,
    pub acc: i16,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub ram: [i16; 100],
}

impl ExecutionState {
    pub fn step<T: LMCIO>(&mut self, io_handler: &mut T) -> Result<(), String> {
        self.mar = self.pc;
        self.pc += 1;
        self.mdr = self.ram[self.mar as usize];
        self.cir = self.mdr;
        // do instruction
        match self.cir {
            0 => self.pc = -1,
            901 => {
                let res = io_handler.get_input();
                if !(-999..=999).contains(&res) {
                    return Err("Number out of range".to_string());
                }
                self.acc = res;
            }
            902 => io_handler.print_output(Output::Int(self.acc)),
            922 => io_handler.print_output(Output::Char(self.acc as u8 as char)),
            100..=199 => {
                self.mar = self.cir - 100;
                self.acc += self.ram[self.mar as usize];
                // handle overflow to -999
                if self.acc > 999 {
                    let diff = self.acc - 999;
                    self.acc = -999 + diff - 1;
                } else if self.acc < -999 {
                    let diff = -999 - self.acc;
                    self.acc = 999 - diff + 1;
                }
            }
            200..=299 => {
                self.mar = self.cir - 200;
                self.acc -= self.ram[self.mar as usize];
                // handle underflow to 999
                if self.acc < -999 {
                    let diff = -999 - self.acc;
                    self.acc = 999 - diff + 1;
                } else if self.acc > 999 {
                    let diff = self.acc - 999;
                    self.acc = -999 + diff - 1;
                }
            }
            300..=399 => {
                self.mar = self.cir - 300;
                self.ram[self.mar as usize] = self.acc;
            }
            500..=599 => {
                self.mar = self.cir - 500;
                self.acc = self.ram[self.mar as usize];
            }
            600..=699 => {
                self.mar = self.cir - 600;
                self.pc = self.mar;
            }
            700..=799 => {
                self.mar = self.cir - 700;
                if self.acc == 0 {
                    self.pc = self.mar;
                }
            }
            800..=899 => {
                self.mar = self.cir - 800;
                if self.acc > 0 {
                    self.pc = self.mar;
                }
            }
            _ => return Err(format!("Invalid instruction: {}", self.cir)),
        };

        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub enum Output {
    Char(char),
    Int(i16),
}

pub trait LMCIO {
    fn get_input(&mut self) -> i16;
    fn print_output(&mut self, val: Output);
}

pub struct DefaultIO;

impl LMCIO for DefaultIO {
    fn get_input(&mut self) -> i16 {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.trim().parse::<i16>().unwrap()
    }

    fn print_output(&mut self, val: Output) {
        match val {
            Output::Char(c) => print!("{}", c),
            Output::Int(i) => println!("{}", i),
        }
    }
}

pub fn run<T: LMCIO>(
    program: [i16; 100],
    io_handler: &mut T,
    debug_mode: bool,
) -> Result<(), String> {
    let mut state = ExecutionState {
        pc: 0,
        cir: 0,
        mar: 0,
        mdr: 0,
        acc: 0,
        ram: program,
    };

    loop {
        state.step(io_handler)?;

        if state.pc == -1 {
            break;
        }

        if debug_mode {
            println!("PC: {}", state.pc);
            println!("CIR: {}", state.cir);
            println!("MAR: {}", state.mar);
            println!("MDR: {}", state.mdr);
            println!("ACC: {}", state.acc);
            println!("RAM: {:?}", state.ram);
            println!();
        }

        if state.pc > 99 {
            break;
        }
    }

    Ok(())
}
