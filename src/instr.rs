#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        //#[derive(Debug, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}


pub enum Operation {
    Store(Store)
}

pub_struct!( Store {
    loc: Location,
    value: u128,
    address: u128,
});
impl Default for Store {
    fn default() -> Self {
        Store {
            loc: Location::Memory,
            value: 0,
            address: 0
        }
    }
}

pub enum Location {
    Memory
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
    Heap(u128)
}

pub_struct!( ProgrammInfo {
    heap_vol: u128,
});


