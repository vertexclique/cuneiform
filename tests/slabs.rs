use cuneiform::*;
use std::mem;

#[cuneiform(slab = "powerpc_mpc8xx")]
pub struct Varying {
    data: u8,
    data_2: u16,
}

fn main() {
    #[cfg(not(target_os = "linux"))]
    assert_eq!(mem::size_of::<Varying>(), 16);
    #[cfg(target_os = "linux")]
    assert_eq!(mem::size_of::<Varying>(), 64);
}
