use super::Module;
use super::Logger;
use super::ctls::Ctl;
use std::fs::File;
use std::io::prelude::*;

pub fn load_file_as_module(file_path : &str) -> Result<Module, ()> {
	load_file_as_module_with_ctls(file_path, Logger::None, &[])
}

pub fn load_file_as_module_with_ctls(file_path: &str, logger: Logger, init_ctls: &[Ctl]) -> Result<Module, ()> {
	let mut f = File::open(file_path).expect("unable to open file");
	let mut buf = Vec::new();
	f.read_to_end(&mut buf).expect("failed to read file completely");
	Module::create_from_memory(&mut buf, logger, init_ctls)
}

pub fn stream_file_as_module(file_path : &str) -> Result<Module, ()> {
	stream_file_as_module_with_ctls(file_path, Logger::None, &[])
}

pub fn stream_file_as_module_with_ctls(file_path: &str, logger: Logger, init_ctls: &[Ctl]) -> Result<Module, ()> {
	let mut stream = ::std::fs::File::open(file_path).expect("unable to open file");

	Module::create(&mut stream, logger, init_ctls)
}
