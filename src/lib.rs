#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]

//! A user friendly game engine written in Rust.

// Reexported crates.
extern crate input as input_lib;
extern crate event_loop as event_loop_lib;
extern crate window as window_lib;

pub use input_lib as input;
pub use event_loop_lib as event_loop;
pub use window_lib as window;
