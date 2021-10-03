#![allow(non_snake_case)]


use crate::instr::*;


pub fn lex_code(source_code: Vec<u8>) -> (Vec<Instruction>, Option<Vec<Directive>>) {
    let instructions = lex_instr(source_code);
    for instruction in &instructions.0 {
        println!("{:?}", instruction);
    }
    for directive in &instructions.1 {
        println!("{:?}", directive);
    }
    instructions
}

fn lex_instr(source_code: Vec<u8>) -> (Vec<Instruction>, Option<Vec<Directive>>) {
    let mut used_chars: [char; 75] = [0 as char; 75];
    let spec_chars = [':', '#', '[', ']', '!', '\n', '*', '&', '+', '-', '<', '>', '@'];
    for (i,c) in ('a'..='z').enumerate() { used_chars[i] = c; }
    for (i,c) in ('A'..='Z').enumerate() { used_chars[i+26] = c; }
    for (i,c) in ('0'..='9').enumerate() { used_chars[i+52] = c; }
    for (i,c) in spec_chars.iter().enumerate() { used_chars[i+62] = *c; }
  
    

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut directives: Vec<Directive> = Vec::new();

    let mut comment = false;

    let mut isDirective = false;
    let mut pushDirective = false;
    let mut parseDirective = false;
    let mut DirStr = String::new();
    let mut parseDirArgs = false;
    let mut DirArgStr = String::new();


    let mut isVariable = false;

    let mut column: usize = 0;
    let mut row: usize = 1;

    for sym_code in source_code {
        column += 1;
        if sym_code == 0xA {
            if column == 1 { 
                column = 0;
                row += 1;
                continue
            }
            column = 0;
            row += 1;
        }

        let symbol: char = sym_code as char;
        
        if symbol == ' ' { continue }

        if symbol == ';' {
            comment = true;
            continue
        }
        if comment {
            if sym_code == 0xA {
                comment = false;
                continue
            } else {
                continue
            }
        }


        if !used_chars.contains(&symbol) {
            error(0, row, column, symbol)
        }

        if isDirective {
            if !parseDirective {
                match symbol {
                    '\n' => {
                        isDirective = false;
                        pushDirective = true;
                    },
                    'A'..='Z' => parseDirective = true,
                    _ => unreachable!()
                }
            } 
            if parseDirective {
                if !parseDirArgs {
                    match symbol {
                        '<' => parseDirArgs = true,
                        'A'..='Z' => DirStr.push(symbol),
                        _ => unreachable!()
                    }
                } else {
                    if symbol == '>' {
                        parseDirArgs = false;
                        isDirective = false;
                        parseDirective = false;
                        pushDirective = true;
                    } else {
                        DirArgStr.push(symbol);
                    }
                }
            }

            if pushDirective {
                pushDirective = false;
                match get_type_dir(DirStr.clone()) {
                    Ok(t) => {
                        if t {
                            match get_directive_args(DirStr, DirArgStr) {
                                Ok(dir) => directives.push(dir),
                                Err(_) => error(6, row, column, symbol)
                            }
                        } else {
                            unreachable!();
                            //match get_directive(DirStr) {
                            //    Ok(dir) => directives.push(dir),
                            //    Err(_) => unreachable!()
                            //}
                        }
                    },
                    Err(_) => error(5, row, column, symbol)
                }
                DirStr = String::new();
                DirArgStr = String::new();
            }
        } else if isVariable {

        } else {
            if symbol == '\n' { continue }
            match symbol {
                '!' => isDirective = true,
                'B'|'W' => isVariable = true,
                _ => unreachable!(symbol)
            }
        }
    }
    if directives.len() > 0 {
        (instructions, Some(directives))
    } else {
        (instructions, None) 
    }
}



/*
 * error codes:
 *  - 0: unknown token
 *  - 1: unspecifed function signature
 *  - 2: unknown variable size
 *  - 3: failed to parse immediate value
 *  - 4: incorrect function
 *  - 5: Unknown directive
 *  - 6: Failed to parse arguments of directive
 */
fn error(err_code: u8, row: usize, column: usize, symbol: char) {
    println!("{}", symbol as u8);
    match err_code {
        0 => eprintln!("Unknown token at {}:{}", row, column),
        1 => eprintln!("Unspecifed function signature at {}:{}", row, column),
        2 => eprintln!("Unknown variable size at {}:{}", row, column),
        3 => eprintln!("Failed to parse immediate value at {}:{}", row, column),
        4 => eprintln!("Incorrect function ar {}:{}", row, column),
        5 => eprintln!("Unknown directive at {}:{}", row, column),
        6 => eprintln!("Failed to parse arguments of directive at {}:{}", row, column),
        _ => panic!("Unreachable error code")
    }
    std::process::exit(1);
}

fn get_size(size_str: String) -> Result<Size, ()> {
    match size_str.as_str() {
        "BYTE" => Ok(Size::Byte),
        "WORD" => Ok(Size::Word),
        _ => Err(())
    }
}

fn get_type_dir(dir: String) -> Result<bool, ()> {
    match dir.as_str() {
        "USES" => Ok(true),
        _ => Err(())
    }
}

fn get_directive_args(dir: String, args: String) -> Result<Directive, ()> {
    let arguments: Vec<Indent> = vec![Indent(args)];
    match dir.as_str() {
        "USES" => Ok(Directive::Use(arguments)),
        _ => Err(())
    }
}

