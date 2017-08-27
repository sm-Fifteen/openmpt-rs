macro_rules! get_string {
	( $operation:expr ) => {
		unsafe {
			// openmpt expects and returns utf-8 strings
			let return_ptr = $operation;

			if return_ptr.is_null() {
				None::<String>
			} else {
				let return_str = ::std::ffi::CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
				openmpt_sys::openmpt_free_string(return_ptr);
				Some(return_str)
			}
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

macro_rules! with_2strings {
	( $string1:ident, $string2:ident, $operation:expr ) => {
		unsafe {
			let cstr1 = ::std::ffi::CString::new($string1).unwrap();
			let cstr2 = ::std::ffi::CString::new($string2).unwrap();
			let $string1 = cstr1.as_ptr();
			let $string2 = cstr2.as_ptr();
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

			if return_ptr.is_null() {
				None::<String>
			} else {
				let return_str = ::std::ffi::CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
				openmpt_sys::openmpt_free_string(return_ptr);
				Some(return_str)
			}
		}
	}
}