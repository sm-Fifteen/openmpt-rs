use super::Module;
use openmpt_sys;
use std::os::raw::*;
use std::cmp::min;

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
}