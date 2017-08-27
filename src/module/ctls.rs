use openmpt_sys;
use super::Module;
use std::str::FromStr;
use std::os::raw::*;

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

pub enum CtlKey {
	SkipLoadingSamples,
	SkipLoadingPatterns,
	SkipLoadingPlugins,
	SkipSubsongPreinit,
	SyncSamplesWhenSeeking,
	PlaybackTempoFactor,
	PlaybackPitchFactor,
	DitherMode16Bit,
}

impl Ctl {
	fn key(&self) -> CtlKey {
		match *self {
			Ctl::SkipLoadingSamples(_) => CtlKey::SkipLoadingSamples,
			Ctl::SkipLoadingPatterns(_) => CtlKey::SkipLoadingPatterns,
			Ctl::SkipLoadingPlugins(_) => CtlKey::SkipLoadingPlugins,
			Ctl::SkipSubsongPreinit(_) => CtlKey::SkipSubsongPreinit,
			Ctl::SyncSamplesWhenSeeking(_) => CtlKey::SyncSamplesWhenSeeking,
			Ctl::PlaybackTempoFactor(_) => CtlKey::PlaybackTempoFactor,
			Ctl::PlaybackPitchFactor(_) => CtlKey::PlaybackPitchFactor,
			Ctl::DitherMode16Bit(_) => CtlKey::DitherMode16Bit,
		}
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

impl CtlKey {
	fn to_str(&self) -> String {
		use self::CtlKey::*;
		match *self {
			SkipLoadingSamples =>  "load.skip_samples",
			SkipLoadingPatterns => "load.skip_patterns",
			SkipLoadingPlugins => "load.skip_plugins",
			SkipSubsongPreinit => "load.skip_subsongs_init",
			SyncSamplesWhenSeeking => "seek.sync_samples",
			PlaybackTempoFactor => "play.tempo_factor",
			PlaybackPitchFactor => "play.pitch_factor",
			DitherMode16Bit => "dither",
		}.to_owned()
	}
}

impl Module {
	pub fn ctl_get_load_skip_samples(&self) -> Option<bool> {
		let return_val = self.ctl_get_string(&CtlKey::SkipLoadingSamples);

		if let Some(ref str_val) = return_val {
			bool::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_samples(&mut self, value: bool) -> bool {
		self.ctl_set(&Ctl::SkipLoadingSamples(value))
	}

	pub fn ctl_get_load_skip_patterns(&self) -> Option<bool> {
		let return_val = self.ctl_get_string(&CtlKey::SkipLoadingPatterns);

		if let Some(ref str_val) = return_val {
			bool::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_patterns(&mut self, value: bool) -> bool {
		self.ctl_set(&Ctl::SkipLoadingPatterns(value))
	}

	pub fn ctl_get_load_skip_plugins(&self) -> Option<bool> {
		let return_val = self.ctl_get_string(&CtlKey::SkipLoadingPlugins);

		if let Some(ref str_val) = return_val {
			bool::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_plugins(&mut self, value: bool) -> bool {
		self.ctl_set(&Ctl::SkipLoadingPlugins(value))
	}

	pub fn ctl_get_load_skip_subsongs_init(&self) -> Option<bool> {
		let return_val = self.ctl_get_string(&CtlKey::SkipSubsongPreinit);

		if let Some(ref str_val) = return_val {
			bool::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_load_skip_subsongs_init(&mut self, value: bool) -> bool {
		self.ctl_set(&Ctl::SkipSubsongPreinit(value))
	}

	pub fn ctl_get_seek_sync_samples(&self) -> Option<bool> {
		let return_val = self.ctl_get_string(&CtlKey::SyncSamplesWhenSeeking);

		if let Some(ref str_val) = return_val {
			bool::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_seek_sync_samples(&mut self, value: bool) -> bool {
		self.ctl_set(&Ctl::SyncSamplesWhenSeeking(value))
	}

	pub fn ctl_get_play_tempo_factor(&self) -> Option<c_double> {
		let return_val = self.ctl_get_string(&CtlKey::PlaybackTempoFactor);

		if let Some(ref str_val) = return_val {
			c_double::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_play_tempo_factor(&mut self, value: c_double) -> bool {
		self.ctl_set(&Ctl::PlaybackTempoFactor(value))
	}

	pub fn ctl_get_play_pitch_factor(&self) -> Option<c_double> {
		let return_val = self.ctl_get_string(&CtlKey::PlaybackPitchFactor);

		if let Some(ref str_val) = return_val {
			c_double::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_play_pitch_factor(&mut self, value: c_double) -> bool {
		self.ctl_set(&Ctl::PlaybackPitchFactor(value))
	}

	pub fn ctl_get_dither(&self) -> Option<DitherMode> {
		let return_val = self.ctl_get_string(&CtlKey::DitherMode16Bit);

		if let Some(ref str_val) = return_val {
			DitherMode::from_str(str_val).ok()
		} else {
			None
		}
	}

	pub fn ctl_set_dither(&mut self, value: DitherMode) -> bool {
		self.ctl_set(&Ctl::DitherMode16Bit(value))
	}

	fn ctl_get_string(&self, ctl_key: &CtlKey) -> Option<String> {
		let key = ctl_key.to_str();
		let return_value = get_string_with_string!(key, {
			openmpt_sys::openmpt_module_ctl_get(self.inner, key)
		});

		if return_value.len() == 0 {
			None
		} else {
			Some(return_value)
		}
	}

	pub(super) fn ctl_set(&mut self, ctl: &Ctl) -> bool {
		let key = ctl.key().to_str();
		let val = ctl.param_to_str();

		let return_value = with_2strings!(key, val, {
			openmpt_sys::openmpt_module_ctl_set(self.inner, key, val)
		});

		if return_value == 1 { true } else { false }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::Module;
	use super::super::test_helper;

	#[test]
	fn initial_ctls_are_respected() {
		use super::super::logging;
		use std::io::prelude::*;
		use std::fs::File;

		let mut f = File::open("empty_module.xm").expect("file not found");
		let mut buf = Vec::new();
		f.read_to_end(&mut buf);

		let initial_ctls = vec!{ Ctl::PlaybackTempoFactor(2.0), Ctl::PlaybackPitchFactor(2.0) };
		let module = Module::create_from_memory(&mut buf, logging::Logger::None, &initial_ctls).unwrap();
		
		assert_eq!(module.ctl_get_play_tempo_factor().unwrap(), 2.0);
		assert_eq!(module.ctl_get_play_pitch_factor().unwrap(), 2.0);
	}
}