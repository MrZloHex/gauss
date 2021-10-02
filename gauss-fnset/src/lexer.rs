#![allow(non_snake_case_names)]
#![allow(non_camel_case_types)]


use crate::instr::*;


pub fn lex_code(source_code: Vec<u8>) -> String {
   //let functions = lex_functions(source_code);
   let functions = lex_func(source_code);
   println!("{:?}", functions);
   "qwe".to_string()
}

fn lex_func(source_code: Vec<u8>) -> Vec<Function> {
    let mut used_chars: [char; 75] = [0 as char; 75];
    let spec_chars = [':', '#', '[', ']', '|', '\n', '*', '&', '+', '-', '/', '\\', '_'];
    for (i,c) in ('a'..='z').enumerate() { used_chars[i] = c; }
    for (i,c) in ('A'..='Z').enumerate() { used_chars[i+26] = c; }
    for (i,c) in ('0'..='9').enumerate() { used_chars[i+52] = c; }
    for (i,c) in spec_chars.iter().enumerate() { used_chars[i+62] = *c; }
  
    

    let mut functions: Vec<Function> = Vec::new();
    let mut arguments: Vec<Argument> = Vec::new();

    let mut checkFunc = false;

    let mut isFunc = false;
    let mut pushFunc = false;

    let mut pushFuncName = false;
    let mut FuncName = String::new();

    let mut parseIndent = false;
    let mut indent = String::new();

    let mut parseArgs = false;
    let mut parseSizeArg = false;
    let mut SizeArg: Size = Size::Byte;
    let mut SizeArgStr = String::new();
    let mut parseIndentArg = false;
    let mut IndentArg = String::new();
    let mut pushArg = false;
    let mut Arg: Argument;

    let mut parseRet = false;
    let mut parseSizeRet = false;
    let mut SizeRet = Size::Byte;
    let mut SizeRetStr = String::new();
    let mut pushRet = false;

    let mut parseCode = false;

    let mut beginOfLine = true;


    let mut row: usize = 1;
    let mut column: usize = 0;


    let mut comment = false;

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

        if isFunc {
            if symbol == '_' { isFunc = false; pushFunc = true; }
        } else {
            if sym_code == 0xA { continue }
            if !parseRet && !parseArgs && !parseIndent {
                match symbol {
                    'B'|'W'|'N'|'D' => parseRet = true,
                    'a'..='z'|'0'..='9' => parseIndent = true,
                    '[' => parseArgs = true,
                    _ => unreachable!()
                }
            }


            if parseRet {
                match symbol {
                    'a'..='z'|'0'..='9' => {
                        parseRet = false;
                        pushRet = true;
                        parseIndent = true;
                    },
                    'A'..='Z' => (),
                    _ => unreachable!()
                }
                if !pushRet {
                    SizeRetStr.push(symbol)
                } else {
                    pushRet = false;
                    match get_size(SizeRetStr) {
                        Ok(sz) => SizeRet = sz,
                        Err(_) => error(2, row, column, symbol)
                    }
                    SizeRetStr = String::new();
                }
            }

            if parseIndent {
                if symbol == ':' {
                    parseIndent = false;
                    FuncName = indent;
                    indent = String::new();
                } else {
                    indent.push(symbol);
                }
            }

            if parseArgs {
                match symbol {
                    '[' => (),
                    'A'..='Z' => parseSizeArg = true,
                    'a'..='z'|'0'..='9' => parseIndentArg = true,
                    '|' => { 
                        pushArg = true;
                        parseIndentArg = false;
                    },
                    ']' => {
                        pushArg = true;
                        parseArgs = false;
                        parseIndentArg = false;
                        isFunc = true;
                    },
                    _ => unreachable!()
                }
                if parseSizeArg {
                    match symbol {
                        'a'..='z'|'0'..='9' => {
                            parseSizeArg = false;
                            parseIndentArg = true;
                        },
                        'A'..='Z' => (),
                        _ => unreachable!()
                    }
                    if !parseIndentArg {
                        SizeArgStr.push(symbol);
                    } else {
                        match get_size(SizeArgStr) {
                            Ok(sz) => SizeArg = sz,
                            Err(_) => error(2, row, column, symbol),
                        }
                        SizeArgStr = String::new();
                    }
                }

                if parseIndentArg {
                    IndentArg.push(symbol);
                }

                if pushArg {
                    pushArg = false;
                    if IndentArg.is_empty() {
                        continue;
                    }
                    Arg = Argument {
                        name: Indent(IndentArg),
                        size: SizeArg,
                    };
                    arguments.push(Arg);
                    IndentArg = String::new();
                }
            }
        }

        if pushFunc {
            pushFunc = false;
            let func = if arguments.len() > 0 {
                Function {
                    name: Indent(FuncName),
                    argc: arguments.len(),
                    args: Some(arguments),
                    ret_size: SizeRet
                }
            } else {
                Function {
                    name: Indent(FuncName),
                    argc: arguments.len(),
                    args: None,
                    ret_size: SizeRet
                }
            }; 

            FuncName = String::new();
            arguments = Vec::new();

            functions.push(func);
        }







    }


    functions
}



/*
 * error codes:
 *  - 0: unknown token
 *  - 1: unspecifed function signature
 *  - 2: unknown variable size
 */
fn error(err_code: u8, row: usize, column: usize, symbol: char) {
    println!("{}", symbol as u8);
    match err_code {
        0 => eprintln!("Unknown token at {}:{}", row, column),
        1 => eprintln!("Unspecifed function signature at {}:{}", row, column),
        2 => eprintln!("Unknown variable size at {}:{}", row, column),
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

