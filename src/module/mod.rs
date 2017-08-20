use openmpt_sys;
use std::os::raw::*;
use std::ptr;

mod logging;

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

pub fn could_open_propability (stream : &mut Vec<u8>, effort : CouldOpenEffort, logger : logging::Logger) -> f64 {
	unsafe {
		openmpt_sys::openmpt_could_open_propability(openmpt_sys::openmpt_stream_callbacks::default(), stream.as_mut_ptr() as *mut _, effort.value(), logger.log_func(), ptr::null_mut())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn empty_file_is_invalid() {
		let mut fake_file = Vec::new();
		let lazy_prob = super::could_open_propability(&mut fake_file, super::CouldOpenEffort::NoEffort, super::logging::Logger::None);
		let probe_prob = super::could_open_propability(&mut fake_file, super::CouldOpenEffort::ProbeFileHeader, super::logging::Logger::None);
		let header_prob = super::could_open_propability(&mut fake_file, super::CouldOpenEffort::VerifyHeader, super::logging::Logger::None);
		let load_partial_prob = super::could_open_propability(&mut fake_file, super::CouldOpenEffort::LoadWithoutPatternOrPluginData, super::logging::Logger::None);
		let load_complete_prob = super::could_open_propability(&mut fake_file, super::CouldOpenEffort::LoadCompleteModule, super::logging::Logger::None);
		
		println!("Probability of opening an empty file (lazy/probe/verify_header/load_partial/load_complete) : {}/{}/{}/{}/{}",
			lazy_prob, probe_prob, header_prob, load_partial_prob, load_complete_prob);

		assert!(lazy_prob > 0.0);
		assert!(probe_prob < 0.5);
		assert!(header_prob == 0.0);
		assert!(load_partial_prob == 0.0);
		assert!(load_complete_prob == 0.0);
	}
}