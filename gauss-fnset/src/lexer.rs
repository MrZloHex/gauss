use crate::instr::*;

pub fn lex_code(source_code: Vec<String>) -> String {
   let functions = lex_functions(source_code);
   "qwe".to_string()
}

fn lex_functions(source_code: Vec<String>) -> Vec<Function> {
    let mut functions: Vec<Function> = Vec::new();

    let mut is_func: bool = false;
    let mut is_args: bool = false;
    let mut push_fn: bool = false;

    'line: for (row, line) in source_code.iter().enumerate() {
        let mut args: Vec<(Size, String)> = Vec::new();
        let mut name: String = String::new();

        let mut arg_name: String = String::new();
        let mut arg_size: Size = Size::Byte;
        let mut is_arg_size: bool = false;

        let mut ret_size: Size = Size::Byte;
        let mut is_ret: bool = false;

        let mut next_arg: bool = false;
        let mut push_arg: bool = false;

        let tokens: Vec<&str> = line.split_whitespace().collect();
        for (column, token) in tokens.iter().enumerate() {
            if token.chars().nth(0).unwrap() == ';' {
                continue 'line;
            }
            
            if is_func {
                if is_args {
                    if token.clone().eq("]") {
                        is_args = false;
                        push_fn = true;
                        push_arg = true;
                        next_arg = false;
                    } else if token.clone().eq("|") {
                        next_arg = true;
                        push_arg = true;
                    } else {
                        if is_arg_size {
                            is_arg_size = false;
                            arg_name = token.to_string();
                        } else {
                            is_arg_size = true;
                            arg_size = get_size(token);
                        }
                    }
                } else {

                if is_ret {
                    ret_size = get_size(token);
                    is_ret = false;
                } else {

                match *token {
                    "[" => is_args = true,
                    "=>" => is_ret = true,
                    _ => {
                        eprintln!("Unknown token:\"{}\"", token);
                        std::process::exit(1);
                    }
                }
                }
                }
            } else {
                if column == 0 && token.chars().last().unwrap() == ':' {
                    is_func = true;
                    name = token.to_string();
                    name.pop();
                }
            }
            if push_arg {
                args.push((arg_size.clone(), arg_name.clone()));
                push_arg = false;
            }
        }
        if push_fn {
            let function = Function {
                name,
                argc: args.len(),
                argv: args,
                ret_size
            };
            functions.push(function);
            push_fn = false;
            break;
        }
    }

    println!("{}", functions.len());
    println!("{}", functions[0].name);
    println!("{}", functions[0].argc);
    println!("{}", functions[0].argv[0].1);
    println!("{}", functions[0].argv[1].1);
    match functions[0].argv[0].0 {
        Size::Byte => println!("BYTE"),
        Size::Word => println!("WORD")
    }
    match functions[0].argv[1].0 {
        Size::Byte => println!("BYTE"),
        Size::Word => println!("WORD")
    }
    match functions[0].ret_size {
        Size::Byte => println!("BYTE"),
        Size::Word => println!("WORD")
    }
    
    functions
}


fn get_size(size_str: &str) -> Size {
    match size_str {
        "BYTE" => Size::Byte,
        "WORD" => Size::Word,
        _ => {
            eprintln!("Incorrect variable size: \"{}\"", size_str);
            std::process::exit(1);
        }
    }
}

