#[cfg(feature = "impersonate")]
mod imp;

#[cfg(feature = "impersonate")]
pub use self::imp::{Impersonate, ImpersonateOS, ImpersonateOption};
