use super::Module;
use openmpt_sys;

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
		let key = key.to_str();
		get_string_with_string! (key, {
			openmpt_sys::openmpt_module_get_metadata(self.inner, key)
		})
	}

	fn get_metadata_keys(&self) -> String {
		let opt_string = get_string! {
			openmpt_sys::openmpt_module_get_metadata_keys(self.inner)
		};

		opt_string.expect("Got null pointer instead of string")
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