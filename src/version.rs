use openmpt_sys;

#[derive(Debug)]
pub struct CoreVersion (u8,	u8,	u8,	u8);

#[derive(Debug)]
pub struct LibraryVersion (u8, u8, u16);

pub fn get_core_version () -> CoreVersion {
	let version_number = unsafe {
		openmpt_sys::openmpt_get_core_version()
	};

	CoreVersion (
		(version_number >> 24) as u8,
		(version_number >> 16) as u8,
		(version_number >> 8)  as u8,
		version_number  as u8
	)
}

pub fn get_library_version () -> LibraryVersion {
	let version_number = unsafe {
		openmpt_sys::openmpt_get_library_version()
	};

	LibraryVersion (
		(version_number >> 24) as u8,
		(version_number >> 16)  as u8,
		version_number as u16
	)
}

#[cfg(test)]
mod tests {
	#[test]
	fn show_version_numbers() {
		let x = super::get_core_version();
		println!("Core Version : {:?}", &x);
		let y = super::get_library_version();
		println!("Lib Version : {:?}", &y);
	}
}