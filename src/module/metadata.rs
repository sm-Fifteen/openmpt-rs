//! Definitions for all types and methods used query module metadata

use super::Module;
use openmpt_sys;

#[derive(Debug)]
pub enum MetadataKey {
	/// Module format extension (e.g. it)
	TypeExt,
	/// Tracker name associated with the module format (e.g. Impulse Tracker)
	TypeName,
	/// Container format the module file is embedded in, if any (e.g. umx)
	ContainerExt,
	/// Full container name if the module is embedded in a container (e.g. Unreal Music)
	ContainerName,
	/// Module title
	ModuleTitle,
	/// Author of the module
	ModuleArtist,
	/// Tracker that was (most likely) used to save the module file, if known
	ModuleTracker,
	/// Date the module was last saved, in ISO-8601 format.
	ModuleSaveDate,
	/// Song message. If the song message is empty or the module format does not support song messages, an empty string is returned.
	SongMessage,
	/// Song message. If the song message is empty or the module format does not support song messages, a list of instrument and sample names is returned instead.
	SongMessageOrInstruments,
	/// A list of warnings that were generated while loading the module.
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
	/// Get a metadata item value.
	///
	/// ### Parameters
	/// * `key` : Metadata item key to query, from the `MetadataKey` enum.
	///
	/// ### Returns
	/// The associated value for key, or None in case of error.
	pub fn get_metadata(&self, key : MetadataKey) -> Option<String> {
		let key = key.to_str();
		get_string_with_string! (key, {
			openmpt_sys::openmpt_module_get_metadata(self.inner, key)
		})
	}

	pub fn get_metadata_keys(&self) -> String {
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
	fn all_known_metadata_keys_are_supported() {
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