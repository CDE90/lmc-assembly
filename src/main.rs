use dotenvy::dotenv;
use std::{
    env, fs,
    io::{self, stdout, Write},
    str::FromStr,
};

#[derive(Debug)]
enum Instruction {
    LDA(Operand),
    STA(Operand),
    ADD(Operand),
    SUB(Operand),
    INP,
    OUT,
    HLT,
    BRZ(Operand),
    BRP(Operand),
    BRA(Operand),
    DAT(Operand),
}

#[derive(Debug)]
enum Operand {
    Value(u16),
    Label(String),
}

#[derive(Debug)]
enum Label {
    LBL(String),
    None,
}

type Program = Vec<(Label, Instruction)>;

impl Instruction {
    fn from_string(opcode: &str, operand: Option<Operand>) -> Option<Self> {
        match opcode {
            "LDA" => Some(Instruction::LDA(operand.expect("LDA requires an operand"))),
            "STA" => Some(Instruction::STA(operand.expect("STA requires an operand"))),
            "ADD" => Some(Instruction::ADD(operand.expect("ADD requires an operand"))),
            "SUB" => Some(Instruction::SUB(operand.expect("SUB requires an operand"))),
            "INP" => Some(Instruction::INP),
            "OUT" => Some(Instruction::OUT),
            "HLT" => Some(Instruction::HLT),
            "BRZ" => Some(Instruction::BRZ(operand.expect("BRZ requires an operand"))),
            "BRP" => Some(Instruction::BRP(operand.expect("BRP requires an operand"))),
            "BRA" => Some(Instruction::BRA(operand.expect("BRA requires an operand"))),
            "DAT" => Some(Instruction::DAT(operand.unwrap_or(Operand::Value(0)))), // DAT can have an operand, but doesn't have to
            _ => None,
        }
    }
    fn get_base(&self) -> u16 {
        match self {
            Self::LDA(_) => 500,
            Self::STA(_) => 300,
            Self::ADD(_) => 100,
            Self::SUB(_) => 200,
            Self::INP => 901,
            Self::OUT => 902,
            Self::HLT => 0,
            Self::BRZ(_) => 700,
            Self::BRP(_) => 800,
            Self::BRA(_) => 600,
            Self::DAT(_) => 0,
        }
    }
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(val) => Ok(Operand::Value(val)),
            Err(_) => Ok(Operand::Label(s.to_string())),
        }
    }
}

impl Operand {
    fn get_value(&self, program: &Program) -> u16 {
        match self {
            Operand::Value(val) => *val,
            Operand::Label(lbl) => program
                .iter()
                .position(|x| x.0 == Label::LBL(lbl.to_string()))
                .expect(&format!("Invalid label... {}", lbl))
                as u16,
        }
    }
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

fn main() {
    dotenv().expect("Failed to load .env file");

    let debug_mode = env::var("DEBUG_MODE").unwrap_or("0".to_string()) == "1";

    let program = fs::read_to_string("program.lmc").expect("Should've been able to read the file");

    let parsed = parse(&program);

    if debug_mode {
        println!("Program:\n{:?}\n", parsed);
    }

    let assembled = assemble(parsed);

    if debug_mode {
        println!("Assembled:\n{:?}\n", assembled);
    }

    run(assembled);
}

fn parse(code: &str) -> Program {
    let debug_mode = env::var("DEBUG_MODE").unwrap_or("0".to_string()) == "1";

    if debug_mode {
        println!("Parsing code...");
    }

    let mut program: Program = vec![];

    for line in code.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if debug_mode {
            println!("{:?}", tokens);
        }

        match tokens.len() {
            0 => continue,
            1 => {
                let instruction = Instruction::from_string(tokens[0], None)
                    .expect(&format!("Invalid opcode... {}", tokens[0]));

                program.push((Label::None, instruction));
            }
            2 => {
                let operand = tokens[1].parse::<Operand>().unwrap();

                match Instruction::from_string(tokens[0], Some(operand)) {
                    Some(val) => program.push((Label::None, val)),
                    None => {
                        let instruction = Instruction::from_string(tokens[1], None)
                            .expect(&format!("Invalid opcode... {}", tokens[1]));

                        program.push((Label::LBL(tokens[0].to_string()), instruction));
                    }
                }
            }
            3 => {
                let operand = tokens[2].parse::<Operand>().unwrap();

                let instruction = Instruction::from_string(tokens[1], Some(operand))
                    .expect(&format!("Invalid opcode... {}", tokens[1]));

                program.push((Label::LBL(tokens[0].to_string()), instruction));
            }
            _ => panic!("Error while reading line: {}", line),
        }
    }

    if debug_mode {
        println!();
    }

    program
}

fn assemble(program: Program) -> [u16; 100] {
    let mut ram = [0; 100];

    for (i, (_, instruction)) in program.iter().enumerate() {
        ram[i] = match instruction {
            Instruction::BRZ(operand) | Instruction::BRP(operand) | Instruction::BRA(operand) => {
                instruction.get_base() + operand.get_value(&program)
            }
            Instruction::DAT(operand) => operand.get_value(&program),
            Instruction::LDA(operand)
            | Instruction::STA(operand)
            | Instruction::ADD(operand)
            | Instruction::SUB(operand) => instruction.get_base() + operand.get_value(&program),
            Instruction::INP | Instruction::OUT | Instruction::HLT => instruction.get_base(),
        }
    }

    ram
}

#[derive(Debug)]
struct ExecutionState {
    pc: u16,
    cir: u16,
    mar: u16,
    mdr: u16,
    acc: u16,
    ram: [u16; 100],
}

fn run(program: [u16; 100]) {
    let debug_mode = env::var("DEBUG_MODE").unwrap_or("0".to_string()) == "1";

    let mut state = ExecutionState {
        pc: 0,
        cir: 0,
        mar: 0,
        mdr: 0,
        acc: 0,
        ram: program,
    };

    loop {
        state.mar = state.pc;
        state.pc += 1;
        state.mdr = state.ram[state.mar as usize];
        state.cir = state.mdr;
        // do instruction
        match state.cir {
            0 => break,
            901 => {
                let mut input = String::new();
                print!("> ");
                stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                state.acc = input.trim().parse().expect("Not a number");
            }
            902 => println!("{}", state.acc),
            100..=199 => {
                state.mar = state.cir - 100;
                state.acc += state.ram[state.mar as usize];
            }
            200..=299 => {
                state.mar = state.cir - 200;
                state.acc -= state.ram[state.mar as usize];
            }
            300..=399 => {
                state.mar = state.cir - 300;
                state.ram[state.mar as usize] = state.acc;
            }
            500..=599 => {
                state.mar = state.cir - 500;
                state.acc = state.ram[state.mar as usize];
            }
            600..=699 => {
                state.mar = state.cir - 600;
                state.pc = state.mar;
            }
            700..=799 => {
                state.mar = state.cir - 700;
                if state.acc == 0 {
                    state.pc = state.mar;
                }
            }
            800..=899 => {
                state.mar = state.cir - 800;
                if state.acc > 0 {
                    state.pc = state.mar;
                }
            }
            _ => panic!("Invalid instruction"),
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
}
