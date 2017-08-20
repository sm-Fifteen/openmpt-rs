use openmpt_sys;
use std::ffi::CString;
use std::ffi::NulError;
use std::os::raw::*;
use std::ptr;

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
	fn to_str(&self) -> &str {
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

	fn to_cstr(&self) -> CString {
		CString::new(self.to_str())
			.expect("Failed to unwrap a CString cast that shouldn't fail")
	}
	
	fn param_to_cstr(&self) -> CString {
		use self::Ctl::*;
		match *self {
			SkipLoadingSamples(ref param) =>  CString::new(if *param {"1"} else {"0"}),
			SkipLoadingPatterns(ref param) => CString::new(if *param {"1"} else {"0"}),
			SkipLoadingPlugins(ref param) => CString::new(if *param {"1"} else {"0"}),
			SkipSubsongPreinit(ref param) => CString::new(if *param {"1"} else {"0"}),
			SyncSamplesWhenSeeking(ref param) => CString::new(if *param {"1"} else {"0"}),
			PlaybackTempoFactor(ref param) => CString::new(param.to_string()),
			PlaybackPitchFactor(ref param) => CString::new(param.to_string()),
			DitherMode16Bit(ref param) => match *param {
				DitherMode::None => CString::new("0"),
				DitherMode::Auto => CString::new("1"),
				DitherMode::ModPlug => CString::new("2"),
				DitherMode::Simple => CString::new("3"),
			},
		}.expect("Failed to unwrap a CString cast that shouldn't fail")
	}
}

pub(super) fn to_initial_ctl_ptr(list : &[Ctl]) -> *const openmpt_sys::openmpt_module_initial_ctl {
	// TODO : Function stub, always returns a null pointer as if there were no ctls
	//if list.len() == 0 {
	//&openmpt_sys::openmpt_module_initial_ctl::default()
	//} else {
	//
	//}
	ptr::null()
}