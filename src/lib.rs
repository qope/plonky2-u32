#![allow(clippy::needless_range_loop)]
#![no_std]

extern crate alloc;

pub mod gadgets;
pub mod gates;
pub mod serialization;
pub mod witness;

pub const fn ceil_div_usize(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}
