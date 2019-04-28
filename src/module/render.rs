//! Definitions for all methods related to module mixing and rendering only.
//!
//! ## Remarks
//! The output buffers are only written to up to the returned number of elements.
//!
//! You can freely switch between any of these function if you see a need to do so.
//! libopenmpt tries to introduce as little switching annoyances as possible.
//! Normally, you would only use a single one of these functions for rendering a particular module.
//!
//! It is recommended to use the floating point API because of the greater dynamic range
//! and no implied clipping.
//! 
//! Floating point samples are in the [-1.0..1.0] nominal range. They are not clipped to
//! that range though and thus might overshoot.

use super::Module;
use openmpt_sys;
use std::os::raw::*;
use std::cmp::min;

const RENDER_MASTERGAIN_MILLIBEL:c_int = 1;
const RENDER_STEREOSEPARATION_PERCENT:c_int = 2;
const RENDER_INTERPOLATIONFILTER_LENGTH:c_int = 3;
const RENDER_VOLUMERAMPING_STRENGTH:c_int = 4;

impl Module {
	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `mono` : Pointer to a buffer for the mono/center output that will receive an amount of audio frames equal to its capacity.
	///
	/// ### Returns
	/// The number of frames actually rendered, or 0 if the end of song has been reached.
	pub fn read_mono(&mut self, sample_rate : i32, mono: &mut [i16]) -> usize {
		let count = mono.len();

		unsafe {
			openmpt_sys::openmpt_module_read_mono(self.inner, sample_rate, count, mono.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `mono` : Pointer to a buffer for the mono/center output that will receive an amount of audio frames equal to its capacity.
	///
	/// ### Returns
	/// The number of frames actually rendered, or 0 if the end of song has been reached.
	pub fn read_float_mono(&mut self, sample_rate : i32, mono: &mut [c_float]) -> usize {
		let count = mono.len();
		
		unsafe {
			openmpt_sys::openmpt_module_read_float_mono(self.inner, sample_rate, count, mono.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `left` : Pointer to a buffer for the left output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `right` : Pointer to a buffer for the right output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	///
	/// ### Returns
	/// The number of frames actually rendered, or 0 if the end of song has been reached.
	pub fn read_stereo(&mut self, sample_rate : i32, left: &mut [i16], right: &mut [i16]) -> usize {
		let count = min(left.len(), right.len());
		
		unsafe {
			openmpt_sys::openmpt_module_read_stereo(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `left` : Pointer to a buffer for the left output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `right` : Pointer to a buffer for the right output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	///
	/// ### Returns
	/// The number of frames actually rendered, or 0 if the end of song has been reached.
	pub fn read_float_stereo(&mut self, sample_rate : i32, left: &mut [c_float], right: &mut [c_float]) -> usize {
		let count = min(left.len(), right.len());

		unsafe {
			openmpt_sys::openmpt_module_read_float_stereo(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `interleaved_stereo` : Pointer to a buffer for the interleaved stereo output (order : L,R) that will receive an amount of audio frames equal to its capacity divided by the number of channels.
	///
	/// ### Returns
	/// The number of frames actually rendered (up to half of the buffer's capacity), or 0 if the end of song has been reached.
	pub fn read_interleaved_stereo(&mut self, sample_rate : i32, interleaved_stereo: &mut [i16]) -> usize {
		let count = interleaved_stereo.len() >> 1; // Buffer needs to be of at least size count*2

		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_stereo(self.inner, sample_rate, count, interleaved_stereo.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `interleaved_stereo` : Pointer to a buffer for the interleaved stereo output (order : L,R) that will receive an amount of audio frames equal to its capacity divided by the number of channels.
	///
	/// ### Returns
	/// The number of frames actually rendered (up to half of the buffer's capacity), or 0 if the end of song has been reached.
	pub fn read_interleaved_float_stereo(&mut self, sample_rate : i32, interleaved_stereo: &mut [c_float]) -> usize {
		let count = interleaved_stereo.len() >> 1; // Buffer needs to be of at least size count*2
		
		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_float_stereo(self.inner, sample_rate, count, interleaved_stereo.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `left` : Pointer to a buffer for the left output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `right` : Pointer to a buffer for the right output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `rear_left` : Pointer to a buffer for the rear left output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `rear_right` : Pointer to a buffer for the rear right output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	///
	/// ### Returns
	/// The number of frames actually rendered, or 0 if the end of song has been reached.
	pub fn read_quad(&mut self, sample_rate : i32, left: &mut [i16], right: &mut [i16], rear_left: &mut [i16], rear_right: &mut [i16]) -> usize {
		let count = min(min(left.len(), right.len()), min(rear_left.len(), rear_right.len()));
		
		unsafe {
			openmpt_sys::openmpt_module_read_quad(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr(), rear_left.as_mut_ptr(), rear_right.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `left` : Pointer to a buffer for the left output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `right` : Pointer to a buffer for the right output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `rear_left` : Pointer to a buffer for the rear left output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	/// * `rear_right` : Pointer to a buffer for the rear right output that will receive an amount of audio frames equal to the capacity of the smallest buffer.
	///
	/// ### Returns
	/// The number of frames actually rendered, or 0 if the end of song has been reached.
	pub fn read_float_quad(&mut self, sample_rate : i32, left: &mut [c_float], right: &mut [c_float], rear_left: &mut [c_float], rear_right: &mut [c_float]) -> usize {
		let count = min(min(left.len(), right.len()), min(rear_left.len(), rear_right.len()));
		
		unsafe {
			openmpt_sys::openmpt_module_read_float_quad(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr(), rear_left.as_mut_ptr(), rear_right.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `interleaved_quad` : Pointer to a buffer for the interleaved stereo output (order : L,R,RL,RR) that will receive an amount of audio frames equal to its capacity divided by the number of channels.
	///
	/// ### Returns
	/// The number of frames actually rendered (up to one fourth of the buffer's capacity), or 0 if the end of song has been reached.
	pub fn read_interleaved_quad(&mut self, sample_rate : i32, interleaved_quad: &mut [i16]) -> usize {
		let count = interleaved_quad.len() >> 2; // Buffer needs to be of at least size count*4

		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_quad(self.inner, sample_rate, count, interleaved_quad.as_mut_ptr())
		}
	}

	/// Render audio data.
	///
	/// ### Parameters
	/// * `sample_rate` : Sample rate to render output. Should be in [8000,192000], but this is not enforced.
	/// * `interleaved_quad` : Pointer to a buffer for the interleaved stereo output (order : L,R,RL,RR) that will receive an amount of audio frames equal to its capacity divided by the number of channels.
	///
	/// ### Returns
	/// The number of frames actually rendered (up to one fourth of the buffer's capacity), or 0 if the end of song has been reached.
	pub fn read_interleaved_float_quad(&mut self, sample_rate : i32, interleaved_quad: &mut [c_float]) -> usize {
		let count = interleaved_quad.len() >> 2; // Buffer needs to be of at least size count*4
		
		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_float_quad(self.inner, sample_rate, count, interleaved_quad.as_mut_ptr())
		}
	}

	/// Set Repeat Count.
	///
	/// ### Parameters
	/// * `repeat_count` : Repeat Count
	///
	/// ### Details
	/// * -1: repeat forever
	/// * 0: play once, repeat zero times (the default)
	/// * n>0: play once and repeat n times after that
	///
	/// ### Returns
	/// Whether or not the operation has succeded.
	pub fn set_repeat_count(&mut self, repeat_count: i32) -> bool {
		let return_value = unsafe {
			openmpt_sys::openmpt_module_set_repeat_count(self.inner, repeat_count)
		};

		return_value == 1
	}

	/// Set master gain
	///
	/// ### Parameters
	/// * `relative_gain` : Relative gain in milliBel.
	///
	/// ### Details
	/// The default value is 0.
	///
	/// The supported value range is unlimited.
	///
	/// ### Returns
	/// Whether or not the operation has succeded.
	pub fn set_render_mastergain_millibel(&mut self, relative_gain: i32) -> bool {
		self.set_render_param(RENDER_MASTERGAIN_MILLIBEL, relative_gain)
	}

	/// Set stereo separation
	///
	/// ### Parameters
	/// * `percentage` : The stereo separation generated by the libopenmpt mixer in percent.
	///
	/// ### Details
	/// * The default value is 100.
	/// * The supported value range is [0,200].
	///
	/// ### Returns
	/// Whether or not the operation has succeded.
	pub fn set_render_stereo_separation(&mut self, percentage: i32) -> bool {
		self.set_render_param(RENDER_STEREOSEPARATION_PERCENT, percentage)
	}

	/// Set length for the interpolation filter
	///
	/// ### Parameters
	/// * `filter_length` : The interpolation filter length used by the libopenmpt mixer.
	///
	/// ### Details
	/// The default value is 0, which indicates a recommended default value.
	///
	/// The supported value range is [0,inf). Values greater than the implementation limit are clamped to the maximum supported value.
	///
	/// Currently supported values:
	/// * 0: internal default
	/// * 1: no interpolation (zero order hold)
	/// * 2: linear interpolation
	/// * 4: cubic interpolation
	/// * 8: windowed sinc with 8 taps
	///
	/// ### Returns
	/// Whether or not the operation has succeded.
	pub fn set_render_interpolation_filter_length(&mut self, filter_length: i32) -> bool {
		self.set_render_param(RENDER_INTERPOLATIONFILTER_LENGTH, filter_length)
	}

	/// Set volume ramping strength
	///
	/// ### Parameters
	/// * `strength` : The amount of volume ramping done by the libopenmpt mixer.
	///
	/// ### Details
	/// * The default value is -1, which indicates a recommended default value.
	/// * The meaningful value range is [-1..10].
	/// * A value of 0 completely disables volume ramping. This might cause clicks in sound output.
	/// * Higher values imply slower/softer volume ramps.
	///
	/// ### Returns
	/// Whether or not the operation has succeded.
	pub fn set_render_volume_ramping(&mut self, strength: i32) -> bool {
		self.set_render_param(RENDER_VOLUMERAMPING_STRENGTH, strength)
	}

	fn set_render_param(&mut self, param: c_int, value: i32) -> bool {
		let return_value = unsafe {
			openmpt_sys::openmpt_module_set_render_param(self.inner, param, value)
		};

		return_value == 1
	}

	/// Get Repeat Count.
	///
	/// ### Returns
	/// Repeat Count
	///
	/// * -1: repeat forever
	/// * 0: play once, repeat zero times (the default)
	/// * n>0: play once and repeat n times after that
	pub fn get_repeat_count(&mut self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_repeat_count(self.inner)
		}
	}

	/// Get master gain
	///
	/// ### Returns
	/// Relative gain in milliBel.
	pub fn get_render_mastergain_millibel(&mut self) -> Option<i32> {
		self.get_render_param(RENDER_MASTERGAIN_MILLIBEL)
	}

	/// Get stereo separation
	///
	/// ### Returns
	/// The stereo separation generated by the libopenmpt mixer in percent.
	pub fn get_render_stereo_separation(&mut self) -> Option<i32> {
		self.get_render_param(RENDER_STEREOSEPARATION_PERCENT)
	}

	/// Get length for the interpolation filter
	///
	/// ### Returns
	/// The interpolation filter length used by the libopenmpt mixer.
	pub fn get_render_interpolation_filter_length(&mut self) -> Option<i32> {
		self.get_render_param(RENDER_INTERPOLATIONFILTER_LENGTH)
	}

	/// Set volume ramping strength
	///
	/// ### Returns
	/// The amount of volume ramping done by the libopenmpt mixer.
	pub fn get_render_volume_ramping(&mut self) -> Option<i32> {
		self.get_render_param(RENDER_VOLUMERAMPING_STRENGTH)
	}

	fn get_render_param(&mut self, param: c_int) -> Option<i32> {
		let mut out:i32 = 0;
		let out_ptr = &mut out as *mut i32;
		let return_value = unsafe {
			openmpt_sys::openmpt_module_get_render_param(self.inner, param, out_ptr)
		};
		//let out = unsafe { *out_ptr };

		if return_value == 1 {
			Some(out)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::test_helper;

	#[test]
	fn dummy_file_opens_with_default_render_parameters() {
		let mut module = test_helper::load_file_as_module("empty_module.xm").unwrap();

		assert_eq!(module.get_render_mastergain_millibel().unwrap(), 0);
		assert_eq!(module.get_render_stereo_separation().unwrap(), 100);
		// Not always 0, just the selected default (8 in this case)
		assert_eq!(module.get_render_interpolation_filter_length().unwrap(), 8);
		assert_eq!(module.get_render_volume_ramping().unwrap(), -1);
	}

	#[test]
	fn render_parameters_changes_are_correctly_applied() {
		let mut module = test_helper::load_file_as_module("empty_module.xm").unwrap();

		module.set_render_mastergain_millibel(10);
		assert_eq!(module.get_render_mastergain_millibel().unwrap(), 10);

		module.set_render_stereo_separation(150);
		assert_eq!(module.get_render_stereo_separation().unwrap(), 150);
		
		module.set_render_interpolation_filter_length(1);
		assert_eq!(module.get_render_interpolation_filter_length().unwrap(), 1);

		module.set_render_volume_ramping(0);
		assert_eq!(module.get_render_volume_ramping().unwrap(), 0);
	}
}
