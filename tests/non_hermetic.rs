use cuneiform::*;
use std::mem;

#[cuneiform(hermetic = false)]
pub struct NonHermetic {
    data: u8,
    data_2: u16,
}

fn main() {
    #[cfg(not(target_os = "windows"))]
    assert_eq!(mem::size_of::<NonHermetic>(), 128);
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    assert_eq!(mem::size_of::<NonHermetic>(), 64);
}
