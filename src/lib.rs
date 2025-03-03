#[cfg(feature = "emulation")]
mod emulation;

#[cfg(feature = "emulation")]
pub use self::emulation::{Emulation, EmulationOS, EmulationOption};
