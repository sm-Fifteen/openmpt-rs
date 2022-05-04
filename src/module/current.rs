//! Definitions for all methods used to set and query
//! the current state of the module

use super::Module;
use openmpt_sys;
use std::os::raw::*;

impl Module {
    /// Select a sub-song from a multi-song module.
    ///
    /// ### Parameters
    /// * `subsong_num` : Index of the sub-song. -1 plays all sub-songs consecutively.
    ///
    /// ### Returns
    /// Whether or not the operation has succeded.
    ///
    /// ### Remarks
    /// Whether subsong -1 (all subsongs consecutively), subsong 0 or some other subsong
    /// is selected by default, is an implementation detail and subject to change.
    /// If you do not want to care about subsongs, it is recommended to just not call this method at all.
    pub fn select_subsong(&mut self, subsong_num: i32) -> bool {
        let return_code =
            unsafe { openmpt_sys::openmpt_module_select_subsong(self.inner, subsong_num) };

        return_code != 0
    }

    /// Set approximate current song position.
    ///
    /// ### Parameters
    /// * `seconds` : Seconds to seek to.
    ///
    /// ### Returns
    /// Approximate new song position in seconds.
    ///
    /// ### Remarks
    /// If seconds is out of range, the position gets set to song start or end respectively.
    pub fn set_position_seconds(&mut self, seconds: c_double) -> c_double {
        // Never fails, will set position to begining or end of the song of out of range
        unsafe { openmpt_sys::openmpt_module_set_position_seconds(self.inner, seconds) }
    }

    /// Set approximate current song position.
    ///
    /// ### Parameters
    /// * `order` : Pattern order number to seek to.
    /// * `row` : Pattern row number to seek to.
    ///
    /// ### Returns
    /// Approximate new song position in seconds.
    ///
    /// ### Remarks
    /// If order or row are out of range, to position is not modified and the current position is returned.
    pub fn set_position_order_row(&mut self, order: i32, row: i32) -> c_double {
        // Returns current position on failure
        unsafe { openmpt_sys::openmpt_module_set_position_order_row(self.inner, order, row) }
    }

    /// Get current song position.
    ///
    /// ### Returns
    /// Current song position in seconds.
    pub fn get_position_seconds(&mut self) -> c_double {
        unsafe { openmpt_sys::openmpt_module_get_position_seconds(self.inner) }
    }

    /// Get the current order.
    ///
    /// ### Returns
    /// The current order at which the module is being played back.
    pub fn get_current_order(&mut self) -> i32 {
        unsafe { openmpt_sys::openmpt_module_get_current_order(self.inner) }
    }

    /// Get the current pattern.
    ///
    /// ### Returns
    /// The current pattern that is being played.
    pub fn get_current_pattern(&mut self) -> i32 {
        unsafe { openmpt_sys::openmpt_module_get_current_pattern(self.inner) }
    }

    /// Get the current row.
    ///
    /// ### Returns
    /// The current row at which the current pattern is being played.
    pub fn get_current_row(&mut self) -> i32 {
        unsafe { openmpt_sys::openmpt_module_get_current_row(self.inner) }
    }

    /// Get the current speed.
    ///
    /// ### Returns
    /// The current speed in ticks per row.
    pub fn get_current_speed(&mut self) -> i32 {
        unsafe { openmpt_sys::openmpt_module_get_current_speed(self.inner) }
    }

    /// Get the current tempo.
    ///
    /// ### Returns
    /// The current tempo in tracker units. The exact meaning of this value depends on the tempo mode being used.
    pub fn get_current_tempo(&mut self) -> i32 {
        unsafe { openmpt_sys::openmpt_module_get_current_tempo(self.inner) }
    }

    /// Get the current amount of playing channels.
    ///
    /// ### Returns
    /// The amount of sample channels that are currently being rendered.
    pub fn get_current_playing_channels(&mut self) -> i32 {
        unsafe { openmpt_sys::openmpt_module_get_current_playing_channels(self.inner) }
    }

    /// Get the approximate song duration.
    ///
    /// ### Returns
    /// Approximate duration of current sub-song in seconds.
    pub fn get_duration_seconds(&mut self) -> c_double {
        // Depends on the current subsong
        unsafe { openmpt_sys::openmpt_module_get_duration_seconds(self.inner) }
    }

    /// Get an approximate indication of the channel volume.
    ///
    /// ### Parameters
    /// * `channel_num` : The channel whose volume should be retrieved.
    ///
    /// ### Returns
    /// The approximate channel volume.
    ///
    /// ### Remarks
    /// The returned value is solely based on the note velocity and
    /// does not take the actual waveform of the playing sample into account.
    pub fn get_current_channel_vu_mono(&mut self, channel_num: i32) -> c_float {
        unsafe { openmpt_sys::openmpt_module_get_current_channel_vu_mono(self.inner, channel_num) }
    }

    /// Get an approximate indication of the channel volume on the front-left speaker.
    ///
    /// ### Parameters
    /// * `channel_num` : The channel whose volume should be retrieved.
    ///
    /// ### Returns
    /// The approximate channel volume.
    ///
    /// ### Remarks
    /// The returned value is solely based on the note velocity and
    /// does not take the actual waveform of the playing sample into account.
    pub fn get_current_channel_vu_left(&mut self, channel_num: i32) -> c_float {
        unsafe { openmpt_sys::openmpt_module_get_current_channel_vu_left(self.inner, channel_num) }
    }

    /// Get an approximate indication of the channel volume on the front-right speaker.
    ///
    /// ### Parameters
    /// * `channel_num` : The channel whose volume should be retrieved.
    ///
    /// ### Returns
    /// The approximate channel volume.
    ///
    /// ### Remarks
    /// The returned value is solely based on the note velocity and
    /// does not take the actual waveform of the playing sample into account.
    pub fn get_current_channel_vu_right(&mut self, channel_num: i32) -> c_float {
        unsafe { openmpt_sys::openmpt_module_get_current_channel_vu_right(self.inner, channel_num) }
    }

    /// Get an approximate indication of the channel volume on the rear-left speaker.
    ///
    /// ### Parameters
    /// * `channel_num` : The channel whose volume should be retrieved.
    ///
    /// ### Returns
    /// The approximate channel volume.
    ///
    /// ### Remarks
    /// The returned value is solely based on the note velocity and
    /// does not take the actual waveform of the playing sample into account.
    pub fn get_current_channel_vu_rear_left(&mut self, channel_num: i32) -> c_float {
        unsafe {
            openmpt_sys::openmpt_module_get_current_channel_vu_rear_left(self.inner, channel_num)
        }
    }

    /// Get an approximate indication of the channel volume on the rear-right speaker.
    ///
    /// ### Parameters
    /// * `channel_num` : The channel whose volume should be retrieved.
    ///
    /// ### Returns
    /// The approximate channel volume.
    ///
    /// ### Remarks
    /// The returned value is solely based on the note velocity and
    /// does not take the actual waveform of the playing sample into account.
    pub fn get_current_channel_vu_rear_right(&mut self, channel_num: i32) -> c_float {
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
