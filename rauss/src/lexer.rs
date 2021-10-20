#![allow(non_snake_case)]
#![allow(dead_code)]


use crate::types::*;

pub fn lex_instr(source_code: Vec<u8>) -> Vec<Instruction> {
    let mut used_chars: [char; 77] = [0 as char; 77];
    let spec_chars = [':', '#', '[', ']', '\n', '*', '&', '+', '-', '<', '>', '@', '|', '.', '='];
    for (i,c) in ('a'..='z').enumerate() { used_chars[i] = c; }
    for (i,c) in ('A'..='Z').enumerate() { used_chars[i+26] = c; }
    for (i,c) in ('0'..='9').enumerate() { used_chars[i+52] = c; }
    for (i,c) in spec_chars.iter().enumerate() { used_chars[i+62] = *c; }
  
    

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut comment = false;

    let mut isDirective = false;
    let mut pushDirective = false;
    let mut parseDirective = false;
    let mut DirStr = String::new();
    let mut parseDirArgs = false;
    let mut DirArgStr = String::new();
    let mut pushDirArg = false;
    let mut DirArgs: Vec<String> = Vec::new();

    let mut isVariable = false;
    let mut parseSizeVar = false;
    let mut pushSizeVar = false;
    let mut parseIndentVar = false;
    let mut SizeVarStr = String::new();
    let mut SizeVar = Size::Byte;
    let mut VarName = String::new();
    let mut indent = String::new();
    let mut isVarInit = false;
    let mut parseValueVar = false;
    let mut pushVar = false;
    let mut pushValVar = false;
    let mut ValVar = Value::Byte(0);
    let mut ValVarStr = String::new();

    let mut isAssignment = false;
    let mut parseVarIndent = false;
    let mut parseValueType = false;
    let mut VarIndent = String::new();
    let mut parseImmValue = false;
    let mut VarValStr = String::new();
    let mut VarVal = ValueType::Immediate(Value::Byte(0));
    let mut AssVal = ValueType::Immediate(Value::Byte(0));
    let mut parseValVarIndent = false;
    let mut ValVarIndent = String::new();
    let mut parseFuncIndent = false;
    let mut pushAssignment = false;

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

        if !isVariable && !isAssignment {
            if symbol == '\n' { continue }
            match symbol {
                'B'|'W' => isVariable = true,
                'a'..='z'|'0'..='9' => isAssignment = true,
                _ => unreachable!(symbol)
            }
        }

        if isVariable {
            if !parseSizeVar && !parseIndentVar && !parseValueVar {
                match symbol {
                    'B'|'W'|'D' => parseSizeVar = true,
                    'a'..='z'|'0'..='9' => parseIndentVar = true,
                    '#' => parseValueVar = true,
                    _ => unreachable!(symbol)
                }
            }

            if parseSizeVar {
                match symbol {
                    'a'..='z'|'0'..='9' => {
                        parseSizeVar = false;
                        pushSizeVar = true;
                        parseIndentVar = true;
                    },
                    'A'..='Z' => (),
                    _ => unreachable!()
                }
                if !pushSizeVar {
                    SizeVarStr.push(symbol)
                } else {
                    pushSizeVar = false;
                    match get_size(SizeVarStr) {
                        Ok(sz) => SizeVar = sz,
                        Err(_) => error(2, row, column, symbol)
                    }
                    SizeVarStr = String::new();
                }
            }

            if parseIndentVar {
                if symbol == ':' {
                    parseIndentVar = false;
                    VarName = indent;
                    isVarInit = true;
                    indent = String::new();
                } else if symbol == '\n' {
                    parseIndentVar = false;
                    VarName = indent;
                    isVarInit = false;
                    indent = String::new();
                    pushVar = true;
                } else {
                    indent.push(symbol);
                }
            }

            if parseValueVar {
                match symbol {
                    '#' => continue,
                    '0'..='9' => (),
                    '\n' => {
                        parseValueVar = false;
                        pushVar = true;
                        pushValVar = true;
                    },
                    _ => unreachable!(symbol)
                }
                if pushValVar {
                    pushValVar = false;
                    ValVar = match SizeVar {
                        Size::Byte => {
                            match ValVarStr.parse::<u8>() {
                                Ok(val) => Value::Byte(val),
                                Err(_) => { error(3, row, column, symbol); Value::Byte(0) }
                            }
                        },
                        Size::Word => {
                            match ValVarStr.parse::<u16>() {
                                Ok(val) => Value::Word(val),
                                Err(_) => { error(3, row, column, symbol); Value::Word(0) }
                            }
                        }
                    };
                    ValVarStr = String::new();
                } else {
                    ValVarStr.push(symbol);
                }
            }


            if pushVar {
                pushVar = false;
                isVariable = false;
                let var = if isVarInit {
                    Variable {
                        name: Indent(VarName),
                        size: SizeVar,
                        init: Init::Initilized(ValVar)
                    }
                } else {
                    Variable {
                        name: Indent(VarName),
                        size: SizeVar,
                        init: Init::Uninitilized
                    }
                };
                instructions.push(Instruction::Variable(var));
                VarName = String::new();
                indent = String::new();
                parseSizeVar = false;
                parseIndentVar = false;
                parseValueVar = false;
            }
        }


        if isAssignment {
            if !parseVarIndent && !parseValueType {
                match symbol {
                    'a'..='z'|'0'..='9' => parseVarIndent = true,
                    _ => unreachable!(symbol)
                }
            }

            if parseVarIndent {
                if symbol == '=' {
                    parseVarIndent = false;
                    VarIndent = indent;
                    indent = String::new();
                    parseValueType = true;
                    continue;
                } else {
                    indent.push(symbol);
                }
            }
            if parseValueType {
                match symbol {
                    '#' => parseImmValue = true,
                    '@' => parseFuncIndent = true,
                    'a'..='z'|'0'..='9' => parseValVarIndent = true,
                    _ => unreachable!(symbol)
                }
                parseValueType = false;
            } 
            if !parseValueType {
                if parseImmValue {
                    if symbol == '#' { continue }
                    if symbol == '\n' {
                        parseImmValue = false;
                        let value = match VarValStr.parse::<u64>() {
                            Ok(val) => val,
                            Err(_) => { error(3, row, column, symbol); 0 }
                        };
                        if value < 256 {
                            VarVal = ValueType::Immediate(Value::Byte(value as u8))
                        } else if value < 65536 {
                            VarVal = ValueType::Immediate(Value::Word(value as u16))
                        } else {
                            error(7, row, column, symbol)
                        }
                        VarValStr = String::new();
                        AssVal = VarVal.clone();
                        pushAssignment = true;
                    } else {
                        VarValStr.push(symbol)
                    }
                } else if parseFuncIndent {
                    if symbol == '@' { continue }
                } else if parseValVarIndent {
                    if symbol == '\n' {
                        parseValVarIndent = false;
                        AssVal = ValueType::Variable(Indent(ValVarIndent));
                        ValVarIndent = String::new();
                        pushAssignment = true;
                    } else {
                        ValVarIndent.push(symbol);
                    }
                }
            }
            
            if pushAssignment {
                pushAssignment = false;
                isAssignment = false;
                let assign = Assignment {
                    var_name: Indent(VarIndent),
                    val: AssVal.clone()
                };
                VarIndent = String::new();
                indent = String::new();
                instructions.push(Instruction::Assignment(assign));
            }
        }
    }

    instructions
}



pub fn lex_func(source_code: Vec<u8>) -> Vec<Function> {
    let mut used_chars: [char; 75] = [0 as char; 75];
    let spec_chars = [':', '#', '[', ']', '|', '\n', '*', '&', '+', '-', '/', '\\', '_'];
    for (i,c) in ('a'..='z').enumerate() { used_chars[i] = c; }
    for (i,c) in ('A'..='Z').enumerate() { used_chars[i+26] = c; }
    for (i,c) in ('0'..='9').enumerate() { used_chars[i+52] = c; }
    for (i,c) in spec_chars.iter().enumerate() { used_chars[i+62] = *c; }
  
    

    let mut functions: Vec<Function> = Vec::new();
    let mut arguments: Vec<Argument> = Vec::new();
    let mut variables: Vec<Variable> = Vec::new();

    let mut isFunc = false;
    let mut isFuncEnd = false;
    let mut pushFunc = false;

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
    let mut SizeRet = Size::Byte;
    let mut SizeRetStr = String::new();
    let mut pushRet = false;


    let mut row: usize = 1;
    let mut column: usize = 0;


    let mut comment = false;


    let mut pushVar = false;

    let mut parseSizeVar = false;
    let mut pushSizeVar = false;
    let mut SizeVarStr = String::new();
    let mut SizeVar = Size::Byte;

    let mut parseIndentVar = false;
    let mut VarName = String::new();

    let mut parseValueVar = false;
    let mut pushValVar = false;
    let mut ValVar = Value::Byte(0);
    let mut ValVarStr = String::new();

    let mut parseRetExpr = false;
    let mut parseRetVar = false;
    let mut pushRetExpr = false;
    let mut RetVar = String::new();

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
            if isFuncEnd {
                if symbol == '_' {
                    isFunc = false;
                    isFuncEnd = false;
                    pushFunc = true;
                }
            }

            if symbol == '|' { continue }
            if isFunc && !parseSizeVar && !parseRetExpr && !parseIndentVar && !parseValueVar && !isFuncEnd {
                match symbol {
                    'B'|'W'|'N'|'D' => parseSizeVar = true,
                    'R' => parseRetExpr = true,
                    'a'..='z'|'0'..='9' => parseIndentVar = true,
                    '#' => parseValueVar = true,
                    '\\' => isFuncEnd = true,
                    '\n' => (),
                    _ => unreachable!(symbol),
                }
            }

            if parseSizeVar {
                match symbol {
                    'a'..='z'|'0'..='9' => {
                        parseSizeVar = false;
                        pushSizeVar = true;
                        parseIndentVar = true;
                    },
                    'A'..='Z' => (),
                    _ => unreachable!()
                }
                if !pushSizeVar {
                    SizeVarStr.push(symbol)
                } else {
                    pushSizeVar = false;
                    match get_size(SizeVarStr) {
                        Ok(sz) => SizeVar = sz,
                        Err(_) => error(2, row, column, symbol)
                    }
                    SizeVarStr = String::new();
                }
            }

            if parseIndentVar {
                if symbol == ':' {
                    parseIndentVar = false;
                    VarName = indent;
                    indent = String::new();
                } else {
                    indent.push(symbol);
                }
            }

            if parseValueVar {
                match symbol {
                    '#' => continue,
                    '0'..='9' => (),
                    '\n' => {
                        parseValueVar = false;
                        pushVar = true;
                        pushValVar = true;
                    },
                    _ => unreachable!(symbol)
                }
                if pushValVar {
                    pushValVar = false;
                    ValVar = match SizeVar {
                        Size::Byte => {
                            match ValVarStr.parse::<u8>() {
                                Ok(val) => Value::Byte(val),
                                Err(_) => { error(3, row, column, symbol); Value::Byte(0) }
                            }
                        },
                        Size::Word => {
                            match ValVarStr.parse::<u16>() {
                                Ok(val) => Value::Word(val),
                                Err(_) => { error(3, row, column, symbol); Value::Word(0) }
                            }
                        }
                    };
                    ValVarStr = String::new();
                } else {
                    ValVarStr.push(symbol);
                }
            }

            if parseRetExpr {
                if !parseRetVar {
                    match symbol {
                        'R'|'E'|'T' => (),
                        '[' => parseRetVar = true,
                        _ => unreachable!()
                    }
                } else {
                    match symbol {
                        'a'..='z'|'0'..='9' => (),
                        ']' => {
                            parseRetVar = false;
                            parseRetExpr = false;
                            pushRetExpr = true;
                        },
                        _ => unreachable!()
                    }
                    if pushRetExpr {
                        pushRetExpr = false;
                        RetVar = indent;
                        indent = String::new();
                    } else {
                        indent.push(symbol);
                    }
                }
            }


            if pushVar {
                pushVar = false;
                let var = Variable {
                    name: Indent(VarName),
                    size: SizeVar,
                    init: Init::Initilized(ValVar),
                };
                variables.push(var);

                VarName = String::new();
            }
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
                if variables.len() > 0 {
                    Function {
                        name: Indent(FuncName),
                        argc: arguments.len(),
                        args: Some(arguments),
                        ret_size: SizeRet,
                        vars: Some(variables),
                        ret_var: Indent(RetVar)
                    }
                } else {
                    Function {
                        name: Indent(FuncName),
                        argc: arguments.len(),
                        args: Some(arguments),
                        ret_size: SizeRet,
                        vars: None,
                        ret_var: Indent(RetVar)
                    }
                }
            } else {
                if variables.len() > 0 {
                    Function {
                        name: Indent(FuncName),
                        argc: arguments.len(),
                        args: None,
                        ret_size: SizeRet,
                        vars: Some(variables),
                        ret_var: Indent(RetVar)
                    }
                } else {
                    Function {
                        name: Indent(FuncName),
                        argc: arguments.len(),
                        args: None,
                        ret_size: SizeRet,
                        vars: None,
                        ret_var: Indent(RetVar)
                    }
                }
            }; 

            FuncName = String::new();
            arguments = Vec::new();
            variables = Vec::new();
            RetVar = String::new();

            if check_func(func.clone()) {
                functions.push(func);
            } else {
                error(4, row, column, symbol);
            }
        }
    }

    functions
}

pub fn lex_direct(code: Vec<u8>) -> Vec<Directive> {
    let mut directives: Vec<Directive> = Vec::new();

    let mut isDirective = false;
    let mut parseDirective = false;
    let mut pushDirective = false;
    let mut parseDirArgs = false;
    let mut pushDirArg = false;
    let mut DirArgStr = String::new();
    let mut DirArgs: Vec<String> = Vec::new();
    let mut parseDirType = false;
    let mut DirTypeStr = String::new();
    let mut parseDirIndent = false;
    let mut DirIndentStr = String::new();

    let mut column: usize = 0;
    let mut row:    usize = 1;

    for sym_code in code {
        column += 1;
        if sym_code == 0xA {
            if column == 1 {
                column = 0;
                row += 1;
                continue;
            }
            column = 0;
            row += 1;
        }

        let symbol: char = sym_code as char;

        if symbol == ' ' { continue }

        if column == 1 && symbol == '!' {
            isDirective = true;
            continue;
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
                if !parseDirArgs && !parseDirType && !parseDirIndent {
                    match symbol {
                        '<' => parseDirArgs = true,
                        'A'..='Z' => parseDirType = true,
                        _ => unreachable!()
                    }
                }
                if parseDirType {
                    match symbol {
                        'A'..='Z' => DirTypeStr.push(symbol),
                        '<' => {
                            parseDirArgs = true;
                            parseDirType = false;
                        },
                        '_'|'a'..='z'|'1'..='9' => {
                            parseDirIndent = true;
                            parseDirType   = false;
                        },
                        _ => error(0, row, column, symbol)
                    }
                }
                if parseDirIndent {
                    if symbol == '<' {
                        parseDirArgs = true;
                        parseDirIndent = false;
                    } else if symbol == '\n' {
                        parseDirIndent = false;
                    } else {
                        DirIndentStr.push(symbol);
                    }
                }
                if parseDirArgs {
                    if symbol == '>' {
                        parseDirArgs = false;
                        isDirective = false;
                        parseDirective = false;
                        pushDirArg = true;
                        pushDirective = true;
                    } else if symbol == '|' {
                        pushDirArg = true;
                    } else if symbol == '<' {
                        ()
                    } else {
                        DirArgStr.push(symbol);
                    }

                    if pushDirArg {
                        pushDirArg = false;
                        DirArgs.push(DirArgStr);
                        DirArgStr = String::new();
                    }
                }
            }
        }
        if pushDirective {
            pushDirective = false;
            if get_type_dir(DirTypeStr.clone()) {
                match get_directive(DirTypeStr, DirIndentStr, DirArgs) {
                    Ok(dir) => directives.push(dir),
                    Err(_) => error(6, row, column, symbol)
                }
            } else {
                error(5, row, column, symbol)
            }
            
            DirTypeStr = String::new();
            DirIndentStr = String::new();
            DirArgs = Vec::new();
        }
    }


    return directives
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
 *  - 7: Unknown value size
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
        7 => eprintln!("Unknown value size at {}:{}", row, column),
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

fn get_type_dir(dir: String) -> bool {
    match dir.as_str() {
        "USES" => true,
        "SET" => true,
        _ => false
    }
}

fn get_directive(dir: String, indent: String, args: Vec<String>) -> Result<Directive, ()> {
    match dir.as_str() {
        "USES" => {
            let arguments: Vec<Indent> = args.iter().map(| arg: &String | Indent((*arg).clone()) ).collect();
            Ok(Directive::Use(arguments))
        },
        "SET" => {
            let set = SetDir {
                name: Indent(indent),
                value: args[0].clone()
            };
            Ok(Directive::Set(set))
        },
        _ => Err(())
    }
}

fn check_func(function: Function) -> bool {
    if let Some(vars) = function.vars {
        for var in vars {
            if var.name.0 == function.ret_var.0 {
                if var.size == function.ret_size {
                    return true
                } else {
                    return false
                }
            }
        }
        false
    } else if let Some(args) = function.args {
        for arg in args {
            if arg.name.0 == function.ret_var.0 {
                if arg.size == function.ret_size {
                    return true
                } else {
                    return false
                }
            }
        }
        false
    } else {
        return false
    }
}
