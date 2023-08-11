//! # The Rust Cool common lib.
//!

pub mod command;
pub use command::*;
pub mod cconfig;
pub use cconfig::*;
pub mod clist;
pub mod ctracing;

pub fn init() {
    ctracing::init_by_config();
}
