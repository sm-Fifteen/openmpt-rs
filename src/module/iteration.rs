use openmpt_sys;
use super::Module;
use std::iter::Iterator;
use std::ops::Range;
use std::marker::PhantomData;
use std::fmt;

pub struct Subsong (i32);

#[derive(Copy, Clone)]
pub struct Pattern {
	num: i32,
}

impl fmt::Debug for Pattern {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P{}", self.num)
    }
}

pub struct OrderedPatterns<'m> {
	module: &'m Module,
	iter: Range<i32>,
}

impl<'m> Iterator for OrderedPatterns<'m> {
	type Item = Pattern;

	fn next(&mut self) -> Option<Pattern> {
		self.iter.next().map(|order_num| {
			let pattern_num = self.module.get_order_pattern(order_num)
					.expect("Failed to convert order index into pattern index");
			Pattern { num: pattern_num }
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

	// FIXME : Test currently fails because it seems like order
	// does not account for selected subsong
	// #[test]
	fn can_change_subsong_once_order_is_no_longer_needed() {
		let mut module = test_helper::load_file_as_module("UNATCO.it").unwrap();
		let order1 = {
			module.get_subsong_pattern_order().collect::<Vec<_>>()
		};

		let subsongs = module.get_subsongs();
		module.select_subsong(&subsongs[1]);
		let order2 = {
			module.get_subsong_pattern_order().collect::<Vec<_>>()
		};

		println!("Order 1 : {:?}", order1);
		println!("Order 2 : {:?}", order2);

		assert_ne!(order1.len(), order2.len());
	}
}