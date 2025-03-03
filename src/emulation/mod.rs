#[cfg(feature = "emulation")]
mod device;

#[cfg(feature = "emulation")]
pub use self::device::{Emulation, EmulationOS, EmulationOption};
