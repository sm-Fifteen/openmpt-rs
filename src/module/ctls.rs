use openmpt_sys;
use std::ffi::CString;
use std::ffi::NulError;
use std::os::raw::*;

pub(super) struct InitialCtl {
	pub ctl: CString,
	pub value: CString,
}

pub enum DitherMode {
	Auto,
	ModPlug,
	Simple,
	None,
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
	fn key_to_str(&self) -> &str {
		use self::Ctl::*;
		match *self {
			SkipLoadingSamples(_) =>  "load.skip_samples",
			SkipLoadingPatterns(_) => "load.skip_patterns",
			SkipLoadingPlugins(_) => "load.skip_plugins",
			SkipSubsongPreinit(_) => "load.skip_subsongs_init",
			SyncSamplesWhenSeeking(_) => "seek.sync_samples",
			PlaybackTempoFactor(_) => "play.tempo_factor",
			PlaybackPitchFactor(_) => "play.pitch_factor",
			DitherMode16Bit(_) => "dither",
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

	pub(super) fn to_initial_ctl(&self) -> InitialCtl {
		InitialCtl {
			ctl: CString::new(self.key_to_str()).unwrap(),
			value: CString::new(self.param_to_str()).unwrap(),
		}
	}
}