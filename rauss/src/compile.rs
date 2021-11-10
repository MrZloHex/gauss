use std::collections::HashMap;

use crate::types::*;

pub fn into_nasm(
    instructions: Vec<Instruction>,
    variables: Vec<Variable>,
    functions: Vec<Function>,
) -> String {
    let mut code = String::new();

    let (u_vars, i_vars) = get_un_init_vars(&variables);
    let instructions = get_executable_instr(instructions);
    // section .bss
    code.push_str("SECTION .bss\n");
    for u_var in u_vars {
        let (size, quantity) = match u_var.size {
            Size::Byte => ("resb", 1_u8),
            Size::Word => ("resw", 1_u8),
            _ => unreachable!()
        };
        code.push_str(format!("\t_{}:\t{} {}\n", u_var.name.0, size, quantity).as_str());
    }
    code.push('\n');

    // section .data
    code.push_str("SECTION .data\n");
    for i_var in i_vars {
        let size = match i_var.size {
            Size::Byte => "db",
            Size::Word => "dw",
            _ => unreachable!()
        };
        let value = if let Init::Initilized(val) = i_var.init {
            match val {
                Value::Byte(v) => v as u16,
                Value::Word(v) => v,
                _ => unreachable!()
            }
        } else {
            unreachable!()
        };
        code.push_str(format!("\t_{}:\t{} {}\n", i_var.name.0, size, value).as_str());
    }
    code.push('\n');

    // section .text
    code.push_str("SECTION .text\n");

    // functions
    for function in &functions {
        let mut vars_offset: HashMap<Indent, u64> = HashMap::new();
        let mut args_offset: HashMap<Indent, u64> = HashMap::new();
        let mut vars_p: u64 = 0;
        let mut args_p: u64 = 0;

        code.push_str(format!("\t_{}_:\n", function.name.0).as_str());
        code.push_str("\t\tpush rbp\n");
        code.push_str("\t\tmov  rbp,\trsp\n");


        // Set offset betwen RBP and RSP for local vars
        let mut size_loc_vars: u64 = 0;
        for var in function.vars.clone() {
            match var.size {
                Size::Byte => size_loc_vars += 1,
                Size::Word => size_loc_vars += 2,
                _ => unreachable!()
            }
        }
        let mut size_args: u64 = 0;
        for arg in function.args.clone() {
            match arg.size {
                Size::Byte => size_args += 1,
                Size::Word => size_args += 2,
                _ => unreachable!()
            }
        }
        code.push_str(format!("\t\tsub  rsp,\t{}\n", size_loc_vars + size_args).as_str());

        // Calculatin offset from arguments and local vars
        vars_p += size_args;

        // Save function args
        for (i, arg) in function.args.iter().enumerate() {
            let (size, reg) = match arg.size {
                Size::Byte => {
                    args_offset.insert(arg.name.clone(), args_p+1);
                    args_p += 1;
                    ("BYTE", "al")
                },
                Size::Word => {
                    args_offset.insert(arg.name.clone(), args_p+2);
                    args_p += 2;
                    ("WORD", "ax")
                },
                _ => unreachable!()
            };
            let offset = args_offset.get(&arg.name).unwrap();
            code.push_str(format!("\t\tmov  rax, QWORD [rbp+{}]\n", (8 * (i+1)) + 8).as_str());
            code.push_str(format!("\t\tmov  {} [rbp-{}], {}\n", size, offset, reg).as_str());
        }

        // MAIN
        for var in &function.vars {
            // add to vars_offset
            match var.size {
                Size::Byte => {
                    vars_offset.insert(var.name.clone(), vars_p+1);
                    vars_p += 1;
                },
                Size::Word => {
                    vars_offset.insert(var.name.clone(), vars_p+2);
                    vars_p += 2;
                },
                _ => unreachable!()
            }
            match var.init {
                Init::Initilized(value) => {
                    let offset = vars_offset.get(&var.name).unwrap();
                    match value {
                        Value::Byte(val) => code.push_str(format!("\t\tmov  BYTE [rbp-{}],\t{}\n", offset, val).as_str()),
                        Value::Word(val) => code.push_str(format!("\t\tmov  WORD [rbp-{}],\t{}\n", offset, val).as_str()),
                        _ => unreachable!()
                    }
                },
                _ => (),
            }
        }

        // RET
        code.push_str("\t\tmov  rax,\t 0\n");
        let offset = if let Some(off) = vars_offset.get(&function.ret_var) {
            off
        } else {
            args_offset.get(&function.ret_var).unwrap()
        };
        match function.ret_size {
            Size::Byte => code.push_str(format!("\t\tmov  al,\tBYTE [rbp-{}]\n", offset).as_str()),
            Size::Word => code.push_str(format!("\t\tmov  ax,\tWORD [rbp-{}]\n", offset).as_str()),
            _ => unreachable!()
        }


        if size_loc_vars == 0 && size_args == 0{
            code.push_str("\t\tpop  rbp\n");
        } else {
            code.push_str("\t\tleave\n");
        }
        code.push_str("\t\tret\n");
    }

    // start
    code.push_str("\tglobal _start\n");
    code.push_str("\t_start:\n");
    for instruction in instructions {
        match instruction {
            Instruction::Assignment(assign) => match assign.val {
                AssignValue::Value(val) => match val {
                    ValueType::Immediate(val_imm) => {
                        let (value, size) = match val_imm {
                            Value::Byte(v) => (v as u16, "BYTE"),
                            Value::Word(v) => (v, "WORD"),
                            _ => unreachable!()
                        };
                        code.push_str(format!("\t\t; Assigning value `{}` to variable `{}`\n", value.clone(), assign.var_name.0.clone()).as_str());
                        code.push_str(
                            format!("\t\tmov\t{} [_{}], {}\n", size, assign.var_name.0, value).as_str(),
                        )
                    }
                    ValueType::Variable(var_name) => {
                        let var = get_variable(&variables, var_name.clone());
                        let (reg, size) = match var.size {
                            Size::Byte => ("al", "BYTE"),
                            Size::Word => ("ax", "WORD"),
                            _ => unreachable!()
                        };
                        code.push_str(format!("\t\t; Assigning variable `{}` to variable `{}`\n", var_name.0.clone(), assign.var_name.0.clone()).as_str());
                        code.push_str(
                            format!("\t\tmov  {}, {} [_{}]\n", reg, size, var_name.0).as_str(),
                        );
                        code.push_str(
                            format!("\t\tmov  {} [_{}], {}\n", size, assign.var_name.0, reg).as_str(),
                        )
                    }
                    ValueType::FunctionValue(func_call) => {
                        code.push_str(format!("\t\t; Assigning result of function `{}` to variable `{}`\n", func_call.name.0.clone(), assign.var_name.0.clone()).as_str());
                        code.push_str(pre_fn_args(func_call.clone(), &variables).as_str());
                        let (reg, size) = match get_size_variable(&variables, assign.var_name.clone()) {
                            Size::Byte => ("al", "BYTE"),
                            Size::Word => ("ax", "WORD"),
                            _ => unreachable!()
                        };
                        code.push_str(format!("\t\tcall _{}_\n", func_call.name.0).as_str());
                        code.push_str(post_fn_args(func_call.argc).as_str());
                        code.push_str(format!("\t\tmov\t{} [_{}], {}\n", size, assign.var_name.0, reg).as_str());
                    },
                },
                AssignValue::Expression(operation) => {
                    match operation {
                        Operation::Binary(bin_operation) => {
                            code.push_str(format!("\t\t; Assigning result expresion to variable `{}`\n", assign.var_name.0.clone()).as_str());
                            let size = get_size_variable(&variables, assign.var_name);
                            let (reg_op_1, reg_op_2) = match size {
                                Size::Byte => ("al", "bl"),
                                Size::Word => ("ax", "bx"),
                                _ => unreachable!()
                            };
                            match bin_operation.operand_1 {
                                ValueType::Immediate(imm_value) => {
                                    match imm_value {
                                        Value::Byte(v) => code.push_str(format!("\t\tmov\tal, {}\n", v).as_str()),
                                        Value::Word(v) => code.push_str(format!("\t\tmov\tax, {}\n", v).as_str()),
                                        _ => unreachable!()
                                    };  
                                },
                                ValueType::Variable(variable_name) => {
                                    let size_op = get_size_variable(&variables, variable_name.clone());
                                    if size_op == Size::Byte && size == Size::Word {
                                        code.push_str("\t\tmov\trax, 0\n");
                                    }
                                    match size_op {
                                        Size::Byte => code.push_str(format!("\t\tmov\tal, BYTE [_{}]\n", variable_name.0).as_str()),
                                        Size::Word => code.push_str(format!("\t\tmov\tax, WORD [_{}]\n", variable_name.0).as_str()),
                                        _ => unreachable!()
                                    }
                                },
                                ValueType::FunctionValue(function_call) => {
                                    code.push_str(pre_fn_args(function_call.clone(), &variables).as_str());
                                    code.push_str(format!("\t\tcall _{}_\n", function_call.name.0).as_str());
                                    code.push_str(post_fn_args(function_call.argc).as_str());
                                }
                            }
                            match bin_operation.operand_2 {
                                ValueType::Immediate(imm_value) => {
                                    match imm_value {
                                        Value::Byte(v) => code.push_str(format!("\t\tmov\tbl, {}\n", v).as_str()),
                                        Value::Word(v) => code.push_str(format!("\t\tmov\tbx, {}\n", v).as_str()),
                                        _ => unreachable!()
                                    };                                    
                                },
                                ValueType::Variable(variable_name) => {
                                    let size_op = get_size_variable(&variables, variable_name.clone());
                                    if size_op == Size::Byte && size == Size::Word {
                                        code.push_str("\t\tmov\trbx, 0\n");
                                    }
                                    match size_op {
                                        Size::Byte => code.push_str(format!("\t\tmov\tbl, BYTE [_{}]\n", variable_name.0).as_str()),
                                        Size::Word => code.push_str(format!("\t\tmov\tbx, WORD [_{}]\n", variable_name.0).as_str()),
                                        _ => unreachable!()
                                    }
                                },
                                ValueType::FunctionValue(function_call) => {
                                    code.push_str(pre_fn_args(function_call.clone(), &variables).as_str());
                                    code.push_str(format!("\t\tcall _{}_\n", function_call.name.0).as_str());
                                    code.push_str(post_fn_args(function_call.argc).as_str());
                                    code.push_str("\t\tmov\trbx, rax\n");
                                }
                            }
                            match bin_operation.op_type {
                                BinaryOpType::Addition       => code.push_str(format!("\t\tadd\t{}, {}\n", reg_op_1, reg_op_2).as_str()),
                                BinaryOpType::Substraction   => code.push_str(format!("\t\tsub\t{}, {}\n", reg_op_1, reg_op_2).as_str()),
                                BinaryOpType::Multiplication => code.push_str(format!("\t\tmul\t{}\n", reg_op_2).as_str()),
                                BinaryOpType::Division       => code.push_str(format!("\t\tdiv\t{}, {}\n", reg_op_1, reg_op_2).as_str())
                            }
                        },
                        Operation::Unary => ()
                    }
                },
            },
            _ => unreachable!(),
        }
    }
    code.push('\n');

    // exit syscall
    code.push_str("\t\t; Exit syscall\n");
    code.push_str("\t\tmov\trax, 0x3c\n");
    code.push_str("\t\tmov\trdi, 0\n");
    code.push_str("\t\tsyscall\n");

    code.push('\n');

    //print!("NASM\n{}", code);
    code
}


fn pre_fn_args(func_call: FunctionCall, vars_p: &Vec<Variable>) -> String {
    let mut code = String::new();
    for arg in func_call.args {
        match arg {
            ValueType::Immediate(val) => {
                match val {
                    Value::Byte(v) => code.push_str(format!("\t\tpush {}\n", v).as_str()),
                    Value::Word(v) => code.push_str(format!("\t\tpush {}\n", v).as_str()),
                    _ => unreachable!()
                }
            },
            ValueType::Variable(var_in) => {
                let var = get_variable(vars_p, var_in);
                match var.size {
                    Size::Byte => code.push_str(format!("\t\tmov  al, BYTE [_{}]\n", var.name.0).as_str()),
                    Size::Word => code.push_str(format!("\t\tmov  ax, WORD [_{}]\n", var.name.0).as_str()),
                    _ => unreachable!()
                }
                code.push_str("\t\tpush rax\n");
            },
            ValueType::FunctionValue(f_c) => {
                code.push_str(pre_fn_args(f_c.clone(), &vars_p.clone()).as_str());
                code.push_str(format!("\t\tcall _{}_\n", f_c.name.0).as_str());
                code.push_str(post_fn_args(f_c.argc).as_str());
                code.push_str("\t\tpush rax\n");
            }
        }
    };
    code
}

fn post_fn_args(argc: usize) -> String {
    format!("\t\tadd  rsp, 8 * {}\n", argc)
}


fn get_un_init_vars(vars: &Vec<Variable>) -> (Vec<Variable>, Vec<Variable>) {
    let mut u_vars: Vec<Variable> = Vec::new();
    let mut i_vars: Vec<Variable> = Vec::new();

    for var in vars {
        match var.init {
            Init::Initilized(_) => i_vars.push(var.clone()),
            Init::Uninitilized => u_vars.push(var.clone()),
        }
    }

    (u_vars, i_vars)
}

fn get_executable_instr(instrs: Vec<Instruction>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for instr in instrs {
        match instr {
            Instruction::Variable(_) => (),
            Instruction::Assignment(_) => instructions.push(instr.clone()),
        }
    }

    instructions
}

fn get_variable(vars: &Vec<Variable>, var_name: Indent) -> Variable {
    for var in vars {
        if var.name == var_name {
            return (*var).clone();
        }
    }
    unreachable!()
}

fn get_size_variable(vars: &Vec<Variable>, var_name: Indent) -> Size {
    for var in vars {
        if var.name == var_name {
            return (*var).size;
        }
    }
    unreachable!()
}

fn get_size_function(functions: &Vec<Function>, func_name: Indent) -> Size {
    for func in functions {
        if func.name == func_name {
            return (*func).ret_size;
        }
    }
    unreachable!()
}
