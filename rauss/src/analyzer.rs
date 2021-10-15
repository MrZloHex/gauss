use crate::types::*;


pub fn analyze_instr(instructions_p: &Vec<Instruction>) -> bool {
    let variables = get_vars(instructions_p);

    for instruction in instructions_p {
        match instruction {
            Instruction::Variable(variable) => {
                match variable.init {
                    Init::Initilized(value) => {
                        match value {
                            Value::Byte(_) => if variable.size != Size::Byte { error(0, variable) },
                            Value::Word(_) => if variable.size != Size::Word { error(0, variable) }
                        }
                    },
                    Init::Uninitilized => ()
                }
            },
            Instruction::Assignment(assignment) => {
                
            }
        }
    }
    true
}


/*  Error code:
 *
 *  - 0: Size of variable not corresponds to it's value
 *
 */

fn error<T>(error_code: u8, problem_struct: T) -> ! 
where T: std::fmt::Debug
{
    match error_code {
        0 => eprintln!("Size of variable not corresponds to its value,\nsee variable {:?}", problem_struct),
        _ => unreachable!()
    }
    std::process::exit(1);
}


fn get_vars(instructions_p: &Vec<Instruction>) -> Vec<Variable> {
    let mut variables: Vec<Variable> = Vec::new();
    for instruction in instructions_p {
        if matches!(instruction, Instruction::Variable(_)) {
            let variable = if let Instruction::Variable(var) = instruction { var } else { unreachable!() };
            variables.push((*variable).clone());
        }
    }
    variables
}

