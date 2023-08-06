//! # The Rust Cool common lib.
//!

pub mod command;
pub use command::*;
pub mod config;
pub use config::*;
pub mod clist;
pub mod ctracing;

pub fn init() {
    ctracing::init_by_config();
}
