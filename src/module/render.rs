use super::Module;
use openmpt_sys;
use std::os::raw::*;
use std::cmp::min;

const RENDER_MASTERGAIN_MILLIBEL:c_int = 1;
const RENDER_STEREOSEPARATION_PERCENT:c_int = 2;
const RENDER_INTERPOLATIONFILTER_LENGTH:c_int = 3;
const RENDER_VOLUMERAMPING_STRENGTH:c_int = 4;

impl Module {
	pub fn read_mono(&mut self, sample_rate : i32, mono: &mut Vec<i16>) -> usize {
		let count = mono.capacity();

		unsafe {
			openmpt_sys::openmpt_module_read_mono(self.inner, sample_rate, count, mono.as_mut_ptr())
		}
	}

	pub fn read_float_mono(&mut self, sample_rate : i32, mono: &mut Vec<c_float>) -> usize {
		let count = mono.capacity();
		
		unsafe {
			openmpt_sys::openmpt_module_read_float_mono(self.inner, sample_rate, count, mono.as_mut_ptr())
		}
	}

	pub fn read_stereo(&mut self, sample_rate : i32, left: &mut Vec<i16>, right: &mut Vec<i16>) -> usize {
		let count = min(left.capacity(), right.capacity());
		
		unsafe {
			openmpt_sys::openmpt_module_read_stereo(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr())
		}
	}

	pub fn read_float_stereo(&mut self, sample_rate : i32, left: &mut Vec<c_float>, right: &mut Vec<c_float>) -> usize {
		let count = min(left.capacity(), right.capacity());

		unsafe {
			openmpt_sys::openmpt_module_read_float_stereo(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr())
		}
	}

	pub fn read_interleaved_stereo(&mut self, sample_rate : i32, interleaved_stereo: &mut Vec<i16>) -> usize {
		let count = interleaved_stereo.capacity() >> 1; // Buffer needs to be of at least size count*2

		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_stereo(self.inner, sample_rate, count, interleaved_stereo.as_mut_ptr())
		}
	}

	pub fn read_interleaved_float_stereo(&mut self, sample_rate : i32, interleaved_stereo: &mut Vec<c_float>) -> usize {
		let count = interleaved_stereo.capacity() >> 1; // Buffer needs to be of at least size count*2
		
		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_float_stereo(self.inner, sample_rate, count, interleaved_stereo.as_mut_ptr())
		}
	}

	pub fn read_quad(&mut self, sample_rate : i32, left: &mut Vec<i16>, right: &mut Vec<i16>, rear_left: &mut Vec<i16>, rear_right: &mut Vec<i16>) -> usize {
		let count = min(min(left.capacity(), right.capacity()), min(rear_left.capacity(), rear_right.capacity()));
		
		unsafe {
			openmpt_sys::openmpt_module_read_quad(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr(), rear_left.as_mut_ptr(), rear_right.as_mut_ptr())
		}
	}

	pub fn read_float_quad(&mut self, sample_rate : i32, left: &mut Vec<c_float>, right: &mut Vec<c_float>, rear_left: &mut Vec<c_float>, rear_right: &mut Vec<c_float>) -> usize {
		let count = min(min(left.capacity(), right.capacity()), min(rear_left.capacity(), rear_right.capacity()));
		
		unsafe {
			openmpt_sys::openmpt_module_read_float_quad(self.inner, sample_rate, count, left.as_mut_ptr(), right.as_mut_ptr(), rear_left.as_mut_ptr(), rear_right.as_mut_ptr())
		}
	}

	pub fn read_interleaved_quad(&mut self, sample_rate : i32, interleaved_quad: &mut Vec<i16>) -> usize {
		let count = interleaved_quad.capacity() >> 2; // Buffer needs to be of at least size count*4

		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_quad(self.inner, sample_rate, count, interleaved_quad.as_mut_ptr())
		}
	}

	pub fn read_interleaved_float_quad(&mut self, sample_rate : i32, interleaved_quad: &mut Vec<c_float>) -> usize {
		let count = interleaved_quad.capacity() >> 2; // Buffer needs to be of at least size count*4
		
		unsafe {
			openmpt_sys::openmpt_module_read_interleaved_float_quad(self.inner, sample_rate, count, interleaved_quad.as_mut_ptr())
		}
	}

	pub fn set_repeat_count(&mut self, repeat_count: i32) -> bool {
		let return_value = unsafe {
			openmpt_sys::openmpt_module_set_repeat_count(self.inner, repeat_count)
		};

		return_value == 1
	}

	pub fn set_render_mastergain_millibel(&mut self, relative_gain: i32) -> bool {
		self.set_render_param(RENDER_MASTERGAIN_MILLIBEL, relative_gain)
	}

	pub fn set_render_stereo_separation(&mut self, percentage: i32) -> bool {
		self.set_render_param(RENDER_STEREOSEPARATION_PERCENT, percentage)
	}

	pub fn set_render_interpolation_filter_length(&mut self, filter_length: i32) -> bool {
		self.set_render_param(RENDER_INTERPOLATIONFILTER_LENGTH, filter_length)
	}

	pub fn set_render_volume_ramping(&mut self, strength: i32) -> bool {
		self.set_render_param(RENDER_VOLUMERAMPING_STRENGTH, strength)
	}

	fn set_render_param(&mut self, param: c_int, value: i32) -> bool {
		let return_value = unsafe {
			openmpt_sys::openmpt_module_set_render_param(self.inner, param, value)
		};

		return_value == 1
	}

	pub fn get_repeat_count(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_repeat_count(self.inner)
		}
	}

	pub fn get_render_mastergain_millibel(&self) -> Option<i32> {
		self.get_render_param(RENDER_MASTERGAIN_MILLIBEL)
	}

	pub fn get_render_stereo_separation(&self) -> Option<i32> {
		self.get_render_param(RENDER_STEREOSEPARATION_PERCENT)
	}

	pub fn get_render_interpolation_filter_length(&self) -> Option<i32> {
		self.get_render_param(RENDER_INTERPOLATIONFILTER_LENGTH)
	}

	pub fn get_render_volume_ramping(&self) -> Option<i32> {
		self.get_render_param(RENDER_VOLUMERAMPING_STRENGTH)
	}

	fn get_render_param(&self, param: c_int) -> Option<i32> {
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
	use super::*;
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