use super::convert::*;
use windows::Win32::{Foundation::*, Security::SC_HANDLE, System::Services::*};

pub struct Service {
    sc_manager_handle: SC_HANDLE,
    service_handle: SC_HANDLE,
}

impl Service {
    pub fn list() {
        unsafe {
            let sc_manager_handle = OpenSCManagerA(
                PSTR::default(),
                PSTR::default(),
                SC_MANAGER_CONNECT | SC_MANAGER_ENUMERATE_SERVICE,
            );
            let mut services_returned = 0;
            let mut resume_handle = 0;
            let mut bytes_needed = 0;
            let mut result: [ENUM_SERVICE_STATUSA; 1000] = [ENUM_SERVICE_STATUSA::default(); 1000];
            EnumServicesStatusA(
                sc_manager_handle,
                SERVICE_WIN32,
                SERVICE_STATE_ALL,
                &mut result as *mut _,
                std::mem::size_of::<[ENUM_SERVICE_STATUSA; 1000]>()
                    .try_into()
                    .unwrap(),
                &mut bytes_needed,
                &mut services_returned,
                &mut resume_handle,
            );

            println!(
                "{:?}, {:?}, {:?}",
                services_returned, resume_handle, bytes_needed,
            );

            for service in &result[..services_returned.try_into().unwrap()] {
                println!(
                    "{}, {:?}",
                    service.lpServiceName.from_win32(),
                    service.ServiceStatus,
                );
            }
        }
    }

    pub fn new(name: &str) -> Option<Self> {
        unsafe {
            let sc_manager_handle =
                OpenSCManagerA(PSTR::default(), PSTR::default(), SC_MANAGER_CONNECT);
            match OpenServiceA(sc_manager_handle, name, SERVICE_QUERY_STATUS) {
                SC_HANDLE(0) => None,
                service_handle => Some(Service {
                    sc_manager_handle,
                    service_handle,
                }),
            }
        }
    }

    pub fn is_running(&self) -> bool {
        let mut status = SERVICE_STATUS::default();
        unsafe {
            QueryServiceStatus(self.service_handle, &mut status);
        }
        status.dwCurrentState == SERVICE_RUNNING
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        unsafe {
            let drop1 = CloseServiceHandle(self.service_handle);
            let drop2 = CloseServiceHandle(self.sc_manager_handle);
            println!("Service dropped({:?}, {:?})", drop1, drop2);
        }
    }
}
