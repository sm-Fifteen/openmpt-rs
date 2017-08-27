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
		if pattern_num < 0 || pattern_num >= self.get_num_patterns() {
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

	pub fn get_num_samples (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_samples(self.inner)
		}
	}

	pub fn get_num_subsongs (&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_num_subsongs(self.inner)
		}
	}

	pub fn get_instrument_name (&self, instrument_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_instrument_name(self.inner, instrument_num)
		};
		
		opt_string.expect("Got null pointer instead of string")
	}
	
	pub fn get_sample_name (&self, sample_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_sample_name(self.inner, sample_num)
		};

		opt_string.expect("Got null pointer instead of string")
	}

	pub fn get_channel_name (&self, channel_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_channel_name(self.inner, channel_num)
		};

		opt_string.expect("Got null pointer instead of string")
	}

	pub fn get_subsong_name (&self, subsong_num: i32) -> String {
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_subsong_name(self.inner, subsong_num)
		};

		opt_string.expect("Got null pointer instead of string")
	}
}

impl<'m> Pattern<'m> {
	pub fn get_row_by_number (&'m self, row_num: i32) -> Option<Row<'m>> {
		let pattern_num_rows = self.get_num_rows();

		assert_ne!(pattern_num_rows, 0); // Pattern does not exist
		
		if row_num < 0 || row_num >= pattern_num_rows {
			None
		} else {
			Some(Row{ num : row_num, pattern: self })
		}
	}

	pub fn get_name (&self) -> String {
		// Order names apparently just gives you the name of the pattern
		let opt_string = get_string!{
			openmpt_sys::openmpt_module_get_pattern_name(self.module.inner, self.num)
		};

		opt_string.expect("Got null pointer instead of string")
	}

	pub fn get_num_rows(&self) -> i32 {
		unsafe {
			openmpt_sys::openmpt_module_get_pattern_num_rows(self.module.inner, self.num)
		}
	}
}

impl<'m> Row<'m> {
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