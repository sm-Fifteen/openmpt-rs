use openmpt_sys;
use std::ptr;
use std::os::raw::*;

pub enum Logger<T> {
    StdErr(T),
    None
}

impl<T> Logger<T> {
    pub(super) fn log_func(&self) -> openmpt_sys::openmpt_log_func {
        match *self {
            Logger::StdErr(_) => Some(openmpt_sys::openmpt_log_func_default),
            Logger::None => Some(openmpt_sys::openmpt_log_func_silent)
        }
    }

    pub(super) fn logging_context(&self) -> *mut c_void {
        match *self {
            Logger::StdErr(_) => ptr::null_mut(),
            Logger::None => ptr::null_mut()
        }
    }
}