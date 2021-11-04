use crate::types::*;

pub fn analyze_instr(
    instructions_p: &Vec<Instruction>,
    functions_p: &Vec<Function>,
) -> (bool, Vec<Variable>) {
    let variables = get_vars(instructions_p);
    let assignments = get_assign(instructions_p);

    // check for unique names of variables
    let mut tmp_is: Vec<Indent> = Vec::new();
    for variable in &variables {
        if !tmp_is.is_empty() {
            for tmp_i in &tmp_is {
                if *tmp_i == variable.name {
                    error(1, variable.name.clone());
                }
            }
        }
        tmp_is.push(variable.name.clone())
    }
    drop(tmp_is);

    // check for right size of variable
    for variable in &variables {
        match variable.init {
            Init::Initilized(value) => match value {
                Value::Byte(_) => {
                    if variable.size != Size::Byte {
                        error(0, variable)
                    }
                }
                Value::Word(_) => {
                    if variable.size != Size::Word {
                        error(0, variable)
                    }
                }
            },
            Init::Uninitilized => (),
        }
    }

    // check for assignment to existing and initilized variable
    for assignment in &assignments {
        if !is_variable(&variables, assignment.var_name.clone()) {
            error(3, assignment.var_name.clone())
        }
    }

    let (uninit_vars, _init_vars) = get_un_init_vars(&variables, &assignments);

    for assignment in &assignments {
        match &assignment.val {
            AssignValue::Value(val) => match val {
                ValueType::Variable(var_name) => {
                    if !is_variable(&variables, var_name.clone()) {
                        error(4, var_name.clone())
                    } else {
                        if is_variable(&uninit_vars, var_name.clone()) {
                            error(5, var_name.clone())
                        }
                    }
                },
                ValueType::FunctionValue(func_call) => {
                    if !is_correct_function_call(&func_call, functions_p, &variables) {
                        error(11, func_call);
                    }
                },
                _ => (),
            },
            AssignValue::Expression(op) => {
                match &op {
                    Operation::Binary(bin_op) => {
                        match &bin_op.operand_1 {
                            ValueType::Variable(var_name ) => {
                                if !is_variable(&variables, var_name.clone()) {
                                    error(4, var_name.clone())
                                } else {
                                    if is_variable(&uninit_vars, var_name.clone()) {
                                        error(5, var_name.clone())
                                    }
                                }
                            },
                            _ => (),
                        }
                        match &bin_op.operand_2 {
                            ValueType::Variable(var_name ) => {
                                if !is_variable(&variables, var_name.clone()) {
                                    error(4, var_name.clone())
                                } else {
                                    if is_variable(&uninit_vars, var_name.clone()) {
                                        error(5, var_name.clone())
                                    }
                                }
                            },
                            _ => (),
                        }
                    },
                    Operation::Unary => (),
                }
            }
        }
    }

    warn_uninit_vars(uninit_vars);

    // check for correct size of operands of assignment
    for assignment in &assignments {
        match &assignment.val {
            AssignValue::Value(value) => match value {
                ValueType::Immediate(val) => {
                    let size_var = get_size_var(&variables, assignment.var_name.clone());
                    match val {
                        Value::Byte(_) => {
                            if size_var == Size::Word {
                                // Ok
                            } else if size_var != Size::Byte {
                                error(2, assignment.var_name.clone())
                            }
                        }
                        Value::Word(_) => {
                            if size_var != Size::Word {
                                error(2, assignment.var_name.clone())
                            }
                        }
                    }
                }
                ValueType::Variable(var) => {
                    let size_var = get_size_var(&variables, assignment.var_name.clone());
                    let size_val = get_size_var(&variables, (*var).clone());
                    if size_var != size_val {
                        error(2, assignment.var_name.clone())
                    }
                }
                ValueType::FunctionValue(func_call) => {
                    let func_call_name = func_call.name.clone();
                    let mut is_such_func = false;
                    let mut argc: usize  = 0;
                    let mut args: Vec<Argument> = Vec::new();
                    let mut ret_size = Size::Byte;
                    for function in functions_p {
                        if function.name == func_call_name {
                            is_such_func = true;
                            argc = function.argc.clone();
                            args = function.args.clone();
                            ret_size = function.ret_size;
                        }
                    }

                    if is_such_func {
                        let size_var = get_size_var(&variables, assignment.var_name.clone());
                        if size_var != ret_size {
                            error(2, assignment.var_name.clone());
                        }
                        if argc != func_call.argc {
                            error(9, func_call_name)
                        }
                        for (f_arg, arg) in args.iter().zip(func_call.args.iter()) {
                            let size = f_arg.size;
                            match arg {
                                ValueType::Immediate(val) => {
                                    match val {
                                        Value::Byte(_) => (), //OK  //if size != Size::Byte { error(10, f_arg.name.clone()) },
                                        Value::Word(_) => if size != Size::Word { error(10, f_arg.name.clone()) }
                                    }
                                },
                                ValueType::Variable(var) => {
                                    let var_size = get_size_var(&variables, (*var).clone());
                                    if var_size != size {
                                        error(10, f_arg.name.clone());
                                    }
                                },
                                ValueType::FunctionValue(fn_c) => {
                                    let f_size = get_size_function(functions_p, fn_c.name.clone());
                                    if f_size != size {
                                        error(10, f_arg.name.clone());
                                    }
                                }
                            }
                        }   
                    } else {
                        error(8, func_call_name);
                    }
                }
            },
            AssignValue::Expression(op) => {
                match &op {
                    Operation::Binary(_bin_op) => {
                         
                    },
                    Operation::Unary => unreachable!()
                }
            }
        }
    }

    (true, variables)
}

// pub fn analyze_direct(instructions: Vec<Instruction>, directives: &Vec<Directive>) -> Vec<Instruction> {
//
//     let mut instrs: Vec<Instruction> = instructions;
//
//     for directive in directives {
//         match directive {
//             Directive::Set(set) => {
//                 let indent = set.name.clone();
//                 for mut instr in &instrs {
//                     let instr_new: Instruction;
//                     match instr {
//                         Instruction::Variable(var) => {
//                             match var.init {
//                                 Init::Initilized(val) => {
//                                     ()
//                                     // TODO: MAKE INITILIAZATION BY ANOTHER VARIABLE
//                                 },
//                                 _ => ()
//                             }
//                         },
//                         Instruction::Assignment(ass) => {
//                             match &ass.val {
//                                 ValueType::Variable(indent_var) => {
//                                     if indent_var == indent {
//                                         instr_new = Instruction::Assignment
//                                 },
//                                 _ => ()
//                             }
//                         }
//                     }
//                 }
//             },
//             Directive::Use(_indents) => ()
//         }
//     }
//
//     instrs
// }

pub fn analyze_func(functions: &Vec<Function>) -> bool {
    'func: for function in functions {
        let mut checked = false;
        if !function.vars.is_empty() {
            for var in function.vars.clone() {
                if var.name.0 == function.ret_var.0 {
                    if var.size == function.ret_size {
                        checked = true;
                    } else {
                        error(6, (*function).name.0.clone())
                    }
                }

                if checked {
                    continue 'func;
                }
            }
        } else {
            for arg in function.args.clone() {
                if arg.name.0 == function.ret_var.0 {
                    if arg.size == function.ret_size {
                        checked = true;
                    } else {
                        error(6, (*function).name.0.clone())
                    }
                }

                if checked {
                    continue 'func;
                }
            }
        }

        if !checked {
            for arg in function.args.clone() {
                if arg.name.0 == function.ret_var.0 {
                    if arg.size == function.ret_size {
                        checked = true;
                    } else {
                        error(6, (*function).name.0.clone())
                    }
                }

                if checked {
                    continue 'func;
                }
            }
            if !checked {
                error(7, (*function).name.0.clone())
            }
        }
    }
    true
}

/*  Error codes:
 *
 *  - 0:  Size of variable not corresponds to it's value
 *  - 1:  Variable name is already used
 *  - 2:  Assignment to different sizes
 *  - 3:  ASsignment to undeclared variable
 *  - 4:  Assigning value of undeclared variable
 *  - 5:  Assigning value of uninitilized variable
 *  - 6:  Returning variable with incorrect size
 *  - 7:  Returning undeclared variable
 *  - 8:  Calling undeclared function
 *  - 9:  Incorrect quantity of provided arguments
 *  - 10: Incorrect argument size
 *  - 11: Incorrect function call
 *
 */

fn error<T>(error_code: u8, problem_struct: T) -> !
where
    T: std::fmt::Debug,
{
    eprint!("ERROR: ");
    match error_code {
        0 => eprintln!(
            "Size of variable not corresponds to its value,\nsee variable {:?}",
            problem_struct
        ),
        1 => eprintln!("Variable name `{:?}` is already used", problem_struct),
        2 => eprintln!("Assigning to `{:?}` diferent size value", problem_struct),
        3 => eprintln!("Assigning to undeclared variable `{:?}`", problem_struct),
        4 => eprintln!(
            "Assigning value of undeclared variable `{:?}`",
            problem_struct
        ),
        5 => eprintln!(
            "Assigning value of uninitilized variable `{:?}`",
            problem_struct
        ),
        6 => eprintln!(
            "Returning variable with incorrect size at function `{:?}`",
            problem_struct
        ),
        7 => eprintln!("Returning undeclared variable at `{:?}`", problem_struct),
        8 => eprintln!("Calling undeclared function `{:?}`", problem_struct),
        9 => eprintln!("Incorrect quantity of provided arguments at `{:?}`", problem_struct),
        10 => eprintln!("Incorrect argument size `{:?}`", problem_struct),
        11 => eprintln!("Incorrect function call `{:?}`", problem_struct),
        _ => unreachable!(),
    }
    std::process::exit(1);
}

fn warn_uninit_vars(vars: Vec<Variable>) {
    for var in vars {
        println!("WARNING: Uninitilized variable `{}`", var.name.0)
    }
}

fn get_vars(instructions_p: &Vec<Instruction>) -> Vec<Variable> {
    let mut variables: Vec<Variable> = Vec::new();
    for instruction in instructions_p {
        if matches!(instruction, Instruction::Variable(_)) {
            let variable = if let Instruction::Variable(var) = instruction {
                var
            } else {
                unreachable!()
            };
            variables.push((*variable).clone());
        }
    }
    variables
}

fn get_assign(instructions_p: &Vec<Instruction>) -> Vec<Assignment> {
    let mut assignments: Vec<Assignment> = Vec::new();
    for instruction in instructions_p {
        if matches!(instruction, Instruction::Assignment(_)) {
            let assignment = if let Instruction::Assignment(ass) = instruction {
                ass
            } else {
                unreachable!()
            };
            assignments.push((*assignment).clone())
        }
    }
    assignments
}

fn get_size_var(vars: &Vec<Variable>, var_name: Indent) -> Size {
    for var in vars {
        if var.name == var_name {
            return var.size.clone();
        }
    }
    unreachable!();
}

fn get_size_function(funcs: &Vec<Function>, func_name: Indent) -> Size {
    for func in funcs {
        if func.name == func_name {
            return func.ret_size.clone();
        }
    }
    unreachable!();
}

fn get_un_init_vars(
    vars: &Vec<Variable>,
    assigns: &Vec<Assignment>,
) -> (Vec<Variable>, Vec<Variable>) {
    let mut uninit_vars: Vec<Variable> = Vec::new();
    let mut init_vars: Vec<Variable> = Vec::new();
    for var in vars {
        match var.init {
            Init::Uninitilized => {
                let mut init = false;
                for assign in assigns {
                    if assign.var_name == var.name {
                        init = true;
                    }
                }
                if !init {
                    uninit_vars.push(var.clone())
                } else {
                    init_vars.push(var.clone())
                }
            }
            _ => (),
        }
    }
    (uninit_vars, init_vars)
}

fn is_variable(vars: &Vec<Variable>, var_name: Indent) -> bool {
    let mut res = false;
    for var in vars {
        if var.name == var_name {
            res = true;
        }
    }
    res
}

fn is_correct_function_call(function_call: &FunctionCall, functions: &Vec<Function>, variables: &Vec<Variable>) -> bool {
    let mut is_function: bool = false;
    let mut function = Function {
        name: Indent("QWE".to_string()),
        args: Vec::new(),
        argc: 0,
        ret_size: Size::Byte,
        vars: Vec::new(),
        ret_var: Indent("QWE".to_string())
    };
    for func in functions {
        if func.name == function_call.name {
            is_function = true;
            function = (*func).clone();
            break;
        }
    }
    if is_function {
        if function.argc != function_call.argc {
            error(9, function_call)
        } else {
            for (f_arg, arg) in function.args.iter().zip(function_call.args.iter()) {
                let size = f_arg.size;
                match arg {
                    ValueType::Immediate(val) => {
                        match val {
                            Value::Byte(_) => (), //OK  //if size != Size::Byte { error(10, f_arg.name.clone()) },
                            Value::Word(_) => if size != Size::Word { error(10, f_arg.name.clone()) }
                        }
                    },
                    ValueType::Variable(var) => {
                        let var_size = get_size_var(&variables, (*var).clone());
                        if var_size != size {
                            error(10, f_arg.name.clone());
                        }
                    },
                    ValueType::FunctionValue(fn_c) => {
                        if !is_correct_function_call(&fn_c, functions, variables) {
                            error(11, function_call)
                        }
                        let f_size = get_size_function(functions, fn_c.name.clone());
                        if f_size != size {
                            error(10, f_arg.name.clone());
                        }
                    }
                }
            };
            return true;
        }
    } else {
        error(8, function_call.name.0.clone())
    }
}