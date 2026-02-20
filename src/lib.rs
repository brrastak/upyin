#![no_std]

pub mod yin;

pub use fixed::types::I24F8;

/// Default data type
pub type Sample = I24F8;

pub use yin::Yin;
