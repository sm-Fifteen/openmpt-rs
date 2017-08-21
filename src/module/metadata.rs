use super::Module;
use openmpt_sys;
use std::ffi::CString;
use std::ffi::CStr;

#[derive(Debug)]
pub enum MetadataKey {
	TypeExt,
	TypeName,
	ContainerExt,
	ContainerName,
	ModuleTitle,
	ModuleArtist,
	ModuleTracker,
	ModuleSaveDate,
	SongMessage,
	SongMessageOrInstruments,
	LoadWarnings,
}

impl MetadataKey {
	fn to_str(&self) -> &str {
		use self::MetadataKey::*;
		match *self {
			TypeExt => "type",
			TypeName => "type_long",
			ContainerExt => "container",
			ContainerName => "container_long",
			ModuleTitle => "title",
			ModuleArtist => "artist",
			ModuleTracker => "tracker",
			ModuleSaveDate => "date",
			SongMessage => "message",
			SongMessageOrInstruments => "message_raw",
			LoadWarnings => "warnings",
		}
	}
}

impl Module {
	pub fn get_metadata(&self, key : MetadataKey) -> Option<String> {
		let cstr = CString::new(key.to_str()).unwrap();
		let cstr_ptr = cstr.as_ptr();
		let value_string = unsafe {
			// openmpt expects and returns utf-8 strings
			let return_ptr = openmpt_sys::openmpt_module_get_metadata(self.inner, cstr_ptr);
			let return_str = CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
			openmpt_sys::openmpt_free_string(return_ptr);
			return_str
		};

		if value_string.len() == 0 {
			None
		} else {
			Some(value_string)
		}
	}

	fn get_metadata_keys(&self) -> String {
		unsafe {
			let return_ptr = openmpt_sys::openmpt_module_get_metadata_keys(self.inner);
			let return_str = CStr::from_ptr(return_ptr).to_string_lossy().into_owned();
			openmpt_sys::openmpt_free_string(return_ptr);
			return_str
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::test_helper;

	#[test]
	fn dummy_file_is_xm() {
		let module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		assert_eq!(module.get_metadata(MetadataKey::TypeExt).unwrap(), "xm");
		assert_eq!(module.get_metadata(MetadataKey::TypeName).unwrap(), "FastTracker II");
	}

	#[test]
	fn all_enum_values_are_supported() {
		let module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		let keys = module.get_metadata_keys();
		
		assert!(keys.contains(MetadataKey::TypeExt.to_str()));
		assert!(keys.contains(MetadataKey::TypeName.to_str()));
		assert!(keys.contains(MetadataKey::ContainerExt.to_str()));
		assert!(keys.contains(MetadataKey::ContainerName.to_str()));
		assert!(keys.contains(MetadataKey::ModuleTitle.to_str()));
		assert!(keys.contains(MetadataKey::ModuleArtist.to_str()));
		assert!(keys.contains(MetadataKey::ModuleTracker.to_str()));
		assert!(keys.contains(MetadataKey::ModuleSaveDate.to_str()));
		assert!(keys.contains(MetadataKey::SongMessage.to_str()));
		assert!(keys.contains(MetadataKey::SongMessageOrInstruments.to_str()));
		assert!(keys.contains(MetadataKey::LoadWarnings.to_str()));
	}
}