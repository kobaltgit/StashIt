mod autostart;
#[cfg(windows)]
mod drop_target;
mod hook;
mod models;

use autostart::{is_autostart_enabled, set_autostart};
use models::StashItem;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, State,
};

#[derive(Default)]
pub struct AppState {
    pub items: Mutex<Vec<StashItem>>,
}

#[tauri::command]
fn get_stash_items(state: State<'_, AppState>) -> Vec<StashItem> {
    let items = state.items.lock().unwrap();
    items.clone()
}

#[tauri::command]
fn add_stash_item_paths(paths: Vec<String>, state: State<'_, AppState>) -> Vec<StashItem> {
    let mut items = state.items.lock().unwrap();
    for p in paths {
        if !items.iter().any(|it| it.path.as_deref() == Some(&p)) {
            items.push(StashItem::from_path(&p));
        }
    }
    items.clone()
}

#[tauri::command]
fn add_stash_text(text: String, state: State<'_, AppState>) -> Vec<StashItem> {
    let mut items = state.items.lock().unwrap();
    items.push(StashItem::from_text(&text));
    items.clone()
}

#[tauri::command]
fn remove_stash_item(id: String, state: State<'_, AppState>) -> Vec<StashItem> {
    let mut items = state.items.lock().unwrap();
    items.retain(|it| it.id != id);
    items.clone()
}

#[tauri::command]
fn clear_stash(state: State<'_, AppState>) -> Vec<StashItem> {
    let mut items = state.items.lock().unwrap();
    items.clear();
    items.clone()
}

#[tauri::command]
fn hide_shelf(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("main") {
        win.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn show_shelf(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("main") {
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn toggle_autostart(enable: bool) -> Result<bool, String> {
    set_autostart(enable)?;
    Ok(is_autostart_enabled())
}

#[tauri::command]
fn check_autostart() -> bool {
    is_autostart_enabled()
}

#[tauri::command]
fn set_trigger_mode(shake: bool, auto_drag: bool) {
    hook::win_hook::SHAKE_DETECTION_ENABLED.store(shake, Ordering::Relaxed);
    hook::win_hook::DRAG_DETECTION_ENABLED.store(auto_drag, Ordering::Relaxed);
}

#[tauri::command]
fn drag_item(window: tauri::WebviewWindow, paths: Vec<String>) -> Result<(), String> {
    use std::path::PathBuf;
    let path_bufs: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();

    // 1x1 прозрачный пиксель PNG
    let empty_png: [u8; 67] = [
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    let drag_item = drag::DragItem::Files(path_bufs);
    let preview_image = drag::Image::Raw(empty_png.to_vec());

    let window_clone = window.clone();
    let paths_clone = paths.clone();

    let _ = drag::start_drag(
        &window,
        drag_item,
        preview_image,
        move |_result, _pos| {
            let _ = window_clone.emit("drag-out-completed", paths_clone.clone());
        },
        drag::Options::default(),
    );

    Ok(())
}

#[tauri::command]
fn copy_to_clipboard(paths: Vec<String>) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Foundation::HWND;
        use windows_sys::Win32::System::DataExchange::{
            CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
        };
        use windows_sys::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GHND};
        use windows_sys::Win32::UI::Shell::DROPFILES;

        const CF_HDROP: u32 = 15;

        if paths.is_empty() {
            return Ok(());
        }

        // Подготовка буфера UTF-16 с двойным нулём в конце: path1\0path2\0\0
        let mut wide_paths: Vec<u16> = Vec::new();
        for p in paths {
            let encoded: Vec<u16> = OsStr::new(&p).encode_wide().collect();
            wide_paths.extend(encoded);
            wide_paths.push(0);
        }
        wide_paths.push(0);

        let dropfiles_size = std::mem::size_of::<DROPFILES>();
        let total_size = dropfiles_size + wide_paths.len() * 2;

        unsafe {
            let h_mem = GlobalAlloc(GHND, total_size);
            if h_mem.is_null() {
                return Err("Failed to allocate global memory for clipboard".into());
            }

            let p_mem = GlobalLock(h_mem) as *mut u8;
            if p_mem.is_null() {
                return Err("Failed to lock global memory".into());
            }

            let dropfiles = p_mem as *mut DROPFILES;
            (*dropfiles).pFiles = dropfiles_size as u32;
            (*dropfiles).fWide = 1; // Unicode

            let dest_paths = p_mem.add(dropfiles_size) as *mut u16;
            std::ptr::copy_nonoverlapping(wide_paths.as_ptr(), dest_paths, wide_paths.len());

            GlobalUnlock(h_mem);

            if OpenClipboard(0 as HWND) == 0 {
                return Err("Failed to open clipboard".into());
            }
            EmptyClipboard();
            if SetClipboardData(CF_HDROP, h_mem as _).is_null() {
                CloseClipboard();
                return Err("Failed to set CF_HDROP clipboard data".into());
            }
            CloseClipboard();
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_stash_items,
            add_stash_item_paths,
            add_stash_text,
            remove_stash_item,
            clear_stash,
            hide_shelf,
            show_shelf,
            toggle_autostart,
            check_autostart,
            set_trigger_mode,
            drag_item,
            copy_to_clipboard
        ])
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Показать StashIt", true, None::<&str>)?;
            let clear_i = MenuItem::with_id(app, "clear", "Очистить карман", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Выход", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &clear_i, &quit_i])?;

            let tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("StashIt — Kobalt Tools");

            let tray_builder = if let Some(icon) = app.default_window_icon() {
                tray_builder.icon(icon.clone())
            } else {
                tray_builder
            };

            let _tray = tray_builder
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "clear" => {
                        let state: State<AppState> = app.state();
                        let mut items = state.items.lock().unwrap();
                        items.clear();
                        let _ = app.emit("stash-cleared", ());
                    }
                    "quit" => {
                        #[cfg(windows)]
                        hook::win_hook::stop_mouse_monitor();

                        for (_, window) in app.webview_windows() {
                            #[cfg(windows)]
                            drop_target::win_drop::unregister_window_drop_target(&window);
                            let _ = window.destroy();
                        }
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if let Ok(visible) = window.is_visible() {
                                if visible {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build(app)?;

            // Запуск фонового мониторинга мыши (Shake to Show & Auto on Drag)
            #[cfg(windows)]
            hook::win_hook::start_mouse_monitor(app.handle().clone());

            // Регистрация кастомного OLE IDropTarget (файлы + URL/текст)
            #[cfg(windows)]
            if let Some(window) = app.get_webview_window("main") {
                if let Err(e) = drop_target::win_drop::register_window_drop_target(&window) {
                    eprintln!("Ошибка регистрации drop target: {:?}", e);
                }
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                #[cfg(windows)]
                hook::win_hook::stop_mouse_monitor();

                for (_, window) in app_handle.webview_windows() {
                    #[cfg(windows)]
                    drop_target::win_drop::unregister_window_drop_target(&window);
                    let _ = window.destroy();
                }
            }
        });
}
