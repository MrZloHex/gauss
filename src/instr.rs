pub enum Operation {
    Store
}

pub enum Location {
    Memory
}

pub enum Instruction {
    Operation,
    Memory
}




pub enum Directive {
    Heap(u128)
}

pub struct ProgrammInfo {
    pub heap_vol: u128
}
