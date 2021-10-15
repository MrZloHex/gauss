use serde::{Serialize, Deserialize};

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
    args: Option<Vec<Argument>>,
    argc: usize,
    ret_size: Size,
    vars: Option<Vec<Variable>>,
    ret_var: Indent,
}); 

pub_struct!( Argument {
    name: Indent,
    size: Size,
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Indent(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Size {
    Byte,
    Word
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Value {
    Byte(u8),
    Word(u16)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Immediate(Value),
    FunctionValue(FunctionCall),
    Variable(Indent)
}

pub_struct!( FunctionCall {
    name: Indent,
    args: Option<Vec<Variable>>,
    argc: usize,
});


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Init {
    Initilized(Value),
    Uninitilized
}

pub_struct!( Variable {
    name: Indent,
    size: Size,
    init: Init,
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directive {
    Use(Vec<Indent>),
}

pub_struct!( Assignment {
    var_name: Indent,
    val: ValueType,
});


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    Variable(Variable),
    Assignment(Assignment)
}
