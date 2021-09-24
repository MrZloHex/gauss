pub enum Operation {
    Store
}

pub enum Location {
    Memory
}

pub struct Instruction {
    pub op: Operation,
    pub loc: Location
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            op: Operation::Store,
            loc: Location::Memory
        }
    }
}





pub enum Directive {
    Heap(u128)
}

pub struct ProgrammInfo {
    pub heap_vol: u128
}
