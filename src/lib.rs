#![no_std]
// #![deny(unsafe_code)]
// #![deny(warnings)]
// #![deny(missing_docs)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "log"))]
#[macro_use]
mod log;

pub use embedded_graphics::{
    self,
    pixelcolor::Rgb888,
    prelude::{Pixel, Size},
    DrawTarget,
};


pub mod gui;
pub mod component;
pub mod helper;