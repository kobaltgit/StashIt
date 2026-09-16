#![allow(non_snake_case)]
//! Нативный OLE drag-out для текстовых карточек и URL.
//! Предоставляет два формата одновременно:
//! - CF_HDROP (15) → временный .txt / .url файл (для Проводника)
//! - CF_UNICODETEXT (13) → raw текст (для редакторов, мессенджеров)

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use tauri::Emitter;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Ole::*;
use windows::Win32::System::SystemServices::MODIFIERKEYS_FLAGS;
use windows::Win32::UI::Shell::DROPFILES;

const CF_HDROP_U: u16 = 15;
const CF_UNICODETEXT_U: u16 = 13;

// HRESULT константы для IDropSource
const S_DRAG_DROP:    HRESULT = HRESULT(0x00040100_u32 as i32);
const S_DRAG_CANCEL:  HRESULT = HRESULT(0x00040101_u32 as i32);
const S_DRAG_DEFCUR:  HRESULT = HRESULT(0x00040102_u32 as i32);
const E_DV_FORMATETC: HRESULT = HRESULT(0x80040064_u32 as i32);
const E_NOTIMPL_HR:   HRESULT = HRESULT(0x80004001_u32 as i32);

// ================================================================
// TextDropSource — IDropSource
// ================================================================

#[implement(IDropSource)]
struct TextDropSource;

impl IDropSource_Impl for TextDropSource_Impl {
    fn QueryContinueDrag(
        &self,
        fescapepressed: BOOL,
        grfkeystate: MODIFIERKEYS_FLAGS,
    ) -> HRESULT {
        if fescapepressed.as_bool() {
            return S_DRAG_CANCEL;
        }
        // MK_LBUTTON=0x0001, MK_RBUTTON=0x0002 — если ни одна не нажата, дроп
        if grfkeystate.0 & 0x0003 == 0 {
            S_DRAG_DROP
        } else {
            S_OK
        }
    }

    fn GiveFeedback(&self, _dweffect: DROPEFFECT) -> HRESULT {
        S_DRAG_DEFCUR
    }
}

// ================================================================
// TextDataObject — IDataObject
// Хранит два HGLOBAL буфера; GetData дублирует их при каждом запросе.
// ================================================================

#[implement(IDataObject)]
struct TextDataObject {
    hdrop: usize, // CF_HDROP  HGLOBAL сохранённый как usize (обход !Send)
    utext: usize, // CF_UNICODETEXT HGLOBAL
}

impl IDataObject_Impl for TextDataObject_Impl {
    fn GetData(&self, pformatetc: *const FORMATETC) -> Result<STGMEDIUM> {
        unsafe {
            let cf = (*pformatetc).cfFormat;
            let src = if cf == CF_HDROP_U {
                HGLOBAL(self.hdrop as *mut _)
            } else if cf == CF_UNICODETEXT_U {
                HGLOBAL(self.utext as *mut _)
            } else {
                return Err(Error::from_hresult(E_DV_FORMATETC));
            };

            let dup = dup_hglobal(src)?;
            Ok(STGMEDIUM {
                tymed: TYMED_HGLOBAL.0 as u32,
                u: STGMEDIUM_0 { hGlobal: dup },
                pUnkForRelease: std::mem::ManuallyDrop::new(None),
            })
        }
    }

    fn GetDataHere(
        &self, _pformatetc: *const FORMATETC, _pmedium: *mut STGMEDIUM,
    ) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL_HR))
    }

    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> HRESULT {
        let cf = unsafe { (*pformatetc).cfFormat };
        if cf == CF_HDROP_U || cf == CF_UNICODETEXT_U {
            S_OK
        } else {
            E_DV_FORMATETC
        }
    }

    fn GetCanonicalFormatEtc(
        &self, _: *const FORMATETC, _: *mut FORMATETC,
    ) -> HRESULT {
        E_NOTIMPL_HR
    }

    fn SetData(
        &self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL,
    ) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL_HR))
    }

    fn EnumFormatEtc(&self, _dwdirection: u32) -> Result<IEnumFORMATETC> {
        Err(Error::from_hresult(E_NOTIMPL_HR))
    }

    fn DAdvise(
        &self, _: *const FORMATETC, _: u32, _: Option<&IAdviseSink>,
    ) -> Result<u32> {
        Err(Error::from_hresult(E_NOTIMPL_HR))
    }

    fn DUnadvise(&self, _: u32) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL_HR))
    }

    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        Err(Error::from_hresult(E_NOTIMPL_HR))
    }
}

// ================================================================
// HGLOBAL helpers
// ================================================================

/// Дублирует HGLOBAL — возвращаемый дубликат принадлежит вызывающей стороне.
unsafe fn dup_hglobal(src: HGLOBAL) -> Result<HGLOBAL> {
    let size = GlobalSize(src);
    let dst = GlobalAlloc(GHND, size)?;
    let sp = GlobalLock(src) as *const u8;
    let dp = GlobalLock(dst) as *mut u8;
    if !sp.is_null() && !dp.is_null() {
        std::ptr::copy_nonoverlapping(sp, dp, size);
    }
    let _ = GlobalUnlock(src);
    let _ = GlobalUnlock(dst);
    Ok(dst)
}

/// DROPFILES + путь UTF-16 в GlobalAlloc памяти.
fn build_hdrop(path: &str) -> Result<HGLOBAL> {
    unsafe {
        // path\0\0 — двойной нуль = конец списка HDROP
        let wide: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain([0u16, 0u16])
            .collect();

        let df_size = std::mem::size_of::<DROPFILES>();
        let hg = GlobalAlloc(GHND, df_size + wide.len() * 2)?;
        let ptr = GlobalLock(hg) as *mut u8;
        if ptr.is_null() {
            let _ = GlobalFree(hg);
            return Err(Error::from_hresult(HRESULT(0x8007000E_u32 as i32)));
        }
        let df = ptr as *mut DROPFILES;
        (*df).pFiles = df_size as u32;
        (*df).fWide = BOOL(1);
        // pt и fNC — уже нули (GHND = GMEM_MOVEABLE|GMEM_ZEROINIT)
        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr.add(df_size) as *mut u16, wide.len());
        let _ = GlobalUnlock(hg);
        Ok(hg)
    }
}

/// UTF-16 строка с нулём в конце в GlobalAlloc памяти.
fn build_utext(text: &str) -> Result<HGLOBAL> {
    unsafe {
        let wide: Vec<u16> = OsStr::new(text)
            .encode_wide()
            .chain([0u16])
            .collect();
        let hg = GlobalAlloc(GHND, wide.len() * 2)?;
        let ptr = GlobalLock(hg) as *mut u16;
        if ptr.is_null() {
            let _ = GlobalFree(hg);
            return Err(Error::from_hresult(HRESULT(0x8007000E_u32 as i32)));
        }
        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
        let _ = GlobalUnlock(hg);
        Ok(hg)
    }
}

// ================================================================
// Temp file (дублирует логику crate::create_temp_file)
// ================================================================

fn make_temp_file(
    name: &str,
    content: &str,
    is_url: bool,
) -> std::result::Result<String, String> {
    let dir = crate::get_stashit_temp_dir()?;
    let safe = crate::sanitize_filename(name);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let fname = if is_url {
        format!("{}_{}.url", safe, ts)
    } else {
        format!("{}_{}.txt", safe, ts)
    };
    let path = dir.join(&fname);
    let body = if is_url {
        format!/// Запускает OLE drag с двумя форматами: CF_HDROP + CF_UNICODETEXT.
/// DoDragDrop выполняется на главном UI-потоке (где живут мышиные события
/// и уже инициализирован OLE). Возвращает немедленно.
pub fn start_text_drag(
    window: tauri::WebviewWindow,
    name: String,
    content: String,
    is_url: bool,
) {
    // 1. Подготовка данных в вызывающем потоке (быстро, не блокирует UI)
    let temp_path = match make_temp_file(&name, &content, is_url) {
        Ok(p) => p,
        Err(e) => { eprintln!("[text_drag] temp file: {e}"); return; }
    };

    let hdrop_hg = match build_hdrop(&temp_path) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("[text_drag] hdrop: {e:?}");
            let _ = std::fs::remove_file(&temp_path);
            return;
        }
    };

    let utext_hg = match build_utext(&content) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("[text_drag] utext: {e:?}");
            unsafe { let _ = GlobalFree(hdrop_hg); }
            let _ = std::fs::remove_file(&temp_path);
            return;
        }
    };

    // HGLOBAL не реализует Send — передаём как usize
    let hdrop_raw = hdrop_hg.0 as usize;
    let utext_raw = utext_hg.0 as usize;

    // 2. DoDragDrop ДОЛЖЕН выполняться на главном (UI) потоке:
    //    - там живут мышиные сообщения (без них drag сразу отменяется)
    //    - OLE уже инициализирован фреймворком (не нужен OleInitialize)
    //    DoDragDrop блокирует главный поток, но внутренне прокачивает сообщения
    let _ = window.run_on_main_thread(move || unsafe {
        let hdrop = HGLOBAL(hdrop_raw as *mut _);
        let utext = HGLOBAL(utext_raw as *mut _);

        let data_obj: IDataObject = TextDataObject {
            hdrop: hdrop_raw,
            utext: utext_raw,
        }
        .into();
        let drop_src: IDropSource = TextDropSource.into();

        let mut dw_effect = DROPEFFECT_COPY;
        let _hr = DoDragDrop(&data_obj, &drop_src, DROPEFFECT_COPY, &mut dw_effect);

        let _ = window.emit("drag-out-completed", vec![temp_path.clone()]);
        let _ = std::fs::remove_file(&temp_path);

        drop(data_obj);
        drop(drop_src);
        let _ = GlobalFree(hdrop);
        let _ = GlobalFree(utext);
    });
}


            // 5. Уведомить Svelte + удалить temp файл
            let _ = window.emit("drag-out-completed", vec![temp_path.clone()]);
            let _ = std::fs::remove_file(&temp_path);

            // 6. Освобождение: сначала COM-объекты, затем оригинальные HGLOBAL
            //    (дубликаты, выданные через GetData, принадлежат получателям)
            drop(data_obj);
            drop(drop_src);
            let _ = GlobalFree(hdrop_hg);
            let _ = GlobalFree(utext_hg);

            OleUninitialize();
        }
    });
}
