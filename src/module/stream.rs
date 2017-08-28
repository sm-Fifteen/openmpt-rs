use openmpt_sys;
use std::ptr;
use std::os::raw::*;
use std::io::Read;
use std::io::Seek;

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
		let mut buf = Vec::with_capacity(bytes);

		match stream_source.read(&mut buf) {
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


#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::File;

	#[test]
	fn stdin_counts_as_non_seekable_stream() {
		let stdin = ::std::io::stdin();
		check_callbacks_non_seekable(&stdin);
	}

	fn check_callbacks_seekable<T:Read+Seek> (stream:&T) {
		let callbacks = T::get_file_callbacks();

		assert!(callbacks.read.is_some());
		assert!(callbacks.seek.is_some());
		assert!(callbacks.tell.is_some());
	}

	fn check_callbacks_non_seekable<T:Read> (stream:&T) {
		let callbacks = T::get_file_callbacks();

		assert!(callbacks.read.is_some());
		assert!(callbacks.seek.is_none());
		assert!(callbacks.tell.is_none());
	}
}