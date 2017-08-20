use openmpt_sys;
use std::ffi::CString;
use std::ffi::CStr;
use std::str;

#[derive(Debug)]
pub enum InfoField {
    LibraryVersion,
    LibraryFeatures,
	CoreVersion,
	SourceURL,
	SourceDate,
	Build,
	BuildCompiler,
	Credits,
	Contact,
	License,
	URL,
	SupportForumUrl,
	BugtrackerURL,
}

impl InfoField {
	fn to_str(&self) -> &str {
		use self::InfoField::*;
		match *self {
			LibraryVersion =>  "library_version",
			LibraryFeatures => "library_features",
			CoreVersion => "core_version",
			SourceURL => "source_url",
			SourceDate => "source_date",
			Build => "build",
			BuildCompiler => "build_compiler",
			Credits => "credits",
			Contact => "contact",
			License => "license",
			URL => "url",
			SupportForumUrl => "support_forum_url",
			BugtrackerURL => "bugtracker_url",
		}
	}
}

pub fn get_string (field : &InfoField) -> Option<String> {
	let cstr = CString::new(field.to_str()).unwrap();
	let cstr_ptr = cstr.as_ptr();
	let info_string = unsafe {
		// openmpt expects and returns utf-8 strings
		let return_ptr = openmpt_sys::openmpt_get_string(cstr_ptr);
		let return_str = CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
		openmpt_sys::openmpt_free_string(return_ptr);
		return_str
	};

	if info_string.len() == 0 {
		None
	} else {
		Some(info_string)
	}
}

pub fn get_supported_extensions() -> String {
	unsafe {
		// openmpt expects and returns utf-8 strings
		let return_ptr = openmpt_sys::openmpt_get_supported_extensions();
		let return_str = CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
		openmpt_sys::openmpt_free_string(return_ptr);
		return_str
	}
}

pub fn is_extension_supported(extension : &str) -> bool {
	let cstr = CString::new(extension).unwrap();
	let cstr_ptr = cstr.as_ptr();
	let result = unsafe {
		openmpt_sys::openmpt_is_extension_supported(cstr_ptr)
	};

	// Returns : 1 if the extension is supported by libopenmpt, 0 otherwise.
	if result == 1 { true } else { false }
}

#[cfg(test)]
mod tests {
	#[test]
	fn try_lib_version_field() {
		// Those should always return something
		test_info_field(&super::InfoField::LibraryVersion);
		test_info_field(&super::InfoField::LibraryFeatures);
		test_info_field(&super::InfoField::CoreVersion);
	}

	fn test_info_field(field : &super::InfoField) {
		let val = super::get_string(&field).unwrap();
		println!("Field {:?} : \"{}\"", &field, &val);		
	}

	#[test]
	fn supported_extensions_include_xm() {
		let support_list = super::get_supported_extensions();
		assert!(support_list.contains("xm"));
	}

	#[test]
	fn xm_is_supported() {
		assert!(super::is_extension_supported("xm"));
	}
}