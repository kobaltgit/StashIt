mod autostart;
#[cfg(windows)]
mod drop_target;
#[cfg(windows)]
mod text_drag;
mod hook;
pub mod local_drop;
mod models;
pub mod updater;

use autostart::{is_autostart_enabled, set_autostart};
use models::StashItem;
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
        #[cfg(windows)]
        let _ = drop_target::win_drop::register_window_drop_target(&win);
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn init_drop_target(window: tauri::WebviewWindow) -> Result<(), String> {
    #[cfg(windows)]
    drop_target::win_drop::register_window_drop_target(&window).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn exit_app(app: AppHandle) {
    #[cfg(windows)]
    hook::win_hook::stop_input_monitor();
    for (_, window) in app.webview_windows() {
        #[cfg(windows)]
        drop_target::win_drop::unregister_window_drop_target(&window);
        let _ = window.destroy();
    }
    std::process::exit(0);
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub shake: bool,
    pub hotkey: bool,
    pub hotkey_combo: u8,
    pub double_tap: bool,
    pub double_tap_key: u8,
    pub edge_dock: bool,
    pub theme: String,
    pub lang: String,
    pub auto_clear: bool,
    #[serde(default = "default_true")]
    pub auto_check_updates: bool,
    #[serde(default)]
    pub last_update_check_time: u64,
    #[serde(default)]
    pub last_notified_version: String,
}

fn default_true() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            shake: true,
            hotkey: true,
            hotkey_combo: 0,
            double_tap: true,
            double_tap_key: 0,
            edge_dock: false,
            theme: "system".to_string(),
            lang: "ru".to_string(),
            auto_clear: true,
            auto_check_updates: true,
            last_update_check_time: 0,
            last_notified_version: String::new(),
        }
    }
}

pub fn get_config_path() -> std::path::PathBuf {
    let base = std::env::var("APPDATA")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
    let dir = std::path::Path::new(&base).join("StashIt");
    let _ = std::fs::create_dir_all(&dir);
    dir.join("config.json")
}

pub fn load_config() -> AppConfig {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str::<AppConfig>(&content) {
                return cfg;
            }
        }
    }
    AppConfig::default()
}

pub fn save_config(cfg: &AppConfig) -> Result<(), String> {
    let path = get_config_path();
    let json = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_app_config() -> AppConfig {
    load_config()
}

#[tauri::command]
fn save_app_config(config: AppConfig) -> Result<(), String> {
    #[cfg(windows)]
    hook::win_hook::apply_settings(
        config.shake,
        config.hotkey,
        config.hotkey_combo,
        config.double_tap,
        config.double_tap_key,
        config.edge_dock,
    );
    save_config(&config)
}

#[tauri::command]
fn set_trigger_mode(
    shake: bool,
    hotkey: bool,
    hotkey_combo: u8,
    double_tap: bool,
    double_tap_key: u8,
    edge_dock: bool,
) {
    #[cfg(windows)]
    hook::win_hook::apply_settings(shake, hotkey, hotkey_combo, double_tap, double_tap_key, edge_dock);
}

#[tauri::command]
fn check_for_updates(app: AppHandle, force: bool) -> Result<updater::UpdateCheckResult, String> {
    updater::check_updates_with_cooldown(&app, force)
}

#[tauri::command]
fn open_external_url(url: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        use std::os::windows::process::CommandExt;
        Command::new("cmd")
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .args(["/c", "start", "", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Sanitize a string for use as a file name (removes illegal Windows chars, max 60 chars).
pub(crate) fn sanitize_filename(s: &str) -> String {
    let sanitized: String = s
        .chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\n' | '\r' | '\t' => '_',
            c => c,
        })
        .take(60)
        .collect();
    let trimmed = sanitized.trim_matches(|c: char| c == '.' || c == ' ');
    if trimmed.is_empty() {
        "stashit_note".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Returns (and creates if needed) the StashIt temp directory: %TEMP%\StashIt\
pub(crate) fn get_stashit_temp_dir() -> Result<std::path::PathBuf, String> {
    let base = std::env::var("TEMP")
        .or_else(|_| std::env::var("TMP"))
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
    let dir = std::path::Path::new(&base).join("StashIt");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

/// Removes the entire %TEMP%\StashIt\ directory (called once on startup).
fn cleanup_temp_dir() {
    if let Ok(base) = std::env::var("TEMP").or_else(|_| std::env::var("TMP")) {
        let dir = std::path::Path::new(&base).join("StashIt");
        if dir.exists() {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}

/// Creates a temporary .txt or .url file in %TEMP%\StashIt\ and returns its full path.
/// The file is automatically deleted by `drag_item` after the OLE drop completes.
#[tauri::command]
fn create_temp_file(name: String, content: String, is_url: bool) -> Result<String, String> {
    let temp_dir = get_stashit_temp_dir()?;

    let safe_name = sanitize_filename(&name);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let filename = if is_url {
        format!("{}_{}.url", safe_name, now)
    } else {
        format!("{}_{}.txt", safe_name, now)
    };

    let file_path = temp_dir.join(&filename);

    let file_content = if is_url {
        format!("[InternetShortcut]\nURL={}\n", content.trim())
    } else {
        content.clone()
    };

    std::fs::write(&file_path, file_content.as_bytes()).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
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
            // Удаляем временные файлы StashIt с задержкой 5 сек, чтобы Проводник гарантированно завершил чтение
            for path_str in &paths_clone {
                let p = std::path::Path::new(path_str);
                let is_stashit_temp = p
                    .parent()
                    .and_then(|parent| parent.file_name())
                    .map(|name| name == "StashIt")
                    .unwrap_or(false);
                if is_stashit_temp {
                    let path_buf = p.to_path_buf();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        let _ = std::fs::remove_file(path_buf);
                    });
                }
            }
        },
        drag::Options::default(),
    );

    Ok(())
}

#[cfg(windows)]
#[tauri::command]
fn drag_text_item(
    window: tauri::WebviewWindow,
    name: String,
    content: String,
    is_url: bool,
) {
    text_drag::start_text_drag(window, name, content, is_url);
}

#[tauri::command]
fn copy_to_clipboard(paths: Vec<String>, text: Option<String>) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows::Win32::Foundation::{GlobalFree, HGLOBAL};
        use windows_sys::Win32::Foundation::HWND;
        use windows_sys::Win32::System::DataExchange::{
            CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
        };
        use windows_sys::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GHND};
        use windows_sys::Win32::UI::Shell::DROPFILES;

        const CF_HDROP: u32 = 15;
        const CF_UNICODETEXT: u32 = 13;

        if paths.is_empty() && text.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            return Ok(());
        }

        // 1. Подготовка CF_HDROP (для вставки файлов в Проводник)
        let h_drop = if !paths.is_empty() {
            let mut wide_paths: Vec<u16> = Vec::new();
            for p in &paths {
                let encoded: Vec<u16> = OsStr::new(p).encode_wide().collect();
                wide_paths.extend(encoded);
                wide_paths.push(0);
            }
            wide_paths.push(0);

            let dropfiles_size = std::mem::size_of::<DROPFILES>();
            let total_size = dropfiles_size + wide_paths.len() * 2;

            unsafe {
                let h_mem = GlobalAlloc(GHND, total_size);
                if !h_mem.is_null() {
                    let p_mem = GlobalLock(h_mem) as *mut u8;
                    if !p_mem.is_null() {
                        let dropfiles = p_mem as *mut DROPFILES;
                        (*dropfiles).pFiles = dropfiles_size as u32;
                        (*dropfiles).fWide = 1; // Unicode
                        let dest_paths = p_mem.add(dropfiles_size) as *mut u16;
                        std::ptr::copy_nonoverlapping(wide_paths.as_ptr(), dest_paths, wide_paths.len());
                        GlobalUnlock(h_mem);
                        Some(h_mem)
                    } else {
                        let _ = GlobalFree(HGLOBAL(h_mem as *mut _));
                        None
                    }
                } else {
                    None
                }
            }
        } else {
            None
        };

        // 2. Подготовка CF_UNICODETEXT (для вставки текста в редакторы/мессенджеры)
        let h_text = if let Some(ref txt) = text {
            if !txt.is_empty() {
                let wide_text: Vec<u16> = OsStr::new(txt).encode_wide().chain(Some(0)).collect();
                let text_size = wide_text.len() * 2;
                unsafe {
                    let h_mem = GlobalAlloc(GHND, text_size);
                    if !h_mem.is_null() {
                        let p_mem = GlobalLock(h_mem) as *mut u16;
                        if !p_mem.is_null() {
                            std::ptr::copy_nonoverlapping(wide_text.as_ptr(), p_mem, wide_text.len());
                            GlobalUnlock(h_mem);
                            Some(h_mem)
                        } else {
                            let _ = GlobalFree(HGLOBAL(h_mem as *mut _));
                            None
                        }
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        unsafe {
            if OpenClipboard(0 as HWND) == 0 {
                if let Some(h) = h_drop {
                    let _ = GlobalFree(HGLOBAL(h as *mut _));
                }
                if let Some(h) = h_text {
                    let _ = GlobalFree(HGLOBAL(h as *mut _));
                }
                return Err("Failed to open clipboard".into());
            }

            EmptyClipboard();

            // Помещаем оба формата в буфер обмена:
            // Проводник заберёт CF_HDROP и вставит файл;
            // Текстовые редакторы заберут CF_UNICODETEXT и вставят строку.
            if let Some(h) = h_drop {
                if SetClipboardData(CF_HDROP, h as _).is_null() {
                    let _ = GlobalFree(HGLOBAL(h as *mut _));
                }
            }

            if let Some(h) = h_text {
                if SetClipboardData(CF_UNICODETEXT, h as _).is_null() {
                    let _ = GlobalFree(HGLOBAL(h as *mut _));
                }
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
        .manage(local_drop::LocalDropManager::default())
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
            get_app_config,
            save_app_config,
            drag_item,
            create_temp_file,
            drag_text_item,
            copy_to_clipboard,
            init_drop_target,
            exit_app,
            check_for_updates,
            open_external_url,
            local_drop::start_local_drop,
            local_drop::stop_local_drop
        ])
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Показать StashIt", true, None::<&str>)?;
            let about_i = MenuItem::with_id(app, "about", "О программе", true, None::<&str>)?;
            let clear_i = MenuItem::with_id(app, "clear", "Очистить карман", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Выход", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &about_i, &clear_i, &quit_i])?;

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
                    "about" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.emit("open-about-tab", ());
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
                        hook::win_hook::stop_input_monitor();

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

            // Зачистка временных файлов предыдущей сессии
            cleanup_temp_dir();

            // Применяем сохраненные настройки пользователя к хукам
            let initial_config = load_config();
            #[cfg(windows)]
            hook::win_hook::apply_settings(
                initial_config.shake,
                initial_config.hotkey,
                initial_config.hotkey_combo,
                initial_config.double_tap,
                initial_config.double_tap_key,
                initial_config.edge_dock,
            );

            // Запуск фонового мониторинга ввода (Shake, Hotkey, Double-Tap, Edge Dock)
            #[cfg(windows)]
            hook::win_hook::start_input_monitor(app.handle().clone());

            // Регистрация кастомного OLE IDropTarget (файлы + URL/текст)
            #[cfg(windows)]
            if let Some(window) = app.get_webview_window("main") {
                if let Err(e) = drop_target::win_drop::register_window_drop_target(&window) {
                    eprintln!("Ошибка регистрации drop target: {:?}", e);
                }
            }

            // Фоновая проверка обновлений (через 3 сек после старта, затем раз в неделю)
            updater::start_background_updater(app.handle().clone());

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                #[cfg(windows)]
                hook::win_hook::stop_input_monitor();

                for (_, window) in app_handle.webview_windows() {
                    #[cfg(windows)]
                    drop_target::win_drop::unregister_window_drop_target(&window);
                    let _ = window.destroy();
                }
            }
        });
}
