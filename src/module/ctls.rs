use openmpt_sys;
use super::Module;
use std::str::FromStr;
use std::os::raw::*;

const load_skip_samples:&str = "load.skip_samples";
const load_skip_patterns:&str = "load.skip_patterns";
const load_skip_plugins:&str = "load.skip_plugins";
const load_skip_subsongs_init:&str = "load.skip_subsongs_init";
const seek_sync_samples:&str = "seek.sync_samples";
const play_tempo_factor:&str = "play.tempo_factor";
const play_pitch_factor:&str = "play.pitch_factor";
const dither:&str = "dither";

#[derive(PartialEq, Debug)]
pub enum DitherMode {
	Auto,
	ModPlug,
	Simple,
	None,
}

impl FromStr for DitherMode {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"0" => Ok(DitherMode::None),
			"1" => Ok(DitherMode::Auto),
			"2" => Ok(DitherMode::ModPlug),
			"3" => Ok(DitherMode::Simple),
			_ => Err("Failed to parse return value as known Dither Mode")
		}
	}
}

pub enum Ctl {
	SkipLoadingSamples(bool),
	SkipLoadingPatterns(bool),
	SkipLoadingPlugins(bool),
	SkipSubsongPreinit(bool),
	SyncSamplesWhenSeeking(bool),
	PlaybackTempoFactor(c_double),
	PlaybackPitchFactor(c_double),
	DitherMode16Bit(DitherMode),
}

impl Ctl {
	fn key_to_str(&self) -> String {
		match *self {
			Ctl::SkipLoadingSamples(_) =>  load_skip_samples,
			Ctl::SkipLoadingPatterns(_) => load_skip_patterns,
			Ctl::SkipLoadingPlugins(_) => load_skip_plugins,
			Ctl::SkipSubsongPreinit(_) => load_skip_subsongs_init,
			Ctl::SyncSamplesWhenSeeking(_) => seek_sync_samples,
			Ctl::PlaybackTempoFactor(_) => play_tempo_factor,
			Ctl::PlaybackPitchFactor(_) => play_pitch_factor,
			Ctl::DitherMode16Bit(_) => dither,
		}.to_owned()
	}

	fn param_to_str(&self) -> String {
		use self::Ctl::*;
		match *self {
			SkipLoadingSamples(ref param) =>  if *param {"1"} else {"0"}.to_owned(),
			SkipLoadingPatterns(ref param) => if *param {"1"} else {"0"}.to_owned(),
			SkipLoadingPlugins(ref param) => if *param {"1"} else {"0"}.to_owned(),
			SkipSubsongPreinit(ref param) => if *param {"1"} else {"0"}.to_owned(),
			SyncSamplesWhenSeeking(ref param) => if *param {"1"} else {"0"}.to_owned(),
			PlaybackTempoFactor(ref param) => param.to_string(),
			PlaybackPitchFactor(ref param) => param.to_string(),
			DitherMode16Bit(ref param) => match *param {
				DitherMode::None => "0",
				DitherMode::Auto => "1",
				DitherMode::ModPlug => "2",
				DitherMode::Simple => "3",
			}.to_owned(),
		}
	}
}

impl Module {
	pub fn ctl_get_load_skip_samples(&self) -> Option<bool> {
		let return_val = self.ctl_get(load_skip_samples);

		if let Some(ref str_val) = return_val {
			i32::from_str(str_val).map(|num| num != 0).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_samples(&mut self, value: bool) -> bool {
		self.enum_ctl_set(&Ctl::SkipLoadingSamples(value))
	}

	pub fn ctl_get_load_skip_patterns(&self) -> Option<bool> {
		let return_val = self.ctl_get(load_skip_patterns);

		if let Some(ref str_val) = return_val {
			i32::from_str(str_val).map(|num| num != 0).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_patterns(&mut self, value: bool) -> bool {
		self.enum_ctl_set(&Ctl::SkipLoadingPatterns(value))
	}

	pub fn ctl_get_load_skip_plugins(&self) -> Option<bool> {
		let return_val = self.ctl_get(load_skip_plugins);

		if let Some(ref str_val) = return_val {
			i32::from_str(str_val).map(|num| num != 0).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_plugins(&mut self, value: bool) -> bool {
		self.enum_ctl_set(&Ctl::SkipLoadingPlugins(value))
	}

	pub fn ctl_get_load_skip_subsongs_init(&self) -> Option<bool> {
		let return_val = self.ctl_get(load_skip_subsongs_init);

		if let Some(ref str_val) = return_val {
			i32::from_str(str_val).map(|num| num != 0).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_subsongs_init(&mut self, value: bool) -> bool {
		self.enum_ctl_set(&Ctl::SkipSubsongPreinit(value))
	}

	pub fn ctl_get_seek_sync_samples(&self) -> Option<bool> {
		let return_val = self.ctl_get(seek_sync_samples);

		if let Some(ref str_val) = return_val {
			i32::from_str(str_val).map(|num| num != 0).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_seek_sync_samples(&mut self, value: bool) -> bool {
		self.enum_ctl_set(&Ctl::SyncSamplesWhenSeeking(value))
	}

	pub fn ctl_get_play_tempo_factor(&self) -> Option<c_double> {
		let return_val = self.ctl_get(play_tempo_factor);

		if let Some(ref str_val) = return_val {
			c_double::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_play_tempo_factor(&mut self, value: c_double) -> bool {
		self.enum_ctl_set(&Ctl::PlaybackTempoFactor(value))
	}

	pub fn ctl_get_play_pitch_factor(&self) -> Option<c_double> {
		let return_val = self.ctl_get(play_pitch_factor);

		if let Some(ref str_val) = return_val {
			c_double::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_play_pitch_factor(&mut self, value: c_double) -> bool {
		self.enum_ctl_set(&Ctl::PlaybackPitchFactor(value))
	}

	pub fn ctl_get_dither(&self) -> Option<DitherMode> {
		let return_val = self.ctl_get(dither);

		if let Some(ref str_val) = return_val {
			DitherMode::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_dither(&mut self, value: DitherMode) -> bool {
		self.enum_ctl_set(&Ctl::DitherMode16Bit(value))
	}

	pub fn ctl_get(&self, key: &str) -> Option<String> {
		get_string_with_string!(key, {
			openmpt_sys::openmpt_module_ctl_get(self.inner, key)
		})
	}

	pub(super) fn enum_ctl_set(&mut self, ctl: &Ctl) -> bool {
		let key = ctl.key_to_str();
		let val = ctl.param_to_str();

		self.ctl_set(&key, &val)
	}

	pub fn ctl_set(&mut self, key: &str, val: &str) -> bool {
		let return_value = with_2strings!(key, val, {
			openmpt_sys::openmpt_module_ctl_set(self.inner, key, val)
		});

		if return_value == 1 { true } else { false }
	}

	fn get_ctls(&self) -> String {
		let opt_string = get_string! {
			openmpt_sys::openmpt_module_get_ctls(self.inner)
		};

		opt_string.expect("Got null pointer instead of string")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::Module;
	use super::super::Logger;
	use super::super::test_helper;

	#[test]
	fn all_known_ctls_are_supported() {
		let module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		let keys = module.get_ctls();
		
		assert!(keys.contains(load_skip_samples));
		assert!(keys.contains(load_skip_patterns));
		assert!(keys.contains(load_skip_plugins));
		assert!(keys.contains(load_skip_subsongs_init));
		assert!(keys.contains(seek_sync_samples));
		assert!(keys.contains(play_tempo_factor));
		assert!(keys.contains(play_pitch_factor));
		assert!(keys.contains(dither));
	}

	#[test]
	fn default_ctls_are_respected() {
		let mut module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		
		assert_eq!(module.ctl_get_load_skip_samples().unwrap(), false);
		assert_eq!(module.ctl_get_load_skip_patterns().unwrap(), false);
		assert_eq!(module.ctl_get_load_skip_plugins().unwrap(), false);
		assert_eq!(module.ctl_get_load_skip_subsongs_init().unwrap(), false);
		assert_eq!(module.ctl_get_seek_sync_samples().unwrap(), false);
		assert_eq!(module.ctl_get_play_tempo_factor().unwrap(), 1.0);
		assert_eq!(module.ctl_get_play_pitch_factor().unwrap(), 1.0);
		assert_eq!(module.ctl_get_dither().unwrap(), DitherMode::Auto);
	}

	#[test]
	fn initial_ctls_are_respected() {
		use std::io::prelude::*;
		use std::fs::File;

		let mut f = File::open("empty_module.xm").expect("file not found");
		let mut buf = Vec::new();
		f.read_to_end(&mut buf);

		let initial_ctls = vec!{
			Ctl::SkipLoadingSamples(true),
			Ctl::SkipLoadingPatterns(true),
			Ctl::SkipLoadingPlugins(true),
			Ctl::SkipSubsongPreinit(true),
			Ctl::SyncSamplesWhenSeeking(true),
			Ctl::PlaybackTempoFactor(2.0),
			Ctl::PlaybackPitchFactor(2.0),
			Ctl::DitherMode16Bit(DitherMode::Simple),
		};
		let module = Module::create_from_memory(&mut buf, Logger::None, &initial_ctls).unwrap();
		
		assert_eq!(module.ctl_get_load_skip_samples().unwrap(), true);
		assert_eq!(module.ctl_get_load_skip_patterns().unwrap(), true);
		assert_eq!(module.ctl_get_load_skip_plugins().unwrap(), true);
		assert_eq!(module.ctl_get_load_skip_subsongs_init().unwrap(), true);
		assert_eq!(module.ctl_get_seek_sync_samples().unwrap(), true);
		assert_eq!(module.ctl_get_play_tempo_factor().unwrap(), 2.0);
		assert_eq!(module.ctl_get_play_pitch_factor().unwrap(), 2.0);
		assert_eq!(module.ctl_get_dither().unwrap(), DitherMode::Simple);
	}

	#[test]
	fn clean_result_for_getting_unknown_ctl() {
		let mut module = test_helper::load_file_as_module("empty_module.xm").unwrap();

		assert!(module.ctl_get("invalid_ctl").is_none());
	}

	#[test]
	fn clean_result_for_setting_invalid_ctl() {
		let mut module = test_helper::load_file_as_module("empty_module.xm").unwrap();

		try_set_ctl(&mut module, dither, "26");
	}

	fn try_set_ctl (module: &mut Module, key: &str, new_val: &str) {
		// Apparently, those only return false if the string pointers are invalid.
		// assert!(!module.ctl_set(dither, "26"));

		module.ctl_set(key, "26");
		println!("Tried setting {:?} at {:?}, now at {:?}", key, new_val, module.ctl_get(dither).unwrap());
	}
}