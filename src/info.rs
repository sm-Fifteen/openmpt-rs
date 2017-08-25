use openmpt_sys;
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
	let field = field.to_str();
	let info_string = get_string_with_string!(field, {
		openmpt_sys::openmpt_get_string(field)
	});

	if info_string.len() == 0 {
		None
	} else {
		Some(info_string)
	}
}

pub fn get_supported_extensions() -> String {
	get_string!{
		openmpt_sys::openmpt_get_supported_extensions()
	}
}

pub fn is_extension_supported(extension : &str) -> bool {
	let result = with_string!(extension, {
		openmpt_sys::openmpt_is_extension_supported(extension)
	});

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