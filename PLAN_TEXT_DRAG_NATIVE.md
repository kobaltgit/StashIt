# План: нативный двойной формат для перетаскивания текста и ссылок

**Дата:** 2026-09-08  
**Версия цели:** 1.0.3 (уже выставлена в Cargo.toml и tauri.conf.json)  
**Статус:** ожидает реализации

---

## Контекст и проблема

В версии 1.0.3 был добавлен `create_temp_file` + `drag_item` для перетаскивания
текста/ссылок в Проводник. Это сломало drag в текстовые редакторы:
раньше для них использовался HTML5 `dataTransfer.setData("text/plain", ...)`,
теперь текст просто недоступен при дропе.

Решение — собственный `IDataObject` (Win32 COM), который одновременно предоставляет:
- `CF_HDROP` (format=15) — путь к `.txt`/`.url` файлу — для Проводника, файловых менеджеров
- `CF_UNICODETEXT` (format=13) — raw текст — для редакторов, мессенджеров, браузеров

Приложение-получатель само выбирает нужный формат. Для пользователя — прозрачно.

---

## Текущее состояние кода

### `src-tauri/src/lib.rs`
- Команды: `drag_item`, `create_temp_file`, `copy_to_clipboard` и стандартные
- `create_temp_file(name, content, is_url)` — создаёт файл в `%TEMP%\StashIt\`, возвращает путь
- `drag_item(window, paths)` — OLE drag через крейт `drag`, в коллбэке авто-удаляет temp-файлы
- `cleanup_temp_dir()` — вызывается при старте

### `src/App.svelte`
- `handleItemDragStart`: для text/url — `create_temp_file` затем `drag_item`
- `handleDragAll`: для смешанных пачек — создаёт temp-файлы, затем `drag_item`

---

## Архитектура решения

### Новый файл: `src-tauri/src/text_drag.rs`

Публичный интерфейс:

```rust
pub fn start_text_drag(
    window: tauri::WebviewWindow,
    name: String,
    content: String,
    is_url: bool,
);
```

#### TextDropSource (IDropSource)

```rust
#[implement(IDropSource)]
struct TextDropSource;

impl IDropSource_Impl for TextDropSource_Impl {
    fn QueryContinueDrag(&self, fescapepressed: BOOL, grfkeystate: MODIFIERKEYS_FLAGS) -> HRESULT {
        if fescapepressed.as_bool() { return DRAGDROP_S_CANCEL; }
        if grfkeystate.0 & 0x0003 == 0 { DRAGDROP_S_DROP } else { S_OK }
    }
    fn GiveFeedback(&self, _dweffect: DROPEFFECT) -> HRESULT {
        DRAGDROP_S_USEDEFAULTCURSORS
    }
}
```

#### TextDataObject (IDataObject)

```rust
#[implement(IDataObject)]
struct TextDataObject {
    hdrop_hglobal: HGLOBAL,   // CF_HDROP (format=15)
    utext_hglobal: HGLOBAL,   // CF_UNICODETEXT (format=13)
}
```

Реализует `IDataObject_Impl`:
- `GetData(CF_HDROP=15)` — возвращает hdrop_hglobal
- `GetData(CF_UNICODETEXT=13)` — возвращает utext_hglobal
- `QueryGetData` — S_OK для 13 и 15, иначе S_FALSE
- Всё остальное — E_NOTIMPL

#### Подготовка HGLOBAL буферов

`build_hdrop_hglobal(path: &str) -> HGLOBAL`:
```
Layout: [DROPFILES (20 bytes)] [wide path] [\0] [\0]
DROPFILES.pFiles = 20
DROPFILES.fWide = 1
DROPFILES.pt = {0,0}
DROPFILES.fNC = 0
```

`build_unicode_hglobal(text: &str) -> HGLOBAL`:
```
Layout: [UTF-16 chars] [\0\0]
```

#### Функция start_text_drag

```
1. std::thread::spawn
2. CoInitializeEx(None, COINIT_APARTMENTTHREADED)
3. create_temp_file_internal(name, content, is_url) -> temp_path
4. build_hdrop_hglobal(temp_path) -> hdrop
5. build_unicode_hglobal(content) -> utext
6. TextDataObject { hdrop, utext }.into::<IDataObject>()
7. TextDropSource.into::<IDropSource>()
8. DoDragDrop(data_obj, drop_src, DROPEFFECT_COPY, &mut dw_effect)  // БЛОКИРУЕТ
9. window.emit("drag-out-completed", [temp_path])
10. std::fs::remove_file(temp_path)
11. CoUninitialize()
```

ВАЖНО по памяти: при GetData возвращаем дубликат HGLOBAL (GlobalAlloc + copy),
а не оригинал — получатель сам освободит дубликат. Оригиналы освобождаем в Drop.

---

## Изменения в lib.rs

1. Добавить `mod text_drag;` вверху
2. Убрать команду `create_temp_file` (становится внутренней в text_drag)
3. Убрать `create_temp_file` из `generate_handler!`
4. Добавить команду:

```rust
#[tauri::command]
fn drag_text_item(window: tauri::WebviewWindow, name: String, content: String, is_url: bool) {
    text_drag::start_text_drag(window, name, content, is_url);
}
```

5. Добавить `drag_text_item` в `generate_handler!`

---

## Изменения в App.svelte

### handleItemDragStart (text/url ветка)

Было (два вызова):
```typescript
const tempPath = await invoke<string>("create_temp_file", { name, content, isUrl });
await invoke("drag_item", { paths: [tempPath] });
```

Стало (один вызов):
```typescript
await invoke("drag_text_item", {
  name: item.name,
  content: item.text_preview ?? "",
  isUrl: item.kind === "url",
});
```

### handleDragAll (для пачек)

Для пачки text+files лучший подход:
- Создать temp-файлы для text/url через отдельную публичную команду `prepare_temp_file`
  (она только создаёт файл, не запускает drag)
- Объединить пути с file paths
- Один вызов `drag_item` со всеми путями

ИЛИ (упрощённо): `start_text_drag` принимает `Vec` элементов, создаёт все temp-файлы
и одним DoDragDrop тащит через CF_HDROP с несколькими путями.

Рекомендуется второй подход: одна команда `drag_text_items_batch(items: Vec<{name,content,is_url}>)`.

---

## Необходимые импорты в text_drag.rs

```rust
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::Memory::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GHND};
use windows::Win32::System::Ole::*;
use windows::Win32::UI::Shell::DROPFILES;
```

---

## Критерии приёмки

- [ ] Текст в Проводник -> файл .txt
- [ ] URL на рабочий стол -> файл .url
- [ ] Текст в Notepad -> вставился текст
- [ ] Текст в VS Code -> вставился текст
- [ ] URL в Telegram -> вставилась ссылка
- [ ] Смешанная пачка (файлы+текст) в папку -> всё оказалось в папке
- [ ] Нет утечек памяти (temp удалены, HGLOBAL освобождены)
- [ ] Таймер автоочистки работает
- [ ] cargo build без ошибок
- [ ] RAM <= 25 MB

---

## Порядок реализации

1. Создать src-tauri/src/text_drag.rs
2. Обновить lib.rs
3. Обновить App.svelte
4. cargo build (исправить ошибки компиляции)
5. npm run tauri build
6. Скопировать в .output/
7. git commit + push

---

## Прогресс выполнения

### ✅ Шаг 1: src-tauri/src/text_drag.rs — СОЗДАН
- TextDropSource (IDropSource): QueryContinueDrag + GiveFeedback
- TextDataObject (IDataObject): GetData(CF_HDROP/CF_UNICODETEXT), QueryGetData, GetCanonicalFormatEtc, остальные E_NOTIMPL
- dup_hglobal() — дублирование HGLOBAL для GetData
- uild_hdrop(path) — DROPFILES + wide path
- uild_utext(text) — UTF-16 строка
- make_temp_file() — создание .txt / .url через crate::get_stashit_temp_dir / sanitize_filename
- start_text_drag() — спавн потока, OleInitialize, DoDragDrop, cleanup, OleUninitialize

Исправленные ошибки компиляции:
- QueryGetData -> возвращает HRESULT (не Result<()>)
- GetCanonicalFormatEtc -> 3 параметра + возвращает HRESULT
- STGMEDIUM.tymed -> u32 (TYMED_HGLOBAL.0 as u32)
- STGMEDIUM.pUnkForRelease -> ManuallyDrop::new(None)

### ✅ Шаг 2: src-tauri/src/lib.rs — ОБНОВЛЁН
- sanitize_filename -> pub(crate)
- get_stashit_temp_dir -> pub(crate)
- #[cfg(windows)] mod text_drag; добавлен
- Команда drag_text_item добавлена и зарегистрирована
- create_temp_file оставлен (используется handleDragAll для батч-дропа)

### ✅ Шаг 3: src/App.svelte — ОБНОВЛЁН
- handleItemDragStart text/url ветка: вызывает drag_text_item (не create_temp_file+drag_item)
- handleDragAll (батч): оставлен на create_temp_file+drag_item (батч в редакторы не актуален)

### ✅ Шаг 4: cargo check — OK
- Компиляция прошла без ошибок

### 🔄 Шаг 5: 
pm run tauri build — В ПРОЦЕССЕ
- Запущен в фоне, ждём результата

### ⬜ Шаг 6: Скопировать в .output/
### ⬜ Шаг 7: git commit + push
