#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        //#[derive(Debug, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}


pub_struct!( Function {
    name: String,
    argv: Vec<(Size, String)>,
    argc: usize,
    ret_size: Size,
    loc_var: Vec<(Size, String)>,
    loc_var_c: usize,
    code: Vec<String>,
}); 

#[derive(Debug, Clone)]
pub enum Size {
    Byte,
    Word
}

