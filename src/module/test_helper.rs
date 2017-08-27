use super::Module;
use super::Logger;
use std::fs::File;
use std::io::prelude::*;

pub fn load_file_as_module(file_path : &str) -> Result<Module, ()> {
	let mut f = File::open(file_path).expect("file not found");
	let mut buf = Vec::new();
	f.read_to_end(&mut buf);
	Module::create_from_memory(&mut buf, Logger::None, &[])
}