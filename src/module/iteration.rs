use openmpt_sys;
use super::Module;
use std::iter::Iterator;
use std::ops::Range;
use std::marker::PhantomData;

pub struct Subsong (i32);

#[derive(Copy, Clone)]
pub struct Pattern<'m, Module: 'm> {
	phantom: PhantomData<&'m Module>,
	num: i32,
}

pub struct OrderedPatterns<'a> {
	module: &'a Module,
	iter: Range<i32>,
}

impl<'m> Iterator for OrderedPatterns<'m> {
	type Item = Pattern<'m, Module>;

	fn next(&mut self) -> Option<Pattern<'m, Module>> {
		self.iter.next().map(|order_num| {
			let pattern_num = self.module.get_order_pattern(order_num)
					.expect("Failed to convert order index into pattern index");
			Pattern { num: pattern_num, phantom: PhantomData }
		})
	}
}

impl Module {
	pub fn get_subsongs(&self) -> Vec<Subsong> {
		let num_subsongs = unsafe {
			openmpt_sys::openmpt_module_get_num_subsongs(self.inner)
		};
		assert_ne!(num_subsongs, 0);

		let mut list_subsongs:Vec<Subsong> = Vec::with_capacity(num_subsongs as usize);

		for i in 0..(num_subsongs) {
			list_subsongs.push(Subsong(i));
		}
		assert_eq!(list_subsongs.len(), num_subsongs as usize);

		return list_subsongs;
	}

	pub fn select_subsong(&mut self, subsong: &Subsong) -> bool {
		let return_code = unsafe {
			openmpt_sys::openmpt_module_select_subsong(self.inner, subsong.0)
		};

		if return_code == 0 { false } else { true }
	}

	fn get_order_pattern (&self, order_num: i32) -> Option<i32> {
		let pattern_num = unsafe {
			openmpt_sys::openmpt_module_get_order_pattern(self.inner, order_num)
		};

		if pattern_num < 0 { None } else { Some(pattern_num) }
	}

	pub fn get_subsong_pattern_order<'a> (&'a self) -> OrderedPatterns {
		let subsong_num_order = unsafe {
			openmpt_sys::openmpt_module_get_num_orders(self.inner)
		};

		OrderedPatterns {
			module: self,
			iter: (0..subsong_num_order),
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use super::super::test_helper;

	#[test]
	fn unatco_can_change_subsong() {
		let mut module = test_helper::load_file_as_module("UNATCO.it").unwrap();
		let subsongs = module.get_subsongs();

		assert_eq!(subsongs.len(), 5); // Main, Game over, Dialogue /w intro, Combat, Dialogue loop
		
		for song in subsongs {
			assert!(module.select_subsong(&song));
		}
	}

	#[test]
	fn dummy_file_has_valid_order() {
		let module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		let order = module.get_subsong_pattern_order();
		order.collect::<Vec<_>>();
	}

	#[test]
	fn can_change_subsong_once_order_is_no_longer_needed() {
		let mut module = test_helper::load_file_as_module("UNATCO.it").unwrap();
		let order1 = module.get_subsong_pattern_order();
		let order1 = order1.collect::<Vec<_>>();

		let subsongs = module.get_subsongs();
		// FIXME : Test fails to compile here because the
		// patterns in order1 still hold the mutable ref's lifetime
		module.select_subsong(&subsongs[1]);
		let order2 = module.get_subsong_pattern_order();
		let order2 = order2.collect::<Vec<_>>();

		assert_ne!(order1.len(), order2.len());
	}
}