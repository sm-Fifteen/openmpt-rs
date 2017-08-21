use openmpt_sys;
use std::ops::Deref;
use std::os::raw::*;

mod logging;
mod ctls;
mod metadata;
#[cfg(test)] mod test_helper;

pub struct Module {
	inner : *mut openmpt_sys::openmpt_module,
}

impl Module {
	pub fn create_from_memory(stream : &mut Vec<u8>, logger : logging::Logger<()>, init_ctls : &[ctls::Ctl]) -> Result<Module, ()> {
		let module_ptr = unsafe {
				openmpt_sys::openmpt_module_create_from_memory(stream.as_ptr() as *const _, stream.len(), logger.log_func(),
						logger.logging_context(), ctls::to_initial_ctl_ptr(init_ctls))
		};

		if module_ptr.is_null() {
			Err(())
		} else {
			Ok(Module { inner : module_ptr })
		}
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		unsafe {
			openmpt_sys::openmpt_module_destroy(self.inner);
		}
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

pub fn could_open_propability (stream : &mut Vec<u8>, effort : CouldOpenEffort, logger : logging::Logger<()>) -> f64 {
	unsafe {
		openmpt_sys::openmpt_could_open_propability(openmpt_sys::openmpt_stream_callbacks::default(), stream.as_mut_ptr() as *mut _, effort.value(), logger.log_func(), logger.logging_context())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::test_helper;

	#[test]
	fn empty_file_is_invalid() {
		let mut fake_file = Vec::new();
		let lazy_prob = could_open_propability(&mut fake_file, CouldOpenEffort::NoEffort, logging::Logger::None);
		let probe_prob = could_open_propability(&mut fake_file, CouldOpenEffort::ProbeFileHeader, logging::Logger::None);
		let header_prob = could_open_propability(&mut fake_file, CouldOpenEffort::VerifyHeader, logging::Logger::None);
		let load_partial_prob = could_open_propability(&mut fake_file, CouldOpenEffort::LoadWithoutPatternOrPluginData, logging::Logger::None);
		let load_complete_prob = could_open_propability(&mut fake_file, CouldOpenEffort::LoadCompleteModule, logging::Logger::None);
		
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