use crate::instr::*;

pub fn parse_code(source_code: Vec<String>) -> (Vec<Instruction>, ProgrammInfo) {
   let proginfo: ProgrammInfo = parse_directives(source_code.clone());
   let instructions = parse_instructions(source_code);
   (instructions, proginfo)
}

fn parse_directives(source_code: Vec<String>) -> ProgrammInfo {
    let mut proginfo = ProgrammInfo { heap_vol: 0 };
    for line in source_code {
        if line.starts_with('!') {
            let direct = lex_directive(&line);
            match direct {
                Directive::Heap(val) => proginfo.heap_vol = val,
            }
        }
    }
    proginfo
}

fn lex_directive(line: &String) -> Directive {
    let tmp: Vec<&str> = (*line).split('!').collect();
    let dir: Vec<&str> = tmp[1].split_whitespace().collect();

    if dir.len() > 1 {
        if dir[0] == "heap" {
            Directive::Heap(dir[1].parse::<u128>().unwrap())
        } else {
            panic!("Unreachabledirective lexer")
        }
    } else {
        panic!("unreachable");
    }
}

fn parse_instructions(source_code: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in source_code {
        if !line.starts_with('!') && !line.is_empty() {
            let instr = lex_instruction(&line);
            instructions.push(instr);
        }
    }

    instructions
}

fn lex_instruction(line: &String) -> Instruction {
    let instr: Vec<&str> = (*line).split_whitespace().collect();

    if instr.len() > 3 {
        if instr[0] == "store" {
            let loc = get_location(instr[1].clone());
            let (value, address) = get_numbers(instr[2].clone(), instr[3].clone());
            let instruction = Instruction::new(Operation::Store(Store {loc,value, address}));
            instruction
        } else {
            panic!("Unimplemented instruction")
        }
    } else {
        panic!("Unreacheable")
    }
}

fn get_location(loc: &str) -> Location {
    match loc {
        "mem" => Location::Memory,
        _ => panic!("Unreachable")
    }
}

fn get_numbers(arg1: &str, arg2: &str) -> (u128, u128) {
    let mut value: u128 = 0;
    let mut address: u128 = 0;
    if arg1.starts_with('$') {
        let address = arg1.split_at(1).1.parse::<u128>().unwrap();
        let value = arg2.parse::<u128>().unwrap();
    } else {
        let value = arg1.parse::<u128>().unwrap();
        let address = arg2.split_at(1).1.parse::<u128>().unwrap();
    }
    (value, address)
}

