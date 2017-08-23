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

impl<'m> fmt::Debug for Pattern<'m> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P{}", self.num)
    }
}

pub struct OrderedPatterns<'m> {
	module: &'m Module,
	iter: Range<i32>,
}

impl<'m> Iterator for OrderedPatterns<'m> {
	type Item = Pattern<'m>;

	fn next(&mut self) -> Option<Pattern<'m>> {
		self.iter.next().map(|order_num| {
			let pattern_num = self.module.get_order_pattern(order_num)
					.expect("Failed to convert order index into pattern index");
			Pattern { module: self.module, num: pattern_num }
		})
	}
}

impl Module {
	fn get_order_pattern (&self, order_num: i32) -> Option<i32> {
		let pattern_num = unsafe {
			openmpt_sys::openmpt_module_get_order_pattern(self.inner, order_num)
		};

		if pattern_num < 0 { None } else { Some(pattern_num) }
	}

	pub fn get_pattern_order<'a> (&'a self) -> OrderedPatterns {
		let num_order = unsafe {
			openmpt_sys::openmpt_module_get_num_orders(self.inner)
		};

		OrderedPatterns {
			module: self,
			iter: (0..num_order),
		}
	}
}

impl<'m> Pattern<'m> {
	pub fn get_rows(self) -> Range<i32> {
		let pattern_num_rows = unsafe {
			openmpt_sys::openmpt_module_get_pattern_num_rows(self.module.inner, self.num)
		};

		assert_ne!(pattern_num_rows, 0); // Pattern does not exist
		
		(0..pattern_num_rows)
	}

	pub fn get_row_channel_command(self, row: i32, channel: i32) -> ModCommand {
		unimplemented!()
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use super::super::test_helper;

	#[test]
	fn dummy_file_has_valid_order() {
		let module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		let order = module.get_pattern_order();
		order.collect::<Vec<_>>();
	}
}