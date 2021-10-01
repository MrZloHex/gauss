#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


use crate::instr::*;


pub fn lex_code(source_code: Vec<u8>) -> String {
   //let functions = lex_functions(source_code);
   let functions = lex_func(source_code);
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

    'line: for (row, line) in source_code.iter().enumerate() {
        if isFunc {
            //'sym: for (column, symbol) in (*line).chars().enumerate() {
            //    if beginOfLine {
            //        match symbol {
            //            ' ' => continue 'sym,
            //            '|' => (),
            //            '\\' => {
            //                isFunc = false;
            //                checkFunc = true;
            //            },
            //            '1'..='9' | 'A'..='Z' | 'a'..='z' => beginOfLine = false,
            //            _ => error(0, row, column),
            //        }
            //    } 
            //    if !beginOfLine {
            //        //match symbol {
            //        //    '0'..=':' | 'a'..='z' | 'A'..='Z' | '#' | '*' | '+' | '     
            //        //}
            //    }
            //}
        } else {
            //for (column, symbol) in (*line).chars().enumerate() {
            //    if column == 0 {
            //        match symbol {
            //            ';' => continue 'line,
            //            '|' | '\\' => error(1, row, column),
            //            'a'..='z' | '0'..='9' => {
            //                parseIndent = true;
            //                isFunc = true;
            //            },
            //            _ => error(0, row, column)
            //        }
            //    } else {
            //        if !parseIndent && !parseArgs && !parseRet {
            //            match symbol {
            //                ' ' => (),
            //                '[' => parseArgs = true,
            //                '>' => parseRet = true,
            //                _ => error(0, row, column)
            //            }
            //        }
            //    }
            //    
            //    if parseIndent {
            //        if symbol == ':' {
            //            parseIndent = false;
            //            pushFuncName = true;
            //        } else {
            //            indent.push(symbol);
            //        }
            //    }

            //    if parseArgs {
            //        match symbol {
            //            ' ' | '[' => (),
            //            'A'..='Z' => parseSizeArg = true,
            //            'a'..='z' | '0'..='9' => parseIndentArg = true,
            //            '|' => {
            //                pushArg = true;
            //                parseSizeArg = false;
            //                parseIndentArg = false;
            //            },
            //            ']' => {
            //                pushArg = true;
            //                parseArgs = false;
            //                parseSizeArg = false;
            //                parseIndentArg = false;
            //            },
            //            _ => error(0, row, column)
            //        }


            //        if parseSizeArg {
            //            if symbol == ' ' {
            //                parseSizeArg = false;
            //                match get_size(SizeArgStr){
            //                    Ok(sz) => SizeArg = sz,
            //                    Err(_) => error(2, row, column),
            //                }
            //                //println!("{}", SizeArgStr);
            //                SizeArgStr = String::new()
            //            } else {
            //                SizeArgStr.push(symbol);
            //            }
            //        }
            //        if parseIndentArg {
            //            if !(symbol == ' ') {
            //                IndentArg.push(symbol);
            //            }
            //        }
            //    }

            //    if parseRet {
            //        match symbol {
            //            ' ' | '>' => (),
            //            'A'..='Z' => parseSizeRet = true,
            //            '<' => {
            //                parseSizeRet = false;
            //                pushRet = true;
            //            },
            //            _ => error(0, row, column)
            //        }
            //        if parseSizeRet {
            //            if !(symbol == ' ') {
            //                SizeRetStr.push(symbol);
            //            }
            //        }
            //    }



            //    if pushFuncName {
            //        pushFuncName = false;
            //        FuncName = indent;
            //        indent = String::new();
            //        println!("{}", FuncName);
            //    }

            //    if pushArg {
            //        pushArg = false;
            //        Arg = Argument {
            //            name: Indent(IndentArg),
            //            size: SizeArg,
            //        };
            //        println!("{:?}", Arg);
            //        arguments.push(Arg);
            //        IndentArg = String::new();
            //    }

            //    if pushRet {
            //        pushRet = false;
            //        match get_size(SizeRetStr) {
            //            Ok(sz) => SizeRet = sz,
            //            Err(_) => error(2, row, column)
            //        }
            //        println!("{:?}", SizeRet);
            //        SizeRetStr = String::new();
            //    }
            //}
        }
    }

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
                    println!("{:?}", SizeRet);
                }
            }

            if parseIndent {
                if symbol == ':' {
                    parseIndent = false;
                    FuncName = indent;
                    indent = String::new();
                    println!("{}", FuncName);
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
                    Arg = Argument {
                        name: Indent(IndentArg),
                        size: SizeArg,
                    };
                    println!("{:?}", Arg);
                    arguments.push(Arg);
                    IndentArg = String::new();
                }
            }
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














//fn lex_functions(source_code: Vec<String>) -> Vec<Function> {
//    let mut functions: Vec<Function> = Vec::new();
//
//    let mut loc_var: Vec<(Size, String)> = Vec::new();
//    let mut code: Vec<String> = Vec::new();
//
//    let mut is_func: bool = false;
//    let mut is_args: bool = false;
//    let mut push_fn: bool = false;
//
//    'line: for (row, line) in source_code.iter().enumerate() {
//        let mut args: Vec<(Size, String)> = Vec::new();
//        let mut name: String = String::new();
//
//        let mut arg_name: String = String::new();
//        let mut arg_size: Size = Size::Byte;
//        let mut is_arg_size: bool = false;
//
//        let mut ret_size: Size = Size::Byte;
//        let mut is_ret: bool = false;
//
//        let mut loc_var_size: Size = Size::Byte;
//        let mut loc_var_name: String = String::new();
//        let mut is_loc_var: bool = false;
//        let mut push_loc_var: bool = false;
//
//        let mut is_fn_block: bool = false;
//
//        let mut next_arg: bool = false;
//        let mut push_arg: bool = false;
//
//        let mut is_token: bool = false;
//
//        let tokens: Vec<&str> = line.split_whitespace().collect();
//        'token: for (column, token) in tokens.iter().enumerate() {
//            if token.chars().nth(0).unwrap() == ';' {
//                continue 'line;
//            }
//            
//            if is_func {
//                if !is_token {
//                    match *token {
//                        "["   =>      {is_args = true; is_token = true; continue 'token;},
//                        "=>"  =>       {is_ret = true; is_token = true; continue 'token;},
//                        "|"   =>  {is_fn_block = true; is_token = true; continue 'token;},
//                        "\\_" => {is_fn_block = false; is_token = true; continue 'token;},
//                        _ => {
//                            eprintln!("Unknown token:\"{}\"", token);
//                            std::process::exit(1);
//                        }
//                    };
//                }
//                if is_args {
//                    if token.clone().eq("]") {
//                        is_args = false;
//                        push_fn = true;
//                        push_arg = true;
//                        next_arg = false;
//                        is_token = false;
//                    } else if token.clone().eq("|") {
//                        next_arg = true;
//                        push_arg = true;
//                    } else {
//                        if is_arg_size {
//                            is_arg_size = false;
//                            arg_name = token.to_string();
//                        } else {
//                            is_arg_size = true;
//                            arg_size = get_size(token);
//                        }
//                    }
//                } 
//
//                if is_ret {
//                    ret_size = get_size(token);
//                    is_ret = false;
//                }
//
//                if is_fn_block {
//                    if is_size(token) {
//                        is_loc_var = true;
//                        loc_var_size = get_size(token);
//                    }
//                    if is_loc_var {
//                        loc_var_name = token.to_string();
//                        is_loc_var = false;
//                        push_loc_var = true;
//                    }
//
//                }
//            } else {
//                if column == 0 && token.chars().last().unwrap() == ':' {
//                    is_func = true;
//                    name = token.to_string();
//                    name.pop();
//                }
//            }
//            if push_arg {
//                args.push((arg_size.clone(), arg_name.clone()));
//                push_arg = false;
//            }
//            if push_loc_var {
//                loc_var.push((loc_var_size.clone(), loc_var_name.clone()));
//                push_loc_var = false;
//            }
//
//            if is_fn_block {
//                code.push(line.replace("|", ""))
//            }
//        }
//        if push_fn {
//            println!("{:?}", loc_var);
//            let function = Function {
//                name,
//                argc: args.len(),
//                argv: args,
//                ret_size,
//                loc_var_c: loc_var.len(),
//                loc_var: loc_var.clone(),
//                code
//            };
//            functions.push(function);
//            push_fn = false;
//            code = Vec::new();
//            loc_var = Vec::new();
//        }
//    }
//
//    println!("{}", functions.len());
//    println!("{}", functions[0].name);
//    println!("{}", functions[0].argc);
//    println!("{}", functions[0].argv[0].1);
//    println!("{}", functions[0].argv[1].1);
//    match functions[0].argv[0].0 {
//        Size::Byte => println!("BYTE"),
//        Size::Word => println!("WORD")
//    }
//    match functions[0].argv[1].0 {
//        Size::Byte => println!("BYTE"),
//        Size::Word => println!("WORD")
//    }
//    match functions[0].ret_size {
//        Size::Byte => println!("BYTE"),
//        Size::Word => println!("WORD")
//    }
//    println!("{:?}", functions[0].code);
//    println!("{}", functions[0].loc_var_c);
//    println!("{}", functions[0].loc_var[0].1);
//    
//    functions
//}
//
//
//fn get_size(size_str: &str) -> Size {
//    match size_str {
//        "BYTE" => Size::Byte,
//        "WORD" => Size::Word,
//        _ => {
//            eprintln!("Incorrect variable size: \"{}\"", size_str);
//            std::process::exit(1);
//        }
//    }
//}
//
//fn is_size(size_str: &str) -> bool {
//    match size_str {
//        "BYTE" => true,
//        "WORD" => true,
//        _ => false
//    }
//}
//
