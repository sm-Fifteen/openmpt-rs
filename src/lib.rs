//! Safe rust bindings for libopenmpt, built on top of the C API.
//!
//! See openmpt_sys for the unsafe bindings.

extern crate openmpt_sys;

#[macro_use] mod string_helper;
pub mod info;
pub mod mod_command;
pub mod module;