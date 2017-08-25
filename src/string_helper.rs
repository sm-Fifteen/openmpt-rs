use openmpt_sys;

macro_rules! get_string {
	( $operation:expr ) => {
		unsafe {
			// openmpt expects and returns utf-8 strings
			let return_ptr = $operation;
			let return_str = ::std::ffi::CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
			openmpt_sys::openmpt_free_string(return_ptr);
			return_str
		}
	};
}

macro_rules! with_string {
	( $string:ident, $operation:expr ) => {
		unsafe {
			let cstr = ::std::ffi::CString::new($string).unwrap();
			let $string = cstr.as_ptr();
			$operation
		}
	}
}

macro_rules! get_string_with_string {
	( $string:ident, $operation:expr ) => {
		unsafe {
			let cstr = ::std::ffi::CString::new($string).unwrap();
			let $string = cstr.as_ptr();
			let return_ptr = $operation;
			let return_str = ::std::ffi::CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
			openmpt_sys::openmpt_free_string(return_ptr);
			return_str
		}
	}
}