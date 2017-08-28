use openmpt_sys;
use std::ptr;
use std::os::raw::*;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

pub trait ModuleStream {
	fn get_file_callbacks() -> openmpt_sys::openmpt_stream_callbacks;
	unsafe extern "C" fn openmpt_read(stream: *mut c_void, dst: *mut c_void, bytes: usize) -> usize;
}

pub trait SeekableStream : ModuleStream {
	fn get_file_callbacks() -> openmpt_sys::openmpt_stream_callbacks;
	unsafe extern "C" fn openmpt_seek(stream: *mut c_void, offset: i64, whence: c_int) -> c_int;
	unsafe extern "C" fn openmpt_tell(stream: *mut c_void) -> i64;
}

impl<T> ModuleStream for T where T:Read {
	unsafe extern "C" fn openmpt_read(stream: *mut c_void, dst: *mut c_void, bytes: usize) -> usize {
		let mut stream_source: &mut T = &mut *(stream as *mut T);
		let mut buf = vec![0;bytes];

		//println!("Read {} bytes", bytes);
		let read_result = stream_source.read(&mut buf);
		//println!("Read result : {:?}", read_result);

		match read_result {
			Ok(0) => 0,
			Ok(n) => {
				ptr::copy(buf.as_ptr() as *const c_void, dst, n);
				n
			},
			Err(_) => 0,
		}
	}

	fn get_file_callbacks() -> openmpt_sys::openmpt_stream_callbacks where T:Read {
		openmpt_sys::openmpt_stream_callbacks {
			read: Some(Self::openmpt_read),
			seek: None::<unsafe extern "C" fn(*mut c_void, i64, i32) -> i32>,
			tell: None::<unsafe extern "C" fn(*mut c_void) -> i64>,
		}
	}
}

impl<T> SeekableStream for T where T:Read+Seek {
	unsafe extern "C" fn openmpt_seek(stream: *mut c_void, offset: i64, whence: c_int) -> c_int {
		let mut stream_source: &mut T = &mut *(stream as *mut T);
		
		let whence = match whence {
			0 => SeekFrom::Start(offset as u64),
			1 => SeekFrom::Current(offset),
			2 => SeekFrom::End(offset),
			_ => return -1,
		};

		match stream_source.seek(whence) {
			Ok(_) => 0,
			Err(_) => -1,
		}
	}

	unsafe extern "C" fn openmpt_tell(stream: *mut c_void) -> i64 {
		let mut stream_source: &mut T = &mut *(stream as *mut T);
		match stream_source.seek(SeekFrom::Current(0)) {
			Ok(pos) => pos as i64,
			Err(_) => -1,
		}
	}

	fn get_file_callbacks() -> openmpt_sys::openmpt_stream_callbacks where T:Read+Seek {
		openmpt_sys::openmpt_stream_callbacks {
			read: Some(Self::openmpt_read),
			seek: Some(Self::openmpt_seek),
			tell: Some(Self::openmpt_tell),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::File;

	#[test]
	fn file_counts_as_seekable_stream() {
		let f = File::open("empty_module.xm").expect("unable to open file");
		check_callbacks_seekable(&f);
	}

	#[test]
	fn stdin_counts_as_non_seekable_stream() {
		let stdin = ::std::io::stdin();
		check_callbacks_non_seekable(&stdin);
	}

	fn check_callbacks_seekable<T:SeekableStream> (stream:&T) {
		let callbacks = <T as SeekableStream>::get_file_callbacks();

		assert!(callbacks.read.is_some());
		assert!(callbacks.seek.is_some());
		assert!(callbacks.tell.is_some());
	}

	fn check_callbacks_non_seekable<T:ModuleStream> (stream:&T) {
		let callbacks = T::get_file_callbacks();

		assert!(callbacks.read.is_some());
		assert!(callbacks.seek.is_none());
		assert!(callbacks.tell.is_none());
	}
}