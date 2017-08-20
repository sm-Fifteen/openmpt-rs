use openmpt_sys;

pub enum Logger {
    StdErr,
    None
}

impl Logger {
    pub(super) fn log_func(&self) -> openmpt_sys::openmpt_log_func {
        match *self {
            Logger::StdErr => Some(openmpt_sys::openmpt_log_func_default),
            Logger::None => Some(openmpt_sys::openmpt_log_func_silent)
        }
    }
}