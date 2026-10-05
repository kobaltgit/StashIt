// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
fn is_another_instance_running() -> bool {
    const ERROR_ALREADY_EXISTS: u32 = 183;
    extern "system" {
        fn CreateMutexW(
            lp_mutex_attributes: *const std::ffi::c_void,
            b_initial_owner: i32,
            lp_name: *const u16,
        ) -> *mut std::ffi::c_void;
        fn GetLastError() -> u32;
    }

    let mutex_name: Vec<u16> = "Global\\StashIt_SingleInstance_Mutex\0"
        .encode_utf16()
        .collect();
    unsafe {
        let handle = CreateMutexW(std::ptr::null(), 1, mutex_name.as_ptr());
        if handle.is_null() || GetLastError() == ERROR_ALREADY_EXISTS {
            return true;
        }
    }
    false
}

fn main() {
    #[cfg(windows)]
    if is_another_instance_running() {
        return;
    }

    stashit_lib::run();
}