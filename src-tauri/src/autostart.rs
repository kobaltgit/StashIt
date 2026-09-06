#[cfg(windows)]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegSetValueExW, HKEY_CURRENT_USER, KEY_SET_VALUE,
        REG_SZ,
    };

    let subkey: Vec<u16> = OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let app_name: Vec<u16> = OsStr::new("StashIt")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut hkey = std::ptr::null_mut();
    unsafe {
        let status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            subkey.as_ptr(),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        );
        if status != 0 {
            return Err(format!("Не удалось открыть реестр: код {}", status));
        }

        if enabled {
            let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
            let exe_path = current_exe.to_str().ok_or("Неверный путь к exe")?;
            let wide_exe: Vec<u16> = OsStr::new(&format!("\"{}\"", exe_path))
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let res = RegSetValueExW(
                hkey,
                app_name.as_ptr(),
                0,
                REG_SZ,
                wide_exe.as_ptr() as *const u8,
                (wide_exe.len() * 2) as u32,
            );
            RegCloseKey(hkey);
            if res != 0 {
                return Err(format!("Не удалось записать автозапуск: код {}", res));
            }
        } else {
            let res = RegDeleteValueW(hkey, app_name.as_ptr());
            RegCloseKey(hkey);
            // 2 = ERROR_FILE_NOT_FOUND, что допустимо при удалении
            if res != 0 && res != 2 {
                return Err(format!("Не удалось удалить автозапуск: код {}", res));
            }
        }
    }

    Ok(())
}

#[cfg(windows)]
pub fn is_autostart_enabled() -> bool {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER, KEY_QUERY_VALUE,
    };

    let subkey: Vec<u16> = OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let app_name: Vec<u16> = OsStr::new("StashIt")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut hkey = std::ptr::null_mut();
    unsafe {
        let status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            subkey.as_ptr(),
            0,
            KEY_QUERY_VALUE,
            &mut hkey,
        );
        if status != 0 {
            return false;
        }

        let res = RegQueryValueExW(
            hkey,
            app_name.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        RegCloseKey(hkey);
        res == 0
    }
}

#[cfg(not(windows))]
pub fn set_autostart(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(not(windows))]
pub fn is_autostart_enabled() -> bool {
    false
}
