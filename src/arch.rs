#[cfg(feature = "bsp_rpi3")]
mod aarch64;

#[cfg(feature = "bsp_rpi3")]
pub use aarch64::*;
