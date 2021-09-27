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
}); 















pub enum Operation {
    Store(Store),
    Set(Set)
}

pub_struct!( Store {
    loc: Location,
    value: u64,
    address: u64,
});
impl Default for Store {
    fn default() -> Self {
        Store {
            loc: Location::Heap,
            value: 0,
            address: 0
        }
    }
}

pub_struct!( Set {
    name: String,
    size: Size,
    value: u64,
});

#[derive(Clone)]
pub enum Size {
    Byte,
    Word
}

pub enum Location {
    Heap 
}

pub_struct!( Instruction {
    op: Operation,
});
impl Instruction {
    pub fn new(op: Operation) -> Self {
        Instruction {
            op
        }
    } 
}





pub enum Directive {
    Heap(u128),
    Uses(String)
}

pub_struct!( ProgrammInfo {
    heap_vol: u128,
});


