//! # The Rust Cool common lib.
//!

pub mod config;
pub mod clist;
pub mod ctracing;

pub fn init() {
    ctracing::init();
}
