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

/// やや泥臭いApacheと同じ方法できれいにイベントログを出している。
/// 9個のパラメータを使い切らないので末尾にスペースが入ってしまうが仕方ないか…
/// -`HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\EventLog\Application\${名前}`
///   - `EventMessageFile` `REG_EXPAND_SZ` `%SystemRoot%\System32\netmsg.dll`
///   - `TypesSupported` `REG_DWORD` `7`(ERROR | WARN | INFOの値)
/// ### 参照
/// - https://www.clear-code.com/blog/2015/9/10.html
/// - https://github.com/apache/httpd/blob/2.4.16/server/mpm/winnt/nt_eventlog.c
impl Log for EventlogLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true // ここはfernが制御
    }

    fn log(&self, record: &Record) {
        let mut wide_level = to_wide(&format!("[{}]", record.level()));
        let mut wide_final_messeage = to_wide(&format!("{}", record.args()));

        let category = 0;
        // netmsg.dllの一般的ログメッセージ形式のIDは`3299`
        // テンプレート: '%1 %2 %3 %4 %5 %6 %7 %8 %9'
        let event_id = 3299;
        let strings = [
            PWSTR(wide_level.as_mut_ptr()),
            PWSTR(wide_final_messeage.as_mut_ptr()),
            PWSTR(std::ptr::null_mut()),
            PWSTR(std::ptr::null_mut()),
            PWSTR(std::ptr::null_mut()),
            PWSTR(std::ptr::null_mut()),
            PWSTR(std::ptr::null_mut()),
            PWSTR(std::ptr::null_mut()),
            PWSTR(std::ptr::null_mut()),
        ];
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
