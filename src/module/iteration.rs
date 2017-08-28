use openmpt_sys;
use super::Module;
use super::mod_command::ModCommand;
use std::os::raw::c_int;

pub struct Pattern<'m> {
	module: &'m Module,
	num: i32,
}

pub struct Row<'m> {
	pattern: &'m Pattern<'m>,
	num: i32,
}

pub struct Cell<'m> {
	row: &'m Row<'m>,
	channel_num: i32,
}

impl Module {
	/// Get pattern at order position.
	///
	/// ### Parameters
	/// * `order_num` : The position from which the pattern should be retrieved.
	///
	/// ### Returns
	/// A Pattern wrapper for the pattern found at the given order position of the current sequence,
	/// or None if no such pattern exists.
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

	/// Get pattern by index.
	///
	/// ### Parameters
	/// * `pattern_num` : The index of the pattern that should be retrieved.
	///
	/// ### Returns
	/// A Pattern wrapper for the pattern, or None if no such pattern exists.
	pub fn get_pattern_by_number (&self, pattern_num: i32) -> Option<Pattern> {
		if pattern_num < 0 || pattern_num >= self.get_num_patterns() {
			None
		} else {
			Some(Pattern{ num : pattern_num, module: self })
		}
	}

	/// Get the number of distinct patterns for that module.
	///
	/// ### Returns
	/// The number of distinct patterns in the module.
	pub fn get_num_patterns (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_patterns(self.inner)
		}
	}

	/// Get the length of the order sequence for that module.
	///
	/// ### Returns
	/// The number of orders in the current sequence of the module.
	pub fn get_num_orders (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_orders(self.inner)
		}
	}

	/// Get the number of pattern channels.
	///
	/// ### Returns
	/// The number of pattern channels in the module. Not all channels do necessarily contain data.
	///
	/// ### Remarks
	/// The number of pattern channels is completely independent of the number of output channels.
	pub fn get_num_channels (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_channels(self.inner)
		}
	}

	/// Get the number of instruments.
	///
	/// ### Returns
	/// The number of instrument slots in the module.
	///
	/// ### Remarks
	/// Instruments are a layer on top of samples, and are not supported by all module formats.
	pub fn get_num_instruments (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_instruments(self.inner)
		}
	}

	/// Get the number of samples.
	///
	/// ### Returns
	/// The number of sample slots in the module.
	pub fn get_num_samples (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_samples(self.inner)
		}
	}

	/// Get the number of sub-songs.
	///
	/// ### Returns
	/// The number of sub-songs in the module.
	/// 
	/// This includes any "hidden" songs (songs that share the same sequence,
	/// but start at different order indices) and "normal" sub-songs
	/// or "sequences" (if the format supports them).
	pub fn get_num_subsongs (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_subsongs(self.inner)
		}
	}

	/// Get an instrument name.
	///
	/// ### Parameters
	/// * `instrument_num` : The index of the instrument whose name should be retrieved
	///
	/// ### Returns
	/// The instrument name.
	pub fn get_instrument_name (&self, instrument_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_instrument_name(self.inner, instrument_num)
		};
		
		opt_string.expect("Got null pointer instead of string")
	}
	
	/// Get a sample name.
	///
	/// ### Parameters
	/// * `sample_num` : The index of the sample whose name should be retrieved
	///
	/// ### Returns
	/// The sample name.
	pub fn get_sample_name (&self, sample_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_sample_name(self.inner, sample_num)
		};

		opt_string.expect("Got null pointer instead of string")
	}

	/// Get a channel name.
	///
	/// ### Parameters
	/// * `channel_num` : The index of the channel whose name should be retrieved
	///
	/// ### Returns
	/// The channel name.
	pub fn get_channel_name (&self, channel_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_channel_name(self.inner, channel_num)
		};

		opt_string.expect("Got null pointer instead of string")
	}

	/// Get a sub-song name.
	///
	/// ### Parameters
	/// * `subsong_num` : The index of the sub-song whose name should be retrieved
	///
	/// ### Returns
	/// The sub-song name.
	pub fn get_subsong_name (&self, subsong_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_subsong_name(self.inner, subsong_num)
		};

		opt_string.expect("Got null pointer instead of string")
	}
}

impl<'m> Pattern<'m> {
	/// Get pattern row by index.
	///
	/// ### Parameters
	/// * `row_num` : The index of the row that should be retrieved.
	///
	/// ### Returns
	/// A Row wrapper for the row, or None if no such row exists.
	pub fn get_row_by_number (&'m self, row_num: i32) -> Option<Row<'m>> {
		let pattern_num_rows = self.get_num_rows();

		assert_ne!(pattern_num_rows, 0); // Pattern does not exist
		
		if row_num < 0 || row_num >= pattern_num_rows {
			None
		} else {
			Some(Row{ num : row_num, pattern: self })
		}
	}

	/// Get name for this pattern/order.
	///
	/// ### Returns
	/// The pattern name.
	pub fn get_name (&self) -> String {
		// Order names apparently just gives you the name of the pattern
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_pattern_name(self.module.inner, self.num)
		};

		opt_string.expect("Got null pointer instead of string")
	}

	/// Get the number of rows for this pattern.
	///
	/// ### Returns
	/// The number of rows in the pattern.
	pub fn get_num_rows(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_pattern_num_rows(self.module.inner, self.num)
		}
	}
}

impl<'m> Row<'m> {
	/// Get pattern cell by pattern channel.
	///
	/// ### Parameters
	/// * `channel_num` : The index of the pattern channel at which the cell should be retrieved.
	///
	/// ### Returns
	/// A Cell wrapper for the cell, or None if the channel doesn't exist.
	pub fn get_cell_by_channel (&'m self, channel_num: i32) -> Option<Cell<'m>> {
		assert!(self.num < self.pattern.get_num_rows());
		assert!(self.num >= 0);

		let num_channels = self.pattern.module.get_num_channels();

		if channel_num < 0 || channel_num >= num_channels {
			None
		} else {
			Some(Cell{ row: self, channel_num: channel_num })
		}
	}
}

impl <'m> Cell<'m> {
	/// Get all of the cell's content as a ModCommand.
	///
	/// ### Returns
	/// A ModCommand containing the raw cell data as a tagged union, for easy pattern-matching.
	pub fn get_data(&self) -> Result<ModCommand, String> {
		ModCommand::new(
			self.get_data_by_command(ModuleCommandIndex::Note),
			self.get_data_by_command(ModuleCommandIndex::Instrument),
			self.get_data_by_command(ModuleCommandIndex::VolumeEffect),
			self.get_data_by_command(ModuleCommandIndex::Effect),
			self.get_data_by_command(ModuleCommandIndex::Volume),
			self.get_data_by_command(ModuleCommandIndex::Parameter)
		)
	}

	/// Get raw cell content.
	///
	/// ### Parameters
	/// * `command` : The cell index at which the data should be retrieved, from `ModuleCommandIndex`.
	///
	/// ### Returns
	/// The internal, raw cell data at the given command index.
	pub fn get_data_by_command(&self, command : ModuleCommandIndex) -> u8 {
		unsafe{
			openmpt_sys::openmpt_module_get_pattern_row_channel_command(
				self.row.pattern.module.inner,
				self.row.pattern.num,
				self.row.num,
				self.channel_num,
				command.value()
			)
		}
	}

	/// Get formatted (human-readable) cell content.
	///
	/// ### Parameters
	/// * `width` : The maximum number of characters the string should contain. 0 means no limit.
	/// * `pad` : 	If true, the string will be resized to the exact length provided in the width parameter.
	///
	/// ### Returns
	/// The formatted pattern data for that cell.
	pub fn get_formatted(&self, width: usize, pad: bool) -> String {
		let opt_string = get_string!({
			openmpt_sys::openmpt_module_format_pattern_row_channel(
				self.row.pattern.module.inner,
				self.row.pattern.num,
				self.row.num,
				self.channel_num,
				width,
				pad as c_int
			)
		});

		opt_string.expect("Got null pointer instead of string")
	}

	/// Get formatted (human-readable) cell content.
	///
	/// ### Parameters
	/// * `command` : The cell index at which the data should be retrieved, from `ModuleCommandIndex`.
	///
	/// ### Returns
	/// The formatted pattern data for that cell, at the given command index.
	pub fn get_formatted_by_command(&self, command: ModuleCommandIndex) -> String {
		let opt_string = get_string!({
			openmpt_sys::openmpt_module_format_pattern_row_channel_command(
				self.row.pattern.module.inner,
				self.row.pattern.num,
				self.row.num,
				self.channel_num,
				command.value()
			)
		});

		opt_string.expect("Got null pointer instead of string")
	}

	/// Get highlighting information for formatted cell content.
	///
	/// ### Parameters
	/// * `width` : The maximum number of characters the string should contain. 0 means no limit.
	/// * `pad` : 	If true, the string will be resized to the exact length provided in the width parameter.
	///
	/// ### Returns
	/// The highlighting string for the formatted pattern data as retrieved by `get_formatted` for that cell.
	pub fn get_highlight(&self, width: usize, pad: bool) -> String {
		let opt_string = get_string!({
			openmpt_sys::openmpt_module_highlight_pattern_row_channel(
				self.row.pattern.module.inner,
				self.row.pattern.num,
				self.row.num,
				self.channel_num,
				width,
				pad as c_int
			)
		});

		opt_string.expect("Got null pointer instead of string")
	}

	/// Get highlighting information for formatted pattern content.
	///
	/// ### Parameters
	/// * `command` : The cell index at which the data should be retrieved, from `ModuleCommandIndex`.
	///
	/// ### Returns
	/// The highlighting string for the formatted pattern data as retrieved by `get_formatted` for that cell, at the given command index.
	pub fn get_highlight_by_command(&self, command: ModuleCommandIndex) -> String {
		let opt_string = get_string!({
			openmpt_sys::openmpt_module_highlight_pattern_row_channel_command(
				self.row.pattern.module.inner,
				self.row.pattern.num,
				self.row.num,
				self.channel_num,
				command.value()
			)
		});

		opt_string.expect("Got null pointer instead of string")
	}
}

/// Parameter index to use with `get_data_by_command`,
/// `get_formatted_by_command` and `get_highlight_by_command`.
pub enum ModuleCommandIndex {
	Note,
	Instrument,
	VolumeEffect,
	Effect,
	Volume,
	Parameter,
}

impl ModuleCommandIndex {
	fn value(&self) -> c_int {
		match *self {
			ModuleCommandIndex::Note => 0,
			ModuleCommandIndex::Instrument => 1,
			ModuleCommandIndex::VolumeEffect => 2,
			ModuleCommandIndex::Effect => 3,
			ModuleCommandIndex::Volume => 4,
			ModuleCommandIndex::Parameter => 5,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::super::test_helper;

	#[test]
	fn unatco_iterative_reading() {
		iterative_reading("UNATCO.it");
	}

	fn iterative_reading(file_name : &str) {
		let module = test_helper::load_file_as_module(file_name).unwrap();
		let num_orders = module.get_num_orders();
		let num_channels = module.get_num_channels();

		for order_num in 0..num_orders {
			let pattern = module.get_pattern_by_order(order_num).unwrap();
			let num_rows = pattern.get_num_rows();

			println!("Checking pattern #{} ({} rows, {} channels)", order_num, num_rows, num_channels);

			for row_num in 0..num_rows {
				let row = pattern.get_row_by_number(row_num).unwrap();
				let mut row_string = String::new();

				for channel_num in 0..num_channels {
					let cell = row.get_cell_by_channel(channel_num).unwrap();
					assert!(cell.get_data().is_ok());

					if channel_num != 0 { row_string.push_str("|"); }
					row_string.push_str(cell.get_formatted(0, false).as_str());
				}
				//println!("{}", row_string);
			}
		}
	}
}