#![no_std]

pub mod yin;

pub use fugit::{HertzU32 as Hertz, MicrosDurationU32 as MicroSecond, ExtU32, RateExtU32};

pub use yin::{Yin, YinCalculator, Sample, Candidates};
