#[cfg(windows)]
pub mod win_hook {
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
    use std::time::Instant;
    use tauri::{AppHandle, Emitter, Manager, PhysicalPosition};
    use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, GetSystemMetrics, SetWindowsHookExW, UnhookWindowsHookEx,
        HHOOK, MSLLHOOKSTRUCT, SM_CXSCREEN, SM_CYSCREEN, WH_MOUSE_LL, WM_LBUTTONDOWN,
        WM_LBUTTONUP, WM_MOUSEMOVE,
    };

    pub static SHAKE_DETECTION_ENABLED: AtomicBool = AtomicBool::new(true);
    pub static DRAG_DETECTION_ENABLED: AtomicBool = AtomicBool::new(false);

    static IS_LBUTTON_DOWN: AtomicBool = AtomicBool::new(false);
    static DRAG_START_X: AtomicI32 = AtomicI32::new(0);
    static DRAG_START_Y: AtomicI32 = AtomicI32::new(0);

    static mut APP_HANDLE: Option<AppHandle> = None;
    static mut LAST_MOUSE_MOVE: Option<Instant> = None;
    static mut DIRECTION_CHANGES: i32 = 0;
    static mut LAST_DIR_X: i32 = 0;
    static mut LAST_POS_X: i32 = 0;
    static mut LAST_SHAKE_TIME: Option<Instant> = None;
    static mut HOOK_THREAD_ID: u32 = 0;
    static mut MOUSE_HOOK: HHOOK = std::ptr::null_mut();

    pub fn start_mouse_monitor(app_handle: AppHandle) {
        unsafe {
            APP_HANDLE = Some(app_handle);
            LAST_MOUSE_MOVE = Some(Instant::now());
        }

        std::thread::spawn(|| unsafe {
            HOOK_THREAD_ID = windows_sys::Win32::System::Threading::GetCurrentThreadId();
            let hook: HHOOK = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(low_level_mouse_proc),
                std::ptr::null_mut(),
                0,
            );

            if hook.is_null() {
                eprintln!("[StashIt] Не удалось установить Win32 Mouse Hook");
                return;
            }
            MOUSE_HOOK = hook;

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
            HOOK_THREAD_ID = 0;
        });
    }

    pub fn stop_mouse_monitor() {
        unsafe {
            if !MOUSE_HOOK.is_null() {
                UnhookWindowsHookEx(MOUSE_HOOK);
                MOUSE_HOOK = std::ptr::null_mut();
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
                    DRAG_START_X.store(pt.x, Ordering::SeqCst);
                    DRAG_START_Y.store(pt.y, Ordering::SeqCst);
                    DIRECTION_CHANGES = 0;
                    LAST_DIR_X = 0;
                    LAST_POS_X = pt.x;
                    LAST_MOUSE_MOVE = Some(Instant::now());
                    LAST_SHAKE_TIME = Some(Instant::now());
                }
                WM_LBUTTONUP => {
                    IS_LBUTTON_DOWN.store(false, Ordering::SeqCst);
                    DIRECTION_CHANGES = 0;
                    LAST_DIR_X = 0;

                    if let Some(ref app) = APP_HANDLE {
                        let _ = app.emit("global-mouse-up", ());
                    }
                }
                WM_MOUSEMOVE if IS_LBUTTON_DOWN.load(Ordering::SeqCst) => {
                    let now = Instant::now();
                    let dx = pt.x - LAST_POS_X;

                    // 1. Детекция "Встряски" (Shake to Show как в Dropover)
                    // Требует резких движений влево-вправо с амплитудой > 25px минимум 3 раза подряд за 600 мс
                    if SHAKE_DETECTION_ENABLED.load(Ordering::Relaxed) {
                        if let Some(shake_start) = LAST_SHAKE_TIME {
                            if now.duration_since(shake_start).as_millis() > 650 {
                                // Сбрасываем счетчик если жест слишком растянут во времени (например обычное выделение текста)
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

                    // 2. Детекция "Начала перетаскивания" (Auto on Drag)
                    // По умолчанию выключена, чтобы не мешать обычному выделению текста мышью.
                    // Если включена пользователем в настройках, требует значительного смещения > 150px
                    if DRAG_DETECTION_ENABLED.load(Ordering::Relaxed) {
                        let start_x = DRAG_START_X.load(Ordering::SeqCst);
                        let start_y = DRAG_START_Y.load(Ordering::SeqCst);
                        let total_dist = ((pt.x - start_x).pow(2) + (pt.y - start_y).pow(2)) as f64;

                        if total_dist > 22500.0 { // 150px дистанция
                            trigger_shelf_appearance(pt.x, pt.y);
                        }
                    }
                }
                _ => {}
            }
        }

        CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
    }

    pub fn trigger_shelf_appearance(cur_x: i32, cur_y: i32) {
        unsafe {
            if let Some(ref app) = APP_HANDLE {
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(is_visible) = window.is_visible() {
                        if !is_visible {
                            let screen_w = GetSystemMetrics(SM_CXSCREEN);
                            let screen_h = GetSystemMetrics(SM_CYSCREEN);

                            let win_w = 340;
                            let win_h = 420;

                            let mut target_x = cur_x + 20;
                            let mut target_y = cur_y - 20;

                            if target_x + win_w > screen_w {
                                target_x = cur_x - win_w - 20;
                            }
                            if target_y + win_h > screen_h {
                                target_y = screen_h - win_h - 20;
                            }
                            if target_y < 10 {
                                target_y = 10;
                            }

                            let _ = window.set_position(PhysicalPosition::new(target_x, target_y));
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = app.emit("shelf-opened", ());
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(windows))]
pub mod win_hook {
    use tauri::AppHandle;
    pub fn start_mouse_monitor(_app_handle: AppHandle) {}
    pub fn trigger_shelf_appearance(_x: i32, _y: i32) {}
}
