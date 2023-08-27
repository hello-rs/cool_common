//! # The Rust Cool common lib.
//!

pub mod command;
pub use command::*;
pub mod cconfig;
pub use cconfig::*;
pub mod cencoding;
pub mod clist;
pub mod crand;
pub mod ctime;
pub mod ctracing;

pub fn init() {
    ctracing::init_by_config();
}
