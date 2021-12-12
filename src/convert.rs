use windows::Win32::Foundation::*;

pub trait FromWin32<T> {
    fn from_win32(&self) -> T;
}

impl FromWin32<String> for PSTR {
    fn from_win32(&self) -> String {
        let base_ptr = self.0;

        let mut size = 0;
        let mut current_ptr = base_ptr;
        unsafe {
            while *current_ptr != 0 {
                size += 1;
                current_ptr = current_ptr.add(1);
            }

            let vec = std::slice::from_raw_parts(base_ptr, size);
            String::from_utf8_lossy(vec).into_owned()
        }
    }
}
