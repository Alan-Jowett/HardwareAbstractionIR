#![no_std]

#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "i2c")]
pub mod i2c;
#[cfg(feature = "interrupt")]
pub mod interrupt;
pub mod metadata;
#[cfg(feature = "spi")]
pub mod spi;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "uart")]
pub mod uart;
#[cfg(feature = "usart")]
pub mod usart;
