use openmpt_sys;
use super::Module;
use std::iter::Iterator;
use std::ops::Range;
use super::mod_command::ModCommand;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Pattern<'m> {
	module: &'m Module,
	num: i32,
}

pub struct Row<'m> {
	pattern: Pattern<'m>,
	num: i32,
}

impl Module {
	pub fn get_pattern_by_order(&self, order_num: i32) -> Option<Pattern> {
		let pattern_num = unsafe {
			openmpt_sys::openmpt_module_get_order_pattern(self.inner, order_num)
		};

		if pattern_num < 0 {
			None
		} else {
			Some(Pattern{ num : pattern_num, module: self })
		}
	}

	pub fn get_pattern_by_number (&self, pattern_num: i32) -> Option<Pattern> {
		if pattern_num >= self.get_num_patterns() {
			None
		} else if pattern_num < 0 {
			None
		} else {
			Some(Pattern{ num : pattern_num, module: self })
		}
	}

	pub fn get_num_patterns (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_patterns(self.inner)
		}
	}

	pub fn get_num_orders (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_orders(self.inner)
		}
	}

	pub fn get_num_channels (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_channels(self.inner)
		}
	}

	pub fn get_num_instruments (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_instruments(self.inner)
		}
	}
}

impl<'m> Pattern<'m> {
	pub fn get_row_by_number (self, row_num: i32) -> Option<Row<'m>> {
		let pattern_num_rows = self.get_num_rows();

		assert_ne!(pattern_num_rows, 0); // Pattern does not exist
		
		if row_num >= pattern_num_rows {
			None
		} else if row_num < 0 {
			None
		} else {
			Some(Row{ num : row_num, pattern: self })
		}
	}

	pub fn get_num_rows(self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_pattern_num_rows(self.module.inner, self.num)
		}
	}
}

impl<'m> Row<'m> {
	pub fn get_command_by_channel (&self, channel_num: i32) -> Option<ModCommand> {
		assert!(self.num < self.pattern.get_num_rows());
		assert!(self.num >= 0);

		let num_channels = self.pattern.module.get_num_channels();

		if channel_num < 0 || channel_num >= num_channels {
			return None
		}

		// TODO : Add aliases to the macros in openmpt_sys
		Some(ModCommand::new(
			self.get_command(channel_num, 0), // Note
			self.get_command(channel_num, 1), // Instrument
			self.get_command(channel_num, 2), // Vol effect
			self.get_command(channel_num, 3), // Effect
			self.get_command(channel_num, 4), // Volume
			self.get_command(channel_num, 5), // Parameter
		))
	}

	fn get_command(&self, channel_num: i32, command_id: ::std::os::raw::c_int) -> u8 {
		unsafe{
			openmpt_sys::openmpt_module_get_pattern_row_channel_command(
				self.pattern.module.inner,
				self.pattern.num,
				self.num,
				channel_num,
				command_id
			)
		}
	}
} 


#[cfg(test)]
mod tests {
	use super::*;
	use super::super::test_helper;

	#[test]
	fn dummy_file_has_valid_order() {
		//let module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		//let order = module.get_pattern_order();
		//order.collect::<Vec<_>>();
	}
}