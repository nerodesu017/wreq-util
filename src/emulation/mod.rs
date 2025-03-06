mod device;
#[cfg(feature = "emulation-rand")]
mod rand;

pub use self::device::{Emulation, EmulationOS, EmulationOption};
