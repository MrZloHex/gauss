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
                _ => ()
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
    let mut instr: Vec<&str> = (*line).split_whitespace().collect();

    if instr.len() > 3 {
        if instr[0] == "store" {
            instr.remove(0);
            let mut loc: Location = Location::Heap;
            let mut address: u64 = 0;
            let mut value: u64 = 0;
            for arg in instr {
                if arg.starts_with('?') {
                    loc = get_location(&arg[1..]);
                } else if arg.starts_with('$') {
                    address = (&arg[1..]).parse::<u64>().unwrap();
                } else if arg.starts_with('#') {
                    value = (&arg[1..]).parse::<u64>().unwrap();
                } else {
                    panic!("unreachable");
                }
            }
            let instruction = Instruction::new(Operation::Store(Store {loc,value, address}));
            instruction
        } else if instr[0] == "set" {
            instr.remove(0);
            let mut size = Size::Byte;
            let mut value: u64 = 0;
            let mut name: String = "_".to_string();
            for arg in instr {
                if arg.starts_with('_') {
                    name = (&arg[1..]).to_string();
                } else if arg.starts_with('#') {
                    value = (&arg[1..]).parse::<u64>().unwrap();
                } else if arg.starts_with('%') {
                    size = get_size(&arg[1..]);
                } else {
                    panic!("unreachable");
                }
            }
            let instruction = Instruction::new(Operation::Set(Set{name, size, value}));
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
        "heap" => Location::Heap,
        _ => panic!("Unreachable")
    }
}

fn get_size(size: &str) -> Size {
    match size {
        "byte" => Size::Byte,
        "word" => Size::Word,
        _ => panic!("unreachable")
    }
}

