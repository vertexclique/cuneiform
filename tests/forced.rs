use cuneiform::*;
use std::mem;

#[cuneiform(force = 64)]
pub struct Forced {
    data: u8,
    data_2: u16,
}

fn main() {
    assert_eq!(mem::size_of::<Forced>(), 64);
}
