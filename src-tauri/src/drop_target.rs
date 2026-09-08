#[cfg(windows)]
pub mod win_drop {
    use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};
    use windows::core::*;
    use windows::Win32::Foundation::*;
    use windows::Win32::System::Com::*;
    use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
    use windows::Win32::System::Ole::*;
    use windows::Win32::System::SystemServices::MODIFIERKEYS_FLAGS;
    use windows::Win32::UI::Shell::{DragQueryFileW, HDROP};

    use crate::models::StashItem;
    use crate::AppState;

    const CF_HDROP_ID: u16 = 15;
    const CF_UNICODETEXT_ID: u16 = 13;

    #[implement(IDropTarget)]
    pub struct CustomDropTarget {
        app_handle: AppHandle,
    }

    impl CustomDropTarget {
        pub fn new(app_handle: AppHandle) -> Self {
            Self { app_handle }
        }

        fn has_supported_format(&self, pdataobj: &IDataObject) -> bool {
            let formats = [CF_HDROP_ID, CF_UNICODETEXT_ID];
            for &cf in &formats {
                let format_etc = FORMATETC {
                    cfFormat: cf,
                    ptd: std::ptr::null_mut(),
                    dwAspect: DVASPECT_CONTENT.0,
                    lindex: -1,
                    tymed: TYMED_HGLOBAL.0 as u32,
                };
                unsafe {
                    if pdataobj.QueryGetData(&format_etc) == S_OK {
                        return true;
                    }
                }
            }
            false
        }
    }

    impl IDropTarget_Impl for CustomDropTarget_Impl {
        fn DragEnter(
            &self,
            pdataobj: Option<&IDataObject>,
            _grfkeystate: MODIFIERKEYS_FLAGS,
            _pt: &POINTL,
            pdweffect: *mut DROPEFFECT,
        ) -> Result<()> {
            unsafe {
                if let Some(data_obj) = pdataobj {
                    if self.has_supported_format(data_obj) {
                        *pdweffect = DROPEFFECT_COPY;
                        let _ = self.app_handle.emit("drag-enter", ());
                        return Ok(());
                    }
                }
                *pdweffect = DROPEFFECT_NONE;
                Ok(())
            }
        }

        fn DragOver(
            &self,
            _grfkeystate: MODIFIERKEYS_FLAGS,
            _pt: &POINTL,
            pdweffect: *mut DROPEFFECT,
        ) -> Result<()> {
            unsafe {
                *pdweffect = DROPEFFECT_COPY;
                Ok(())
            }
        }

        fn DragLeave(&self) -> Result<()> {
            let _ = self.app_handle.emit("drag-leave", ());
            Ok(())
        }

        fn Drop(
            &self,
            pdataobj: Option<&IDataObject>,
            _grfkeystate: MODIFIERKEYS_FLAGS,
            _pt: &POINTL,
            pdweffect: *mut DROPEFFECT,
        ) -> Result<()> {
            unsafe {
                *pdweffect = DROPEFFECT_NONE;
                let _ = self.app_handle.emit("drag-leave", ());

                let Some(data_obj) = pdataobj else {
                    return Ok(());
                };

                let mut handled = false;

                // 1. Проверяем файловый формат CF_HDROP
                let format_file = FORMATETC {
                    cfFormat: CF_HDROP_ID,
                    ptd: std::ptr::null_mut(),
                    dwAspect: DVASPECT_CONTENT.0,
                    lindex: -1,
                    tymed: TYMED_HGLOBAL.0 as u32,
                };

                if let Ok(medium) = data_obj.GetData(&format_file) {
                    let hdrop = HDROP(medium.u.hGlobal.0 as _);
                    let count = DragQueryFileW(hdrop, 0xFFFFFFFF, None);
                    let mut file_paths = Vec::new();

                    for i in 0..count {
                        let len = DragQueryFileW(hdrop, i, None) as usize;
                        if len > 0 {
                            let mut buf = vec![0u16; len + 1];
                            DragQueryFileW(hdrop, i, Some(&mut buf));
                            if let Some(null_pos) = buf.iter().position(|&c| c == 0) {
                                buf.truncate(null_pos);
                            }
                            let path = String::from_utf16_lossy(&buf);
                            file_paths.push(path);
                        }
                    }

                    ReleaseStgMedium(&medium as *const _ as *mut _);

                    if !file_paths.is_empty() {
                        let state: State<AppState> = self.app_handle.state();
                        let mut items = state.items.lock().unwrap();
                        let mut added_any = false;

                        for p in file_paths {
                            if !items.iter().any(|it| it.path.as_deref() == Some(&p)) {
                                items.push(StashItem::from_path(&p));
                                added_any = true;
                            }
                        }

                        if added_any {
                            let cloned = items.clone();
                            let _ = self.app_handle.emit("stash-updated", cloned);
                        }
                        *pdweffect = DROPEFFECT_COPY;
                        handled = true;
                    }
                }

                // 2. Если не файлы, проверяем текст / URL (CF_UNICODETEXT)
                if !handled {
                    let format_text = FORMATETC {
                        cfFormat: CF_UNICODETEXT_ID,
                        ptd: std::ptr::null_mut(),
                        dwAspect: DVASPECT_CONTENT.0,
                        lindex: -1,
                        tymed: TYMED_HGLOBAL.0 as u32,
                    };

                    if let Ok(text_medium) = data_obj.GetData(&format_text) {
                        let ptr = GlobalLock(text_medium.u.hGlobal);
                        if !ptr.is_null() {
                            let wide_slice = {
                                let mut len = 0;
                                let mut cur = ptr as *const u16;
                                while *cur != 0 {
                                    len += 1;
                                    cur = cur.add(1);
                                }
                                std::slice::from_raw_parts(ptr as *const u16, len)
                            };
                            let text = String::from_utf16_lossy(wide_slice);
                            let _ = GlobalUnlock(text_medium.u.hGlobal);

                            let non_empty_lines: Vec<&str> = text
                                .lines()
                                .map(|l| l.trim())
                                .filter(|l| !l.is_empty())
                                .collect();

                            if !non_empty_lines.is_empty() {
                                // Если ВСЕ непустые строки — валидные URL, добавляем каждую отдельно.
                                // Иначе весь текст — одна текстовая заметка.
                                let all_urls = non_empty_lines
                                    .iter()
                                    .all(|l| l.starts_with("http://") || l.starts_with("https://"));

                                let state: State<AppState> = self.app_handle.state();
                                let mut items = state.items.lock().unwrap();

                                if all_urls {
                                    for line in &non_empty_lines {
                                        items.push(StashItem::from_text(line));
                                    }
                                } else {
                                    // Сохраняем весь текст целиком (с переносами строк) как одну карточку
                                    items.push(StashItem::from_text(&text));
                                }

                                let cloned = items.clone();
                                let _ = self.app_handle.emit("stash-updated", cloned);
                                *pdweffect = DROPEFFECT_COPY;
                            }
                        }
                        ReleaseStgMedium(&text_medium as *const _ as *mut _);
                    }
                }

                Ok(())
            }
        }
    }

    unsafe extern "system" fn enum_child_proc(
        child_hwnd: windows_sys::Win32::Foundation::HWND,
        lparam: windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::BOOL {
        let list = &mut *(lparam as *mut Vec<HWND>);
        list.push(HWND(child_hwnd as _));
        1 // TRUE
    }

    fn collect_all_hwnds(top_hwnd: HWND) -> Vec<HWND> {
        let mut hwnds = vec![top_hwnd];
        unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::EnumChildWindows(
                top_hwnd.0 as _,
                Some(enum_child_proc),
                &mut hwnds as *mut _ as isize,
            );
        }
        hwnds
    }

    pub fn register_window_drop_target(window: &WebviewWindow) -> Result<()> {
        let hwnd_val = window.hwnd().map_err(|e| Error::new(HRESULT(-1), e.to_string()))?;
        let top_hwnd = HWND(hwnd_val.0 as _);

        let all_hwnds = collect_all_hwnds(top_hwnd);

        for h in all_hwnds {
            unsafe {
                let _ = RevokeDragDrop(h);
                let drop_target: IDropTarget = CustomDropTarget::new(window.app_handle().clone()).into();
                let _ = RegisterDragDrop(h, &drop_target);
            }
        }

        Ok(())
    }

    pub fn unregister_window_drop_target(window: &WebviewWindow) {
        if let Ok(hwnd_val) = window.hwnd() {
            let top_hwnd = HWND(hwnd_val.0 as _);
            let all_hwnds = collect_all_hwnds(top_hwnd);
            for h in all_hwnds {
                unsafe {
                    let _ = RevokeDragDrop(h);
                }
            }
        }
    }
}
