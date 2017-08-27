use openmpt_sys;
use std::os::raw::*;
use std::ptr;

mod ctls;
mod metadata;
mod mod_command;
mod iteration;
mod render;
#[cfg(test)] mod test_helper;

pub struct Module {
	inner : *mut openmpt_sys::openmpt_module,
}

impl Drop for Module {
	fn drop(&mut self) {
		unsafe {
			openmpt_sys::openmpt_module_destroy(self.inner);
		}
	}
}

impl Module {
	pub fn create_from_memory(stream : &mut Vec<u8>, logger : Logger, init_ctls : &[ctls::Ctl]) -> Result<Module, ()> {
		let module_ptr = unsafe {
			openmpt_sys::openmpt_module_create_from_memory(
				stream.as_ptr() as *const _,
				stream.len(),
				logger.log_func(),
				ptr::null_mut(), // user (As unsafe as it gets! Not touching this.)
				ptr::null() // init_ctls (Setting those manually below.)
			)
		};

		if module_ptr.is_null() {
			return Err(())
		}
		
		let mut module = Module { inner : module_ptr };

		// Set each init ctl by hand, lists of stucts of FFI string pointers are too much of a nightmare to deal with in Rust
		for init_ctl in init_ctls {
			module.enum_ctl_set(init_ctl);
		}

		Ok(module)
	}
}

pub enum CouldOpenEffort {
	NoEffort,
	ProbeFileHeader,
	VerifyHeader,
	LoadWithoutPatternOrPluginData,
	LoadCompleteModule,
}

impl CouldOpenEffort {
	fn value(&self) -> c_double {
		use self::CouldOpenEffort::*;

		match *self {
			NoEffort => 0.0,
			ProbeFileHeader => 0.1,
			VerifyHeader => 0.2,
			LoadWithoutPatternOrPluginData => 0.6,
			LoadCompleteModule => 1.0,
		}
	}
}

pub enum Logger {
    StdErr,
    None
}

impl Logger {
    pub(super) fn log_func(&self) -> openmpt_sys::openmpt_log_func {
        match *self {
            Logger::StdErr => Some(openmpt_sys::openmpt_log_func_default),
            Logger::None => Some(openmpt_sys::openmpt_log_func_silent)
        }
    }
}

pub fn could_open_propability (stream : &mut Vec<u8>, effort : CouldOpenEffort, logger : Logger) -> f64 {
	unsafe {
		openmpt_sys::openmpt_could_open_propability(openmpt_sys::openmpt_stream_callbacks::default(), stream.as_mut_ptr() as *mut _, effort.value(), logger.log_func(), ptr::null_mut())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::test_helper;

	#[test]
	fn empty_file_is_invalid() {
		let mut fake_file = Vec::new();
		let lazy_prob = could_open_propability(&mut fake_file, CouldOpenEffort::NoEffort, Logger::None);
		let probe_prob = could_open_propability(&mut fake_file, CouldOpenEffort::ProbeFileHeader, Logger::None);
		let header_prob = could_open_propability(&mut fake_file, CouldOpenEffort::VerifyHeader, Logger::None);
		let load_partial_prob = could_open_propability(&mut fake_file, CouldOpenEffort::LoadWithoutPatternOrPluginData, Logger::None);
		let load_complete_prob = could_open_propability(&mut fake_file, CouldOpenEffort::LoadCompleteModule, Logger::None);
		
		println!("Probability of opening an empty file (lazy/probe/verify_header/load_partial/load_complete) : {}/{}/{}/{}/{}",
			lazy_prob, probe_prob, header_prob, load_partial_prob, load_complete_prob);

		assert!(lazy_prob > 0.0);
		assert!(probe_prob < 0.5);
		assert!(header_prob == 0.0);
		assert!(load_partial_prob == 0.0);
		assert!(load_complete_prob == 0.0);
	}

	#[test]
	fn text_file_fails_to_load() {
		let module = test_helper::load_file_as_module("Cargo.toml");
		assert!(module.is_err());
	}

	#[test]
	fn dummy_file_loads_successfully() {
		let module = test_helper::load_file_as_module("empty_module.xm");
		assert!(module.is_ok());
	}
}