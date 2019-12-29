use cuneiform::*;
use std::mem;

#[cuneiform(hermetic = true)]
pub struct Hermetic {
    data: u8,
    data_2: u16,
}

fn main() {
    #[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
    assert_eq!(mem::size_of::<Hermetic>(), 128);
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    assert_eq!(mem::size_of::<Hermetic>(), 128);
    #[cfg(target_os = "linux")]
    assert_eq!(mem::size_of::<Hermetic>(), 64);
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    assert_eq!(mem::size_of::<Hermetic>(), 64);
}
