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

fn parse_bool(s: &str) -> Result<bool, &'static str> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err("Failed to parse return value as boolean"),
    }
}

fn parse_float(s: &str) -> Result<f64, &'static str> {
    s.parse().map_err(|_| "Failed to parse return value as boolean")
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
	pub fn ctl_get(&self, ctl_key: &CtlKey) -> Result<Ctl, &str> {
		let return_value = self.ctl_get_string(&ctl_key);

		if let Some(value) = return_value {
			let new_ctl = match *ctl_key {
				CtlKey::SkipLoadingSamples => Ctl::SkipLoadingSamples(parse_bool(&value)?),
				CtlKey::SkipLoadingPatterns => Ctl::SkipLoadingPatterns(parse_bool(&value)?),
				CtlKey::SkipLoadingPlugins => Ctl::SkipLoadingPlugins(parse_bool(&value)?),
				CtlKey::SkipSubsongPreinit => Ctl::SkipSubsongPreinit(parse_bool(&value)?),
				CtlKey::SyncSamplesWhenSeeking => Ctl::SyncSamplesWhenSeeking(parse_bool(&value)?),
				CtlKey::PlaybackTempoFactor => Ctl::PlaybackTempoFactor(parse_float(&value)?),
				CtlKey::PlaybackPitchFactor => Ctl::PlaybackPitchFactor(parse_float(&value)?),
				CtlKey::DitherMode16Bit => Ctl::DitherMode16Bit(DitherMode::from_str(&value)?),
			};

			Ok(new_ctl)
		} else {
			Err("No value for this ctl")
		}
	}

	pub fn ctl_get_string(&self, ctl_key: &CtlKey) -> Option<String> {
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

	pub fn ctl_set(&mut self, ctl: &Ctl) -> bool {
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
		
		assert_eq!(module.ctl_get_string(&CtlKey::PlaybackTempoFactor).unwrap(), "2");
		assert_eq!(module.ctl_get_string(&CtlKey::PlaybackPitchFactor).unwrap(), "2");

		match module.ctl_get(&CtlKey::PlaybackTempoFactor).unwrap() {
			Ctl::PlaybackTempoFactor(2.0) => (),
			_ => panic!(),
		}

		match module.ctl_get(&CtlKey::PlaybackPitchFactor).unwrap() {
			Ctl::PlaybackPitchFactor(2.0) => (),
			_ => panic!(),
		}
	}
}