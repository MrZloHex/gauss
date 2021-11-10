use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, Serialize, Deserialize)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!( Function {
    name: Indent,
    args: Vec<Argument>,
    argc: usize,
    ret_size: Size,
    vars: Vec<Variable>,
    ret_var: Indent,
});

pub_struct!(Argument {
    name: Indent,
    size: Size,
});

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Indent(pub String);

pub_struct!( FunctionCall {
    name: Indent,
    args: Vec<ValueType>,
    argc: usize,
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directive {
    Use(Vec<Indent>),
    Args((Indent, Indent)),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Size {
    Byte,
    Word,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Value {
    Byte(u8),
    Word(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Immediate(Value),
    FunctionValue(FunctionCall),
    Variable(Indent),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Init {
    Initilized(Value),
    Uninitilized,
}

pub_struct!(Variable {
    name: Indent,
    size: Size,
    init: Init,
});

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BinaryOpType {
    Addition,
    Substraction,
    Multiplication,
    Division
}

pub_struct!( BinaryOperation {
    op_type: BinaryOpType,
    operand_1: ValueType,
    operand_2: ValueType,
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Binary(BinaryOperation),
    Unary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignValue {
    Value(ValueType),
    Expression(Operation)
}

pub_struct!(Assignment {
    var_name: Indent,
    val: AssignValue,
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    Variable(Variable),
    Assignment(Assignment),
}
