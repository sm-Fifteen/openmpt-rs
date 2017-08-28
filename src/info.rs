//! # Info
//! Get library-related metadata

use openmpt_sys;
use std::str;

#[derive(Debug)]
/// A struct containing the OpenMPT version number, in big-endian.
///
/// `CoreVersion(majormajor, major, minor, minorminor)`
pub struct CoreVersion (u8,	u8,	u8,	u8);

#[derive(Debug)]
/// A struct containing the libopenmpt version number, in big-endian.
///
/// `LibraryVersion(major, minor, revision)`
pub struct LibraryVersion (u8, u8, u16);

#[derive(Debug)]
/// An enum containing all the potentially valid keys for `openmpt_get_string`
pub enum InfoField {
	/// Verbose library version string
    LibraryVersion,
	/// Verbose library features string
    LibraryFeatures,
	/// Verbose OpenMPT core version string
	CoreVersion,
	/// Original source code URL
	SourceURL,
	/// Original source code date
	SourceDate,
	/// Information about the current build (e.g. the build date or compiler used)
	Build,
	/// Information about the compiler used to build libopenmpt
	BuildCompiler,
	/// All contributors
	Credits,
	/// Contact information about libopenmpt
	Contact,
	/// The libopenmpt license
	License,
	/// libopenmpt website URL
	URL,
	/// libopenmpt support and discussions forum URL
	SupportForumUrl,
	/// libopenmpt bug and issue tracker URL
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

/// Get a library-related metadata string.
///
/// ### Parameters
/// * `field` : Key to query, from the `InfoField` enum.
///
/// ### Returns
/// A (possibly multi-line) string containing the queried information.
/// If no information is available, the string is empty.
pub fn get_string (field : &InfoField) -> Option<String> {
	let field = field.to_str();
	get_string_with_string!(field, {
		openmpt_sys::openmpt_get_string(field)
	})
}

/// Get a list of supported file extensions.
///
/// ### Returns
/// The semicolon-separated list of extensions supported by
/// the libopenmpt build. The extensions are returned lower-case
/// without a leading dot.
pub fn get_supported_extensions() -> String {
	let opt_string = get_string!{
		openmpt_sys::openmpt_get_supported_extensions()
	};

	opt_string.expect("Got null pointer instead of string")
}

/// Query whether a file extension is supported.
///
/// ### Parameters
/// * `extension` : File extension to query without a leading dot. Case-insensitive.
///
/// ### Returns
/// Whether a module tracker format with that extension is supported or not.
pub fn is_extension_supported(extension : &str) -> bool {
	let result = with_string!(extension, {
		openmpt_sys::openmpt_is_extension_supported(extension)
	});

	// Returns : 1 if the extension is supported by libopenmpt, 0 otherwise.
	if result == 1 { true } else { false }
}

/// Get the OpenMPT core version number.
///
/// ### Returns
/// A struct containing the OpenMPT version number, in big-endian :
///
/// `CoreVersion(majormajor, major, minor, minorminor)`
pub fn get_core_version () -> CoreVersion {
	let version_number = unsafe {
		openmpt_sys::openmpt_get_core_version()
	};

	CoreVersion (
		(version_number >> 24) as u8,
		(version_number >> 16) as u8,
		(version_number >> 8)  as u8,
		version_number  as u8
	)
}

/// Get the libopenmpt version number.
///
/// ### Returns
/// A struct containing the libopenmpt version number, in big-endian :
///
/// `LibraryVersion(major, minor, revision)`
pub fn get_library_version () -> LibraryVersion {
	let version_number = unsafe {
		openmpt_sys::openmpt_get_library_version()
	};

	LibraryVersion (
		(version_number >> 24) as u8,
		(version_number >> 16)  as u8,
		version_number as u16
	)
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

	#[test]
	fn show_version_numbers() {
		let x = super::get_core_version();
		println!("Core Version : {:?}", &x);
		let y = super::get_library_version();
		println!("Lib Version : {:?}", &y);
	}
}