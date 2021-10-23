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
        };
        code.push_str(format!("\t{}_:\t{} {}\n", u_var.name.0, size, quantity).as_str());
    }
    code.push('\n');

    // section .data
    code.push_str("SECTION .data\n");
    for i_var in i_vars {
        let size = match i_var.size {
            Size::Byte => "db",
            Size::Word => "dw",
        };
        let value = if let Init::Initilized(val) = i_var.init {
            match val {
                Value::Byte(v) => v as u16,
                Value::Word(v) => v,
            }
        } else {
            unreachable!()
        };
        code.push_str(format!("\t{}_:\t{} {}\n", i_var.name.0, size, value).as_str());
    }
    code.push('\n');

    // section .text
    code.push_str("SECTION .text\n");

    // functions
    for function in functions {
        let mut vars_offset: HashMap<Indent, u64> = HashMap::new();
        let mut vars_p: u64 = 0;

        code.push_str(format!("\t{}_:\n", function.name.0).as_str());
        code.push_str("\t\tpush rbp\n");
        code.push_str("\t\tmov  rbp,\trsp\n");

        let mut size_loc_vars: u64 = 0;
        if !function.vars.is_empty() {
            for var in function.vars.clone() {
                match var.size {
                    Size::Byte => size_loc_vars += 1,
                    Size::Word => size_loc_vars += 2,
                }
            }
        }
        if size_loc_vars != 0 {
            code.push_str(format!("\t\tsub  rsp,\t{}\n", size_loc_vars).as_str());
        }

        // MAIN
        for var in function.vars {
            // add to vars_offset
            match var.size {
                Size::Byte => {
                    vars_offset.insert(var.name.clone(), vars_p+1);
                    vars_p += 1;
                },
                Size::Word => {
                    vars_offset.insert(var.name.clone(), vars_p+2);
                    vars_p += 2;
                }
            }
            match var.init {
                Init::Initilized(value) => {
                    let offset = vars_offset.get(&var.name).unwrap();
                    match value {
                        Value::Byte(val) => code.push_str(format!("\t\tmov  BYTE [rbp-{}],\t{}\n", offset, val).as_str()),
                        Value::Word(val) => code.push_str(format!("\t\tmov  WORD [rbp-{}],\t{}\n", offset, val).as_str()),
                    }
                },
                _ => (),
            }
        }

        // RET
        code.push_str("\t\tmov  rax,\t 0\n");
        let offset = vars_offset.get(&function.ret_var).unwrap();
        match function.ret_size {
            Size::Byte => code.push_str(format!("\t\tmov  al,\tBYTE [rbp-{}]\n", offset).as_str()),
            Size::Word => code.push_str(format!("\t\tmov  ax,\tWORD [rbp-{}]\n", offset).as_str())
        }


        if size_loc_vars == 0 {
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
                ValueType::Immediate(val) => {
                    let (value, size) = match val {
                        Value::Byte(v) => (v as u16, "BYTE"),
                        Value::Word(v) => (v, "WORD"),
                    };
                    code.push_str(
                        format!("\t\tmov\t{} [{}_], {}\n", size, assign.var_name.0, value).as_str(),
                    )
                }
                ValueType::Variable(var_name) => {
                    let var = get_variable(&variables, var_name.clone());
                    let (reg, size) = match var.size {
                        Size::Byte => ("al", "BYTE"),
                        Size::Word => ("ax", "WORD"),
                    };
                    code.push_str(
                        format!("\t\tmov\t{}, {} [{}_]\n", reg, size, var_name.0).as_str(),
                    );
                    code.push_str(
                        format!("\t\tmov\t{} [{}_], {}\n", size, assign.var_name.0, reg).as_str(),
                    )
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    code.push('\n');

    // exi t syscall
    code.push_str("\t\tmov\trax, 0x3c\n");
    code.push_str("\t\tmov\trdi, 0\n");
    code.push_str("\t\tsyscall\n");

    code.push('\n');

    // print!("NASM\n{}", code);
    code
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
