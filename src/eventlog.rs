use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::*;
use windows::Win32::System::EventLog::*;

#[derive(Debug)]
pub enum LogLevel {
    ERROR,
    WARN,
    INFO,
    DEBUG,
}

impl LogLevel {
    fn as_win32(&self) -> REPORT_EVENT_TYPE {
        match *self {
            LogLevel::ERROR => EVENTLOG_ERROR_TYPE,
            LogLevel::WARN => EVENTLOG_WARNING_TYPE,
            LogLevel::INFO => EVENTLOG_INFORMATION_TYPE,
            LogLevel::DEBUG => EVENTLOG_INFORMATION_TYPE,
        }
    }
}

impl std::convert::TryFrom<&str> for LogLevel {
    type Error = &'static str;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_ascii_uppercase() {
            name if name == LogLevel::ERROR.to_string() => Ok(LogLevel::ERROR),
            name if name == LogLevel::WARN.to_string() => Ok(LogLevel::WARN),
            name if name == LogLevel::INFO.to_string() => Ok(LogLevel::INFO),
            name if name == LogLevel::DEBUG.to_string() => Ok(LogLevel::DEBUG),
            _ => Err("unknown type"),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Logger {
    name: Vec<u16>, // 保持しておかないとRegisterEventSourceの名前のライフタイムが合わない
    handle: EventSourceHandle,
    level: LogLevel,
}

impl Logger {
    pub fn new(name: &str, level: LogLevel) -> Logger {
        let localhost = PWSTR::default();
        let mut wide_name = to_wide(name);
        let event_source_handle;
        unsafe {
            event_source_handle = RegisterEventSourceW(localhost, PWSTR(wide_name.as_mut_ptr()))
        };
        Logger {
            name: wide_name,
            handle: event_source_handle,
            level: level,
        }
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::ERROR, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::WARN, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::INFO, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::DEBUG, message);
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let category = 0;
        let event_id = 0;
        let user_sid = PSID::default();
        let strings = [PWSTR(
            to_wide(&format!("[{}] {}", level, message)).as_mut_ptr(),
        )];
        let data_size = 0;
        let data = std::ptr::null();
        unsafe {
            ReportEventW(
                HANDLE(self.handle.0),
                level.as_win32(),
                category,
                event_id,
                user_sid,
                strings.len().try_into().unwrap(),
                data_size,
                strings.as_ptr(),
                data,
            );
        };
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        let result;
        unsafe { result = DeregisterEventSource(self.handle) };
        println!("Logger Drop Success?: {}", result.as_bool());
    }
}

fn to_wide(str: &str) -> Vec<u16> {
    OsString::from(str)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>()
}
