use openmpt_sys;
use std::os::raw::*;
use std::ptr;

pub mod ctls;
pub mod metadata;
pub mod mod_command;
pub mod iteration;
pub mod render;
pub mod stream;
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
	/// Construct an openmpt_module from a buffer.
	///
	/// ### Parameters
	/// * `buffer` : Buffer containing the data to load the module from.
	/// * `logger` : The logging function to use, from the `Logger` enum.
	/// * `init_ctls` : A list of initial ctl values, see the `ctls` module.
	///
	/// ### Returns
	/// The constructed openmpt_module, or None on failure.
	///
	/// ### Remarks
	/// The input data can be discarded after a Module has been constructed successfully.
	pub fn create_from_memory(buffer : &mut Vec<u8>, logger : Logger, init_ctls : &[ctls::Ctl]) -> Result<Module, ()> {
		let module_ptr = unsafe {
			openmpt_sys::openmpt_module_create_from_memory(
				buffer.as_ptr() as *const _,
				buffer.len(),
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

	/// Construct an openmpt_module from a stream.
	///
	/// ### Parameters
	/// * `stream` : Input stream to load the module from. Must implement the `Read` and possibly `Seek` trait.
	/// * `logger` : The logging function to use, from the `Logger` enum.
	/// * `init_ctls` : A list of initial ctl values, see the `ctls` module.
	///
	/// ### Returns
	/// The constructed openmpt_module, or None on failure.
	///
	/// ### Remarks
	/// The input data can be discarded after a Module has been constructed successfully.
	pub fn create<T : stream::ModuleStream>(stream : &mut T, logger : Logger, init_ctls : &[ctls::Ctl]) -> Result<Module, ()> {
		let stream_ptr:*mut T = stream;
		
		let module_ptr = unsafe {
			openmpt_sys::openmpt_module_create(
				T::get_file_callbacks(),
				stream_ptr as *mut _,
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

/// An enum containing the key effort values for `could_open_propability`
pub enum CouldOpenEffort {
	/// Does not even look at stream at all
	NoEffort,
	/// Only probe the header data of the module file
	ProbeFileHeader,
	/// Only verify the header data of the module file
	VerifyHeader,
	/// Loads the file from stream, but skips pattern and plugin data
	LoadWithoutPatternOrPluginData,
	/// Completely loads the file from stream
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

/// Roughly scan the input stream to find out whether libopenmpt might be able to open it.
///
/// ### Parameters
/// * `stream` : Input stream to scan. Must implement the `Read` trait.
/// * `effort` : Effort to make when validating stream, from the `CouldOpenEffort` enum.
/// * `logger` : The logging function to use, from the `Logger` enum.
///
/// ### Returns
/// Probability between 0.0 and 1.0.
///
/// ### Remarks
/// Can return any value between 0.0 and 1.0. Only 0.0 and 1.0 are definitive answers,
/// all values in between are just estimates. In general, any return value >0.0
/// means that you should try loading the file, and any value below 1.0 means that loading may fail.
/// If you want a threshold above which you can be reasonably sure that libopenmpt will be able to load the file,
/// use >=0.5. If you see the need for a threshold below which you could reasonably outright reject a file,
/// use <0.25 (Note: Such a threshold for rejecting on the lower end is not recommended,
/// but may be required for better integration into some other framework's probe scoring.).
pub fn could_open_propability<T : stream::ModuleStream> (stream : &mut T, effort : CouldOpenEffort, logger : Logger) -> f64 {
	let stream_ptr:*mut T = stream;

	unsafe {
		openmpt_sys::openmpt_could_open_propability(openmpt_sys::openmpt_stream_callbacks::default(), stream_ptr as *mut _, effort.value(), logger.log_func(), ptr::null_mut())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::test_helper;
	use std::io::Cursor;

	#[test]
	fn empty_file_is_invalid() {
		let mut fake_file:Vec<u8> = Vec::new();
		let mut fake_file = Cursor::new(fake_file);
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

	#[test]
	fn create_from_stream_doesnt_explode_sometimes() {
		let module = test_helper::stream_file_as_module("empty_module.xm");
		assert!(module.is_ok());
	}
}