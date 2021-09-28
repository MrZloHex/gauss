use crate::instr::*;

pub fn lex_code(source_code: Vec<String>) -> String {
   let functions = lex_functions(source_code);
   "qwe".to_string()
}

fn lex_functions(source_code: Vec<String>) -> Vec<Function> {
    let mut functions: Vec<Function> = Vec::new();

    let mut loc_var: Vec<(Size, String)> = Vec::new();
    let mut code: Vec<String> = Vec::new();

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

        let mut loc_var_size: Size = Size::Byte;
        let mut loc_var_name: String = String::new();
        let mut is_loc_var: bool = false;
        let mut push_loc_var: bool = false;

        let mut is_fn_block: bool = false;

        let mut next_arg: bool = false;
        let mut push_arg: bool = false;

        let mut is_token: bool = false;

        let tokens: Vec<&str> = line.split_whitespace().collect();
        'token: for (column, token) in tokens.iter().enumerate() {
            if token.chars().nth(0).unwrap() == ';' {
                continue 'line;
            }
            
            if is_func {
                if !is_token {
                    match *token {
                        "["   =>      {is_args = true; is_token = true; continue 'token;},
                        "=>"  =>       {is_ret = true; is_token = true; continue 'token;},
                        "|"   =>  {is_fn_block = true; is_token = true; continue 'token;},
                        "\\_" => {is_fn_block = false; is_token = true; continue 'token;},
                        _ => {
                            eprintln!("Unknown token:\"{}\"", token);
                            std::process::exit(1);
                        }
                    };
                }
                if is_args {
                    if token.clone().eq("]") {
                        is_args = false;
                        push_fn = true;
                        push_arg = true;
                        next_arg = false;
                        is_token = false;
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
                } 

                if is_ret {
                    ret_size = get_size(token);
                    is_ret = false;
                }

                if is_fn_block {
                    if is_size(token) {
                        is_loc_var = true;
                        loc_var_size = get_size(token);
                    }
                    if is_loc_var {
                        loc_var_name = token.to_string();
                        is_loc_var = false;
                        push_loc_var = true;
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
            if push_loc_var {
                loc_var.push((loc_var_size.clone(), loc_var_name.clone()));
                push_loc_var = false;
            }

            if is_fn_block {
                code.push(line.replace("|", ""))
            }
        }
        if push_fn {
            println!("{:?}", loc_var);
            let function = Function {
                name,
                argc: args.len(),
                argv: args,
                ret_size,
                loc_var_c: loc_var.len(),
                loc_var: loc_var.clone(),
                code
            };
            functions.push(function);
            push_fn = false;
            code = Vec::new();
            loc_var = Vec::new();
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
    println!("{}", functions[0].loc_var_c);
    println!("{}", functions[0].loc_var[0].1);
    println!("{}", functions[0].code[0]);
    
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

fn is_size(size_str: &str) -> bool {
    match size_str {
        "BYTE" => true,
        "WORD" => true,
        _ => false
    }
}

