//! Windowsイベントログに出力するロガー実装です。
//!
//! ## 仕組み
//! - レガシーではありますがツールを必要としないためEvent Logging API(not Windows Event Log API)を使用しています。
//! - イベントログにきれいに出力するためにはログメッセージテンプレートの定義が必要ですがこれを省略するため、Windowsに同梱されている`netmsg.dll`を流用します。
//! - `netmsg.dll`のイベントID`3299`は9個のパラメータをスペースで区切った汎用的なテンプレートです。
//!   - テンプレート: `%1 %2 %3 %4 %5 %6 %7 %8 %9`
//!   - 9個のパラメータを使い切らないので末尾にスペースが入ってしまいますがこれは仕方がないものとします。
//!
//! ## 事前に設定が必要なレジストリ
//! -`HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\EventLog\Application\${名前}`
//!   - `EventMessageFile` `REG_EXPAND_SZ` `%SystemRoot%\System32\netmsg.dll`
//!   - `TypesSupported` `REG_DWORD` `7`(ERROR | WARN | INFOの値)
//!
//! ## 参照
//! - https://www.clear-code.com/blog/2015/9/10.html
//! - https://docs.microsoft.com/en-us/windows/win32/eventlog/event-sources
//! - https://github.com/apache/httpd/blob/2.4.16/server/mpm/winnt/nt_eventlog.c

use log::*;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::*;
use windows::Win32::System::EventLog::*;

pub struct EventlogLogger {
    /// イベントログのアプリケーション名です。
    /// 保持しておかないとRegisterEventSourceで指定したポインタが消えるためここで保持します
    #[allow(dead_code)]
    wide_app_name: Vec<u16>,
    /// イベントログのソースハンドルです。
    handle: EventSourceHandle,
}

impl EventlogLogger {
    pub fn new(app_name: &str) -> Self {
        let localhost = PWSTR::default();
        let mut wide_app_name = app_name.to_win32();
        let handle;
        unsafe { handle = RegisterEventSourceW(localhost, PWSTR(wide_app_name.as_mut_ptr())) };
        EventlogLogger {
            wide_app_name,
            handle,
        }
    }
}

trait ToWin32<T> {
    fn to_win32(&self) -> T;
}

impl ToWin32<REPORT_EVENT_TYPE> for Level {
    /// log crateのレベルをイベントログの対応するレベルに変換します。
    /// Debug以下のレベルはイベントログには存在しないため、Infoレベルとして扱います。
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

impl<T> ToWin32<Vec<u16>> for T
where
    T: AsRef<str>,
{
    fn to_win32(&self) -> Vec<u16> {
        OsString::from(self.as_ref())
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<u16>>()
    }
}

impl Log for EventlogLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true // ここはfernが制御するので常にtrue
    }

    fn log(&self, record: &Record) {
        let mut level_key = "Log level:".to_win32();
        let mut level_value = format!("{},", record.level()).to_win32();
        let mut thread_key = "Thread:".to_win32();
        let mut thread_value = format!("{:?},", std::thread::current().id()).to_win32();
        let mut message_key = "Message:".to_win32();
        let mut message_value = format!("{}", record.args()).to_win32();

        let category = 0;
        let event_id = 3299;
        let strings: [PWSTR; 9] = [
            PWSTR(level_key.as_mut_ptr()),
            PWSTR(level_value.as_mut_ptr()),
            PWSTR(thread_key.as_mut_ptr()),
            PWSTR(thread_value.as_mut_ptr()),
            PWSTR(message_key.as_mut_ptr()),
            PWSTR(message_value.as_mut_ptr()),
            PWSTR::default(),
            PWSTR::default(),
            PWSTR::default(),
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

    fn flush(&self) {
        // イベントログにflushしなければならない処理はないはず
    }
}

impl Drop for EventlogLogger {
    fn drop(&mut self) {
        let result;
        unsafe { result = DeregisterEventSource(self.handle) };
        println!("EventlogLogger Drop Success?: {}", result.as_bool());
    }
}
