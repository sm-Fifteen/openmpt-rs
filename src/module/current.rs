use super::Module;
use openmpt_sys;
use std::os::raw::*;

impl Module {
	pub fn select_subsong(&mut self, subsong_num: i32) -> bool {
		let return_code = unsafe {
			openmpt_sys::openmpt_module_select_subsong(self.inner, subsong_num)
		};

		if return_code == 0 { false } else { true }
	}

	pub fn set_position_seconds(&mut self, seconds: c_double) -> c_double {
		// Never fails, will set position to begining or end of the song of out of range
		unsafe {
			openmpt_sys::openmpt_module_set_position_seconds(self.inner, seconds)
		}
	}

	pub fn set_position_order_row(&mut self, order: i32, row: i32) -> c_double {
		// Returns current position on failure
		unsafe {
			openmpt_sys::openmpt_module_set_position_order_row(self.inner, seconds)
		}
	}

	pub fn get_position_seconds(&self) -> c_double {
		unsafe {
			openmpt_sys::openmpt_module_get_position_seconds(self.inner)
		}
	}

	pub fn get_current_order(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_current_order(self.inner)
		}
	}

	pub fn get_current_pattern(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_current_pattern(self.inner)
		}
	}

	pub fn get_current_row(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_current_row(self.inner)
		}
	}

	pub fn get_current_speed(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_current_speed(self.inner)
		}
	}

	pub fn get_current_tempo(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_current_tempo(self.inner)
		}
	}

	pub fn get_current_playing_channels(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_current_playing_channels(self.inner)
		}
	}

	pub fn get_duration_seconds(&self) -> c_float {
		// Depends on the current subsong
		unsafe {
			openmpt_sys::openmpt_module_get_duration_seconds(self.inner)
		}
	}

	pub fn get_current_channel_vu_mono(&self, channel_num: i32) -> c_float {
		unsafe {
			openmpt_sys::openmpt_module_get_current_channel_vu_mono(self.inner, channel_num)
		}
	}

	pub fn get_current_channel_vu_left(&self, channel_num: i32) -> c_float {
		unsafe {
			openmpt_sys::openmpt_module_get_current_channel_vu_left(self.inner, channel_num)
		}
	}

	pub fn get_current_channel_vu_right(&self, channel_num: i32) -> c_float {
		unsafe {
			openmpt_sys::openmpt_module_get_current_channel_vu_right(self.inner, channel_num)
		}
	}

	pub fn get_current_channel_vu_rear_left(&self, channel_num: i32) -> c_float {
		unsafe {
			openmpt_sys::openmpt_module_get_current_channel_vu_rear_left(self.inner, channel_num)
		}
	}

	pub fn get_current_channel_vu_rear_right(&self, channel_num: i32) -> c_float {
		unsafe {
			openmpt_sys::openmpt_module_get_current_channel_vu_rear_right(self.inner, channel_num)
		}
	}
}

// Tests

	// #[test]
	// fn unatco_can_change_subsong() {
	// 	let mut module = test_helper::load_file_as_module("UNATCO.it").unwrap();
	// 	let subsongs = module.get_subsongs();

	// 	assert_eq!(subsongs.len(), 5); // Main, Game over, Dialogue /w intro, Combat, Dialogue loop
		
	// 	for song in subsongs {
	// 		assert!(module.select_subsong(&song));
	// 	}
	// }