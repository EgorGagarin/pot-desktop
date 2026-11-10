use crate::config::{get, set};
use crate::APP;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

pub struct ClipboardMonitorEnableWrapper(pub AtomicBool);

// ─── Windows: event-driven via AddClipboardFormatListener ──────────────────

#[cfg(target_os = "windows")]
mod platform {
    use crate::window::text_translate;
    use crate::APP;
    use log::{error, info};
    use std::sync::atomic::Ordering;
    use tauri::{ClipboardManager, Manager};
    use windows::core::{PCSTR, PSTR};
    use windows::Win32::Foundation::*;
    use windows::Win32::System::DataExchange::*;
    use windows::Win32::System::LibraryLoader::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    const WM_CLIPBOARDUPDATE: u32 = 0x031D;
    const TIMER_CHECK_ID: usize = 1;

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CLIPBOARDUPDATE => {
                if let Some(app) = APP.get() {
                    let enabled = app
                        .state::<super::ClipboardMonitorEnableWrapper>()
                        .0
                        .load(Ordering::Relaxed);
                    if enabled {
                        if let Ok(Some(text)) = app.clipboard_manager().read_text() {
                            // Some clipboard managers re-set identical content
                            let mut pre_text = super::PRE_TEXT.lock().unwrap();
                            if *pre_text != text {
                                info!("Clipboard update received");
                                *pre_text = text.clone();
                                text_translate(text);
                            }
                        }
                    }
                }
                LRESULT(0)
            }
            WM_TIMER if wparam.0 == TIMER_CHECK_ID => {
                if let Some(app) = APP.get() {
                    let enabled = app
                        .state::<super::ClipboardMonitorEnableWrapper>()
                        .0
                        .load(Ordering::Relaxed);
                    if !enabled {
                        let _ = PostMessageA(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
                    }
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, msg, wparam, lparam),
        }
    }

    pub fn start_clipboard_monitor(_app_handle: tauri::AppHandle) {
        std::thread::spawn(move || {
            unsafe {
                let instance = match GetModuleHandleA(None) {
                    Ok(h) => h,
                    Err(_) => return,
                };

                // Mutable null-terminated class name for PSTR compat
                let mut class_name = *b"PotClipMonWnd\0";
                let class_name_pstr = PSTR(class_name.as_mut_ptr());
                let class_name_pcstr = PCSTR(class_name_pstr.0.cast());

                let wc = WNDCLASSA {
                    style: WNDCLASS_STYLES(CS_HREDRAW.0 | CS_VREDRAW.0),
                    lpfnWndProc: Some(wndproc),
                    hInstance: HINSTANCE(instance.0),
                    lpszClassName: class_name_pcstr,
                    ..Default::default()
                };

                // Register class; ignore ALREADY_EXISTS when restarting monitor
                let registered = RegisterClassA(&wc);
                if registered == 0 {
                    let err = GetLastError();
                    if err.0 != ERROR_CLASS_ALREADY_EXISTS.0 {
                        error!("[clipboard] RegisterClassA failed: {:?}", err);
                        return;
                    }
                }

                let hwnd = match CreateWindowExA(
                    WINDOW_EX_STYLE::default(),
                    class_name_pstr,
                    PSTR(core::ptr::null_mut()),
                    WINDOW_STYLE::default(),
                    0,
                    0,
                    0,
                    0,
                    None,
                    None,
                    instance,
                    None,
                ) {
                    Ok(h) => h,
                    Err(_) => return,
                };

                if AddClipboardFormatListener(hwnd).is_err() {
                    let _ = DestroyWindow(hwnd);
                    return;
                }

                // Periodic check whether the monitor got disabled
                SetTimer(hwnd, TIMER_CHECK_ID, 500, None);
                info!("Clipboard listener started (message-driven)");

                let mut msg = MSG::default();
                loop {
                    let ret = GetMessageA(&mut msg, None, 0, 0);
                    if ret.0 == 0 || ret.0 == -1 {
                        break;
                    }
                    let _ = TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }

                KillTimer(hwnd, TIMER_CHECK_ID).ok();
                RemoveClipboardFormatListener(hwnd).ok();
                DestroyWindow(hwnd).ok();
                info!("Clipboard listener stopped");
            }
        });
    }
}

// ─── Non-Windows: adaptive polling fallback ────────────────────────────────

#[cfg(not(target_os = "windows"))]
mod platform {
    use crate::window::text_translate;
    use std::sync::atomic::Ordering;
    use tauri::{ClipboardManager, Manager};

    pub fn start_clipboard_monitor(app_handle: tauri::AppHandle) {
        tauri::async_runtime::spawn(async move {
            let mut previous_text = String::new();
            let mut idle_iterations = 0u32;

            loop {
                let handle = app_handle.app_handle();
                let state = handle.state::<super::ClipboardMonitorEnableWrapper>();
                if !state.0.load(Ordering::Relaxed) {
                    break;
                }

                if let Ok(Some(text)) = app_handle.clipboard_manager().read_text() {
                    if text != previous_text {
                        idle_iterations = 0;
                        previous_text = text.clone();
                        text_translate(text);
                        continue;
                    }
                }

                idle_iterations = idle_iterations.saturating_add(1);
                let delay_ms = if idle_iterations > 20 {
                    2000
                } else if idle_iterations > 10 {
                    1000
                } else {
                    500
                };

                std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            }
        });
    }
}

// Last translated clipboard content (dedup against duplicate update events)
static PRE_TEXT: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

pub use platform::start_clipboard_monitor;

pub fn toggle_clipboard_monitor() {
    let app_handle = APP.get().unwrap();
    let enable_clipboard_monitor = match get("clipboard_monitor") {
        Some(v) => v.as_bool().unwrap(),
        None => {
            set("clipboard_monitor", false);
            false
        }
    };
    let current = !enable_clipboard_monitor;
    // Update Config File
    set("clipboard_monitor", current);
    // Update State and Start Monitor
    let state = app_handle.state::<ClipboardMonitorEnableWrapper>();
    state.0.store(current, Ordering::SeqCst);
    if current {
        start_clipboard_monitor(app_handle.app_handle());
    }
    // Update Tray Menu Status
    app_handle
        .tray_handle()
        .get_item("clipboard_monitor")
        .set_selected(current)
        .unwrap();
    info!("Clipboard Monitor: {}", current);
}
