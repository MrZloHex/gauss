#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone)] // ewww
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
    //loc_var: Vec<(Size, String)>,
    //loc_var_c: usize,
    //code: Vec<String>,
}); 

pub_struct!( Argument {
    name: Indent,
    size: Size,
});

#[derive(Debug, Clone)]
pub struct Indent(pub String);

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Byte,
    Word
}

#[derive(Debug, Clone, Copy)]
pub enum ValueSize {
    Byte(u8),
    Word(u16)
}



