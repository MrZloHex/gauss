use crate::instr::*;

pub fn parse_instr(source_code: Vec<String>) -> (Vec<Instruction>, ProgrammInfo) {
   let mut instructions: Vec<Instruction> = Vec::new();
   let proginfo: ProgrammInfo = parse_directives(&mut instructions, source_code);
   (instructions, proginfo)
}

fn parse_directives(instructions: &mut Vec<Instruction>, source_code: Vec<String>) -> ProgrammInfo {
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

