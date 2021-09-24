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
    let mut instruction = Instruction::default();
    
    let instr: Vec<&str> = (*line).split_whitespace().collect();

    if instr.len() > 3 {
        if instr[0] == "store" {
            instruction.op = Operation::Store;
            instruction.loc = get_location(instr[1].clone());
        } else {
            panic!("Unimplemented instruction")
        }
    } else {
        panic!("Unreacheable")
    }
    instruction
}

fn get_location(loc: &str) -> Location {
    match loc {
        "mem" => Location::Memory,
        _ => panic!("Unreachable")
    }
}

