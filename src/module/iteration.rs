use openmpt_sys;
use super::Module;

pub struct Subsong (i32);

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
}