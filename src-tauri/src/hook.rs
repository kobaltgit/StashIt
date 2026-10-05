#[cfg(windows)]
pub mod win_hook {
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use std::time::Instant;
    use tauri::{AppHandle, Emitter, Manager, PhysicalPosition};
    use windows_sys::Win32::Foundation::{LPARAM, LRESULT, POINT, RECT, WPARAM};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
        MONITOR_DEFAULTTONULL,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_CONTROL, VK_LCONTROL, VK_LSHIFT, VK_LWIN, VK_MENU, VK_RCONTROL,
        VK_RSHIFT, VK_RWIN, VK_SHIFT, VK_SPACE,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, GetCursorPos, SetWindowsHookExW, UnhookWindowsHookEx,
        HHOOK, KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, WH_KEYBOARD_LL,
        WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
        WM_SYSKEYDOWN, WM_SYSKEYUP,
    };

    // Переключатели способов активации
    pub static SHAKE_DETECTION_ENABLED: AtomicBool = AtomicBool::new(true);
    pub static HOTKEY_DETECTION_ENABLED: AtomicBool = AtomicBool::new(true);
    pub static DOUBLE_TAP_DETECTION_ENABLED: AtomicBool = AtomicBool::new(true);
    pub static EDGE_DOCK_DETECTION_ENABLED: AtomicBool = AtomicBool::new(false);

    // ID комбинации хоткея:
    // 0: Ctrl + Shift + Space
    // 1: Alt + S
    // 2: Ctrl + Alt + S
    // 3: Win + Alt + S
    // 4: Ctrl + Shift + X
    pub static HOTKEY_COMBO_ID: AtomicU8 = AtomicU8::new(0);

    // ID клавиши для двойного нажатия:
    // 0: Control (Ctrl)
    // 1: Shift
    pub static DOUBLE_TAP_KEY_ID: AtomicU8 = AtomicU8::new(0);

    // Состояние мыши
    static IS_LBUTTON_DOWN: AtomicBool = AtomicBool::new(false);
    static mut APP_HANDLE: Option<AppHandle> = None;
    static mut DIRECTION_CHANGES: i32 = 0;
    static mut LAST_DIR_X: i32 = 0;
    static mut LAST_POS_X: i32 = 0;
    static mut LAST_SHAKE_TIME: Option<Instant> = None;
    static mut LAST_EDGE_ENTER_TIME: Option<Instant> = None;
    static mut LAST_TRIGGER_TIME: Option<Instant> = None;

    // Состояние клавиатуры для Double-Tap
    static mut MODIFIER_DOWN: bool = false;
    static mut OTHER_KEY_PRESSED_SINCE_MODIFIER: bool = false;
    static mut LAST_MODIFIER_RELEASE: Option<Instant> = None;

    // Хэндлы Win32 хуков и потока
    static mut HOOK_THREAD_ID: u32 = 0;
    static mut MOUSE_HOOK: HHOOK = std::ptr::null_mut();
    static mut KEYBOARD_HOOK: HHOOK = std::ptr::null_mut();

    pub fn start_input_monitor(app_handle: AppHandle) {
        unsafe {
            APP_HANDLE = Some(app_handle);
        }

        std::thread::spawn(|| unsafe {
            HOOK_THREAD_ID = windows_sys::Win32::System::Threading::GetCurrentThreadId();

            let m_hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(low_level_mouse_proc),
                std::ptr::null_mut(),
                0,
            );
            if m_hook.is_null() {
                eprintln!("[StashIt] Не удалось установить Win32 Mouse Hook");
            } else {
                MOUSE_HOOK = m_hook;
            }

            let k_hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(low_level_keyboard_proc),
                std::ptr::null_mut(),
                0,
            );
            if k_hook.is_null() {
                eprintln!("[StashIt] Не удалось установить Win32 Keyboard Hook");
            } else {
                KEYBOARD_HOOK = k_hook;
            }

            let mut msg = std::mem::zeroed();
            while windows_sys::Win32::UI::WindowsAndMessaging::GetMessageW(
                &mut msg,
                std::ptr::null_mut(),
                0,
                0,
            ) > 0
            {
                windows_sys::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
                windows_sys::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg);
            }

            if !MOUSE_HOOK.is_null() {
                UnhookWindowsHookEx(MOUSE_HOOK);
                MOUSE_HOOK = std::ptr::null_mut();
            }
            if !KEYBOARD_HOOK.is_null() {
                UnhookWindowsHookEx(KEYBOARD_HOOK);
                KEYBOARD_HOOK = std::ptr::null_mut();
            }
            HOOK_THREAD_ID = 0;
        });
    }

    pub fn stop_input_monitor() {
        unsafe {
            if !MOUSE_HOOK.is_null() {
                UnhookWindowsHookEx(MOUSE_HOOK);
                MOUSE_HOOK = std::ptr::null_mut();
            }
            if !KEYBOARD_HOOK.is_null() {
                UnhookWindowsHookEx(KEYBOARD_HOOK);
                KEYBOARD_HOOK = std::ptr::null_mut();
            }
            if HOOK_THREAD_ID != 0 {
                windows_sys::Win32::UI::WindowsAndMessaging::PostThreadMessageW(
                    HOOK_THREAD_ID,
                    windows_sys::Win32::UI::WindowsAndMessaging::WM_QUIT,
                    0,
                    0,
                );
                HOOK_THREAD_ID = 0;
            }
        }
    }

    #[inline]
    fn is_key_down(vk: u16) -> bool {
        unsafe { (GetAsyncKeyState(vk as i32) as u16 & 0x8000) != 0 }
    }

    /// Получить рабочую область (rcWork) монитора, на котором находится точка pt
    #[inline]
    unsafe fn get_monitor_work_area_at(pt: POINT) -> Option<RECT> {
        let h_mon = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
        if h_mon == 0 as _ {
            return None;
        }
        let mut mi: MONITORINFO = std::mem::zeroed();
        mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(h_mon, &mut mi) != 0 {
            Some(mi.rcWork)
        } else {
            None
        }
    }

    /// Проверить, существует ли монитор в заданной глобальной точке pt
    #[inline]
    unsafe fn is_monitor_at(pt: POINT) -> bool {
        let h_mon = MonitorFromPoint(pt, MONITOR_DEFAULTTONULL);
        h_mon != 0 as _
    }

    // Низкоуровневый обработчик мыши
    unsafe extern "system" fn low_level_mouse_proc(
        n_code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        if n_code >= 0 {
            let hook_struct = *(l_param as *const MSLLHOOKSTRUCT);
            let pt = hook_struct.pt;

            match w_param as u32 {
                WM_LBUTTONDOWN => {
                    IS_LBUTTON_DOWN.store(true, Ordering::SeqCst);
                    DIRECTION_CHANGES = 0;
                    LAST_DIR_X = 0;
                    LAST_POS_X = pt.x;
                    LAST_SHAKE_TIME = Some(Instant::now());
                    LAST_EDGE_ENTER_TIME = None;
                }
                WM_LBUTTONUP => {
                    IS_LBUTTON_DOWN.store(false, Ordering::SeqCst);
                    DIRECTION_CHANGES = 0;
                    LAST_DIR_X = 0;
                    LAST_EDGE_ENTER_TIME = None;

                    if let Some(ref app) = APP_HANDLE {
                        let _ = app.emit("global-mouse-up", ());
                    }
                }
                WM_MOUSEMOVE => {
                    let now = Instant::now();

                    // 1. Детекция "Встряски" (Shake to Show) при зажатом ЛКМ
                    if IS_LBUTTON_DOWN.load(Ordering::SeqCst)
                        && SHAKE_DETECTION_ENABLED.load(Ordering::Relaxed)
                    {
                        let dx = pt.x - LAST_POS_X;
                        if let Some(shake_start) = LAST_SHAKE_TIME {
                            if now.duration_since(shake_start).as_millis() > 650 {
                                DIRECTION_CHANGES = 0;
                                LAST_SHAKE_TIME = Some(now);
                            }
                        }

                        if dx.abs() > 25 {
                            let dir = if dx > 0 { 1 } else { -1 };
                            if LAST_DIR_X != 0 && dir != LAST_DIR_X {
                                DIRECTION_CHANGES += 1;
                                LAST_SHAKE_TIME = Some(now);
                            }
                            LAST_DIR_X = dir;
                            LAST_POS_X = pt.x;
                        }

                        if DIRECTION_CHANGES >= 3 {
                            DIRECTION_CHANGES = 0;
                            trigger_shelf_appearance(pt.x, pt.y);
                        }
                    }

                    // 2. Детекция "Края экрана" (Edge Dock) с поддержкой мультимониторов
                    if EDGE_DOCK_DETECTION_ENABLED.load(Ordering::Relaxed) {
                        if let Some(work) = get_monitor_work_area_at(pt) {
                            let mon_h = work.bottom - work.top;
                            // Активная зона по вертикали: 15% - 85% рабочей области текущего монитора
                            if pt.y >= work.top + (mon_h * 15 / 100) && pt.y <= work.top + (mon_h * 85 / 100) {
                                let mut at_right = false;
                                let mut at_left = false;

                                if pt.x >= work.right - 5 {
                                    // Проверяем: есть ли соседний монитор справа от этой точки.
                                    // Если за кромкой пустота — это внешний физический край!
                                    let has_right_neighbor = is_monitor_at(POINT {
                                        x: work.right + 10,
                                        y: pt.y,
                                    });
                                    if !has_right_neighbor {
                                        at_right = true;
                                    }
                                } else if pt.x <= work.left + 5 {
                                    // Проверяем: есть ли соседний монитор слева от этой точки.
                                    // Если за кромкой пустота — это внешний физический край!
                                    let has_left_neighbor = is_monitor_at(POINT {
                                        x: work.left - 10,
                                        y: pt.y,
                                    });
                                    if !has_left_neighbor {
                                        at_left = true;
                                    }
                                }

                                if at_right || at_left {
                                    let is_dragging = IS_LBUTTON_DOWN.load(Ordering::SeqCst);
                                    let triggered = if is_dragging {
                                        // При зажатом ЛКМ (перетаскивании) раскрываем моментально
                                        true
                                    } else {
                                        // При свободном курсоре требуем задержку dwell time 200 мс
                                        if let Some(dwell) = LAST_EDGE_ENTER_TIME {
                                            now.duration_since(dwell).as_millis() >= 200
                                        } else {
                                            LAST_EDGE_ENTER_TIME = Some(now);
                                            false
                                        }
                                    };

                                    if triggered {
                                        LAST_EDGE_ENTER_TIME = None;
                                        trigger_shelf_docked(at_right, pt.y, work);
                                    }
                                } else {
                                    LAST_EDGE_ENTER_TIME = None;
                                }
                            } else {
                                LAST_EDGE_ENTER_TIME = None;
                            }
                        } else {
                            LAST_EDGE_ENTER_TIME = None;
                        }
                    }
                }
                _ => {}
            }
        }

        CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
    }

    // Низкоуровневый обработчик клавиатуры
    unsafe extern "system" fn low_level_keyboard_proc(
        n_code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        if n_code >= 0 {
            let hook_struct = *(l_param as *const KBDLLHOOKSTRUCT);
            let vk = hook_struct.vkCode as u16;

            let msg = w_param as u32;
            let is_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            let is_up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

            let target_mod_id = DOUBLE_TAP_KEY_ID.load(Ordering::Relaxed);
            let is_target_modifier = match target_mod_id {
                0 => vk == VK_CONTROL || vk == VK_LCONTROL || vk == VK_RCONTROL,
                _ => vk == VK_SHIFT || vk == VK_LSHIFT || vk == VK_RSHIFT,
            };

            // --- 1. Обработка Double-Tap модификатора ---
            if is_down {
                if is_target_modifier {
                    if !MODIFIER_DOWN {
                        MODIFIER_DOWN = true;
                        OTHER_KEY_PRESSED_SINCE_MODIFIER = false;
                    }
                } else {
                    // Любая другая клавиша (например C в Ctrl+C) аннулирует double-tap
                    OTHER_KEY_PRESSED_SINCE_MODIFIER = true;
                    LAST_MODIFIER_RELEASE = None;
                }
            } else if is_up && is_target_modifier {
                MODIFIER_DOWN = false;
                if !OTHER_KEY_PRESSED_SINCE_MODIFIER {
                    let now = Instant::now();
                    if let Some(prev_release) = LAST_MODIFIER_RELEASE {
                        let elapsed = now.duration_since(prev_release).as_millis();
                        if elapsed >= 50 && elapsed <= 380 {
                            LAST_MODIFIER_RELEASE = None;
                            if DOUBLE_TAP_DETECTION_ENABLED.load(Ordering::Relaxed) {
                                trigger_shelf_toggle_at_cursor();
                            }
                        } else {
                            LAST_MODIFIER_RELEASE = Some(now);
                        }
                    } else {
                        LAST_MODIFIER_RELEASE = Some(now);
                    }
                }
            }

            // --- 2. Обработка Глобального Хоткея ---
            if is_down && HOTKEY_DETECTION_ENABLED.load(Ordering::Relaxed) {
                let ctrl = is_key_down(VK_CONTROL);
                let shift = is_key_down(VK_SHIFT);
                let alt = is_key_down(VK_MENU);
                let win = is_key_down(VK_LWIN) || is_key_down(VK_RWIN);

                let combo_id = HOTKEY_COMBO_ID.load(Ordering::Relaxed);
                let matched = match combo_id {
                    // 0: Ctrl + Shift + Space
                    0 => vk == VK_SPACE && ctrl && shift && !alt && !win,
                    // 1: Alt + S
                    1 => (vk == 0x53) && alt && !ctrl && !shift && !win,
                    // 2: Ctrl + Alt + S
                    2 => (vk == 0x53) && ctrl && alt && !shift && !win,
                    // 3: Win + Alt + S
                    3 => (vk == 0x53) && win && alt && !ctrl && !shift,
                    // 4: Ctrl + Shift + X
                    4 => (vk == 0x58) && ctrl && shift && !alt && !win,
                    _ => false,
                };

                if matched {
                    trigger_shelf_toggle_at_cursor();
                    return 1; // Поглощаем событие хоткея
                }
            }
        }

        CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
    }

    /// Показать полку около курсора мыши (Shake, Hotkey, Double-tap) с учетом текущего монитора
    pub fn trigger_shelf_appearance(cur_x: i32, cur_y: i32) {
        unsafe {
            let now = Instant::now();
            if let Some(last) = LAST_TRIGGER_TIME {
                if now.duration_since(last).as_millis() < 600 {
                    return;
                }
            }
            LAST_TRIGGER_TIME = Some(now);

            if let Some(ref app) = APP_HANDLE {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(is_visible) = window.is_visible() {
                        if !is_visible {
                            let pt = POINT { x: cur_x, y: cur_y };
                            let work = get_monitor_work_area_at(pt).unwrap_or(RECT {
                                left: 0,
                                top: 0,
                                right: 1920,
                                bottom: 1080,
                            });

                            let win_w = 340;
                            let win_h = 420;

                            let mut target_x = cur_x + 20;
                            let mut target_y = cur_y - 20;

                            // Если окно не помещается справа от курсора на текущем мониторе — открываем слева
                            if target_x + win_w > work.right - 10 {
                                target_x = cur_x - win_w - 20;
                            }
                            // Если и слева выходит за пределы текущего монитора — прижимаем к левому краю
                            if target_x < work.left + 10 {
                                target_x = work.left + 10;
                            }

                            // Если окно не помещается снизу (или задевает панель задач) — сдвигаем вверх
                            if target_y + win_h > work.bottom - 10 {
                                target_y = work.bottom - win_h - 10;
                            }
                            // Если вылезает вверх — прижимаем к верху текущего монитора
                            if target_y < work.top + 10 {
                                target_y = work.top + 10;
                            }

                            let _ = window.set_position(PhysicalPosition::new(target_x, target_y));
                            let _ = window.show();
                            let win_clone = window.clone();
                            let _ = window.run_on_main_thread(move || {
                                let _ = crate::drop_target::win_drop::register_window_drop_target(&win_clone);
                            });
                            // ВАЖНО: Ни в коем случае не крадем фокус, если зажата ЛКМ (идёт перетаскивание из Explorer).
                            // В Windows вызов set_focus() прерывает захват мыши OLE DoDragDrop,
                            // ломая DragEnter и оставляя зависший ghost-образ drag image.
                            if !IS_LBUTTON_DOWN.load(Ordering::SeqCst) {
                                let _ = window.set_focus();
                            }
                            let _ = app.emit("shelf-opened", ());
                        }
                    }
                }
            }
        }
    }

    /// Показать полку, примагниченную к краю экрана (Edge Dock) на текущем мониторе
    pub fn trigger_shelf_docked(is_right: bool, cur_y: i32, work: RECT) {
        unsafe {
            let now = Instant::now();
            if let Some(last) = LAST_TRIGGER_TIME {
                if now.duration_since(last).as_millis() < 600 {
                    return;
                }
            }
            LAST_TRIGGER_TIME = Some(now);

            if let Some(ref app) = APP_HANDLE {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(is_visible) = window.is_visible() {
                        if !is_visible {
                            let win_w = 340;
                            let win_h = 420;

                            let target_x = if is_right {
                                work.right - win_w - 6
                            } else {
                                work.left + 6
                            };

                            let mut target_y = cur_y - win_h / 2;
                            if target_y < work.top + 10 {
                                target_y = work.top + 10;
                            } else if target_y + win_h > work.bottom - 10 {
                                target_y = work.bottom - win_h - 10;
                            }

                            let _ = window.set_position(PhysicalPosition::new(target_x, target_y));
                            let _ = window.show();
                            let win_clone = window.clone();
                            let _ = window.run_on_main_thread(move || {
                                let _ = crate::drop_target::win_drop::register_window_drop_target(&win_clone);
                            });
                            if !IS_LBUTTON_DOWN.load(Ordering::SeqCst) {
                                let _ = window.set_focus();
                            }
                            let _ = app.emit("shelf-opened-docked", ());
                        }
                    }
                }
            }
        }
    }

    /// Переключить видимость полки около текущего положения курсора (для Хоткея и Double-Tap)
    pub fn trigger_shelf_toggle_at_cursor() {
        unsafe {
            if let Some(ref app) = APP_HANDLE {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(is_visible) = window.is_visible() {
                        if is_visible {
                            let _ = window.hide();
                        } else {
                            let mut pt = POINT { x: 0, y: 0 };
                            GetCursorPos(&mut pt);
                            trigger_shelf_appearance(pt.x, pt.y);
                        }
                    }
                }
            }
        }
    }

    pub fn apply_settings(
        shake: bool,
        hotkey: bool,
        hotkey_combo: u8,
        double_tap: bool,
        double_tap_key: u8,
        edge_dock: bool,
    ) {
        SHAKE_DETECTION_ENABLED.store(shake, Ordering::Relaxed);
        HOTKEY_DETECTION_ENABLED.store(hotkey, Ordering::Relaxed);
        HOTKEY_COMBO_ID.store(hotkey_combo, Ordering::Relaxed);
        DOUBLE_TAP_DETECTION_ENABLED.store(double_tap, Ordering::Relaxed);
        DOUBLE_TAP_KEY_ID.store(double_tap_key, Ordering::Relaxed);
        EDGE_DOCK_DETECTION_ENABLED.store(edge_dock, Ordering::Relaxed);
    }
}

#[cfg(not(windows))]
pub mod win_hook {
    use tauri::AppHandle;
    pub fn start_input_monitor(_app_handle: AppHandle) {}
    pub fn stop_input_monitor() {}
    pub fn trigger_shelf_appearance(_x: i32, _y: i32) {}
    pub fn trigger_shelf_docked(_is_right: bool, _y: i32, _w: i32, _h: i32) {}
    pub fn trigger_shelf_toggle_at_cursor() {}
    pub fn apply_settings(_shake: bool, _hotkey: bool, _combo: u8, _dt: bool, _dt_key: u8, _edge: bool) {}
}
