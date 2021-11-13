use log::*;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::*;
use windows::Win32::System::EventLog::*;

pub struct EventlogLogger {
    pub name: Vec<u16>, // 保持しておかないとRegisterEventSourceの名前のライフタイムが合わない
    handle: EventSourceHandle,
}

impl EventlogLogger {
    pub fn new(name: &str) -> Self {
        let localhost = PWSTR::default();
        let mut wide_name = to_wide(name);
        let event_source_handle;
        unsafe {
            event_source_handle = RegisterEventSourceW(localhost, PWSTR(wide_name.as_mut_ptr()))
        };
        EventlogLogger {
            name: wide_name,
            handle: event_source_handle,
        }
    }
}

trait ToWin32<T> {
    fn to_win32(&self) -> T;
}

impl ToWin32<REPORT_EVENT_TYPE> for Level {
    fn to_win32(&self) -> REPORT_EVENT_TYPE {
        match *self {
            Level::Error => EVENTLOG_ERROR_TYPE,
            Level::Warn => EVENTLOG_WARNING_TYPE,
            Level::Info => EVENTLOG_INFORMATION_TYPE,
            Level::Debug => EVENTLOG_INFORMATION_TYPE,
            Level::Trace => EVENTLOG_INFORMATION_TYPE,
        }
    }
}

impl Log for EventlogLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let mut wide_final_messeage = to_wide(&format!("{}", record.args()));

        let category = 0;
        let event_id = 0;
        let strings = [PWSTR(wide_final_messeage.as_mut_ptr())];
        let user_sid = PSID::default();
        let num_strings = strings.len().try_into().unwrap();
        let data_size = 0;
        let data = std::ptr::null();
        unsafe {
            ReportEventW(
                HANDLE(self.handle.0),
                record.level().to_win32(),
                category,
                event_id,
                user_sid,
                num_strings,
                data_size,
                strings.as_ptr(),
                data,
            );
        };
    }

    fn flush(&self) {}
}

impl Drop for EventlogLogger {
    fn drop(&mut self) {
        let result;
        unsafe { result = DeregisterEventSource(self.handle) };
        println!("EventlogLogger Drop Success?: {}", result.as_bool());
    }
}

fn to_wide(str: &str) -> Vec<u16> {
    OsString::from(str)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>()
}
