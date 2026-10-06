# 📝 История версий StashIt (Release Notes)

## [1.4.0] - 2026-10-06
### 🇷🇺 Что нового:
- **Мульти-полки и вкладки (Multi-Stash Tabs)**:
  - **Многопоточная организация файлов**: теперь можно создавать несколько независимых полок для разных задач, проектов или очередей файлов.
  - **Быстрое создание с клавишей Shift**: перетащите файлы и отпустите их с зажатой клавишей `Shift` — приложение автоматически создаст новую полку и поместит файлы в неё. Компактный плавающий бейдж подсказки подскажет эту возможность при драге.
  - **Постоянные (закреплённые) и временные полки**:
    - Любую полку можно закрепить (Pin 📌) — она не закроется автоматически при очистке и сохранится между перезапусками приложения.
    - Временные полки аккуратно самоликвидируются при выгрузке последнего файла.
  - **Эргономичная навигация и горячие клавиши**:
    - Кнопка создания новой полки `[+]` жёстко зафиксирована справа и всегда под рукой.
    - Быстрое переключение полок кликом мыши, колёсиком над панелью вкладок или шорткатами `Ctrl + 1..9` и `Ctrl + Tab`.
    - Создание новой полки: `Ctrl + T`, закрытие активной полки: `Ctrl + W`, переименование: `F2` или двойной клик.
    - Контекстное меню по правому клику (ПКМ): переименование, закрепление/открепление и закрытие полки.
  - **Spring-Loaded Tabs при перетаскивании**:
    - При перетаскивании файлов из Проводника Windows задержка над неактивной вкладкой на 300 мс автоматически открывает её.
    - Прямой сброс файла на шапку неактивной вкладки отправляет файл напрямую в эту полку.
    - Сброс на кнопку `[+]` сразу создаёт новую полку с этими файлами.
- **Комплексное автоматическое тестирование**:
  - 41 автоматический юнит-тест во всех слоях проекта: Rust бэкенд (`cargo test`), Svelte 5 фронтенд (`vitest`) и сайт-презентация Flutter Web (`flutter test`).

---

### 🇬🇧 What's New:
- **Multi-Shelf Tabs (Multi-Stash)**:
  - **Multi-lane file organization**: organize files across multiple independent shelves for different tasks, projects, or batches.
  - **Instant Creation with Shift Key**: drag files and drop them while holding `Shift` — a new shelf is instantly created with those files. A sleek, centered pill badge prompts this shortcut during drag operations.
  - **Pinned vs. Temporary Shelves**:
    - Pin 📌 favorite shelves so they persist even when empty and survive app restarts.
    - Temporary shelves automatically close when their last item is dragged out.
  - **Ergonomic Tab Navigation & Shortcuts**:
    - The new shelf `[+]` button is securely pinned on the right and always accessible.
    - Switch shelves effortlessly via tab clicks, mouse wheel scrolling over tabs, or `Ctrl + 1..9` and `Ctrl + Tab`.
    - New shelf: `Ctrl + T`, close shelf: `Ctrl + W`, inline rename: `F2` or double-click.
    - Right-click context menu: rename, pin/unpin, and close.
  - **Spring-Loaded Tabs during Drag & Drop**:
    - Hovering over an inactive tab for 300 ms while dragging files from Windows Explorer automatically switches to that shelf.
    - Dropping files directly onto an inactive tab header adds items to that specific shelf.
    - Dropping onto the `[+]` button immediately creates a new shelf with the dropped files.
- **Automated Test Suite**:
  - 41 automated unit tests across the stack: Rust backend (`cargo test`), Svelte 5 frontend (`vitest`), and Flutter Web presentation site (`flutter test`).

---

## [1.3.1] - 2026-10-06
### 🇷🇺 Что нового:
- **Упрощение управления окном и скрытия в трей**:
  - **Крестик `✕` теперь аккуратно скрывает карман в трей** (полный эквивалент клавиши `Esc`), не прерывая работу фоновых перехватчиков и горячих клавиш.
  - **Очистка панели инструментов**: убрана лишняя кнопка «—» (минус) из шапки окна, устранена путаница между сворачиванием и выходом.
  - **Завершение работы**: выход из программы осуществляется через привычные системные механизмы — контекстное меню иконки в трее («Выход») или кнопку «Завершить работу StashIt» во вкладке настроек.
  - Удалены избыточные модальные окна подтверждения закрытия.

---

### 🇬🇧 What's New:
- **Simplified Window Management & Tray Behavior**:
  - **Cross button `✕` now hides the shelf to tray** (identical to pressing `Esc`) without terminating background hooks or shortcuts.
  - **Cleaner Header Toolbar**: removed redundant minimize «—» button next to close, preventing confusion.
  - **Application Exit**: full exit is cleanly managed via system tray context menu ("Exit") or via Settings -> "Quit StashIt".
  - Removed cumbersome modal confirmation dialogs.

---

## [1.3.0] - 2026-10-05
### 🇷🇺 Что нового:
- **Двусторонний беспроводной обмен с телефоном по Wi-Fi (Local Drop / Phone Drop)**:
  - **Мгновенное сопряжение по QR-коду**: кнопка со значком телефона в шапке кармана открывает экран сопряжения. Никаких сторонних серверов, облаков, регистрации или внешних утилит — только локальная сеть Wi-Fi.
  - **Передача со смартфона на ПК**: отправка фотографий из галереи, съёмка на камеру напрямую в карман, передача любых документов и файлов, а также отправка текстовых заметок и ссылок. Все отправленные элементы сразу появляются в кармане StashIt на ПК.
  - **Скачивание и шеринг с ПК на смартфон**: все файлы из кармана доступны на экране телефона с миниатюрами; поддержка прямого скачивания файлов и нативного системного меню «Поделиться» (Web Share API).
  - **Автономный микросервер на Rust (On-Demand)**: встроенный легковесный HTTP-сервер поднимается исключительно при открытии экрана QR-кода и автоматически останавливается при закрытии кармана (0% CPU в фоне, строгое сохранение лимита <25 МБ RAM).
  - **Безопасность и изоляция**: каждый запуск генерирует уникальный одноразовый токен безопасности; запросы без действующего токена отклоняются.
  - **Адаптивный мобильный интерфейс**: современная тёмная тема в стиле Fluent, сегментированный переключатель языка `[RU][EN]` с сохранением выбора и полная автономность без внешних скриптов.

---

### 🇬🇧 What's New:
- **Two-Way Wi-Fi Local Drop via QR Code (Phone Drop)**:
  - **Instant QR Pairing**: tap the phone icon in the shelf header to open the pairing screen. Zero cloud dependencies, zero external accounts or server setup — works entirely within your local Wi-Fi.
  - **Phone to PC Upload**: upload photos from gallery, capture live camera shots directly into the shelf, transfer documents, and send text notes or URLs. All received items appear in the desktop shelf instantly.
  - **PC to Phone Download**: browse desktop shelf items on your smartphone with image previews; download files directly or share them via the native mobile share sheet (Web Share API).
  - **On-Demand Rust Micro-Server**: lightweight embedded HTTP server spins up only while the QR screen is active and stops automatically when the shelf is closed (0% CPU idle, preserving strict <25 MB RAM ceiling).
  - **Session Token Security**: random one-time security token generated per launch prevents unauthorized local network access.
  - **Responsive Mobile Web UI**: sleek dark Fluent-inspired mobile web app, segmented `[RU][EN]` language switch with persistent preference, and zero external CDN dependencies.

---

## [1.2.1] - 2026-10-05
### 🇷🇺 Что нового:
- **Полная поддержка мультимониторных конфигураций (Multi-Monitor Support)**:
  - **Динамическое определение активного монитора**: встряхивание (Shake), глобальный хоткей (`Ctrl + Shift + Space`) и двойное нажатие клавиши (`2× Ctrl` / `2× Shift`) теперь мгновенно открывают карман ровно на том мониторе, где находится курсор в момент действия.
  - **Позиционирование в рабочей области (`rcWork`)**: окно кармана позиционируется строго внутри границ текущего экрана с учётом индивидуального положения панели задач каждого монитора.
  - **Интеллектуальный Edge Dock на стыках экранов**:
    - **Сквозной проход на межмониторных стыках**: при перемещении курсора или перетаскивании файлов с экрана на экран карман не выскакивает ложно и не мешает движению мыши.
    - **Внешние кромки**: на физических внешних границах стола (где за кромкой нет соседа) Edge Dock открывает карман мгновенно.
    - **Одиночный монитор**: при наличии только одного экрана обе кромки автоматически активны без дополнительных настроек.

---

### 🇬🇧 What's New:
- **Full Multi-Monitor Support**:
  - **Dynamic Active Monitor Detection**: mouse shake (Shake to Show), global shortcuts (`Ctrl + Shift + Space`), and double-tap modifier triggers (`2× Ctrl` / `2× Shift`) now instantly summon the shelf on the exact monitor where the mouse cursor is located.
  - **Precise Work Area Clamping (`rcWork`)**: shelf coordinates are constrained strictly within the active monitor's work area, respecting per-monitor taskbars and virtual desktop offsets.
  - **Intelligent Edge Dock on Multi-Display Setups**:
    - **Seamless Inter-Monitor Seams**: moving cursor or dragging files across monitors will not accidentally trigger the shelf, preventing interference with drag-and-drop between screens.
    - **Outer Screen Edges**: physical outer boundaries without neighbor displays expand the shelf smoothly.
    - **Single Monitor Setup**: on single-display setups, both left and right edges remain automatically active.

---

## [1.2.0] - 2026-10-05
### 🇷🇺 Что нового:
- **4 независимых способа вызова кармана (Issue #4)**:
  1. **Встряхивание мыши (Shake to Show)**: проверенный жест с зажатым ЛКМ при перетаскивании.
  2. **Глобальный хоткей (Global Shortcut)**: мгновенный toggle кармана по сочетанию клавиш (по умолчанию `Ctrl + Shift + Space`, с возможностью выбора `Alt + S`, `Ctrl + Alt + S`, `Win + Alt + S`, `Ctrl + Shift + X`).
  3. **Двойное быстрое нажатие модификатора (Double-tap)**: открытие кармана по быстрому двойному нажатию `Ctrl` или `Shift` (алгоритм защищен от случайных нажатий при обычном использовании `Ctrl+C`, `Ctrl+V`).
  4. **Прилипание к краю экрана (Edge Dock)**: окно автоматически выдвигается навстречу курсору у границы экрана при перетаскивании или удержании курсора; если файлов нет и курсор уходит — аккуратно сворачивается.
- **Вкладки настроек и Раздел «О программе» (по стандарту Kobalt Tools)**:
  - Панель настроек разделена на две удобные вкладки: **[Управление]** (триггеры, автостарт, автоочистка, выход) и **[О программе]** (версия, проверка обновлений, ссылки экосистемы).
  - В контекстное меню системного трея добавлен пункт **«О программе»**, открывающий соответствующую вкладку.
- **Встроенная система проверки обновлений (GitHub Releases)**:
  - Легковесный Zero-Dependency чекер версий на базе PowerShell `Invoke-RestMethod` (строгое сохранение потребления памяти <25 МБ RAM).
  - Сравнение версий SemVer, автоматическое обнаружение ссылок на установщик (`Setup.exe` / `msi`) и портативную версию (`Portable.exe` / `zip`).
  - Нативные тост-уведомления Windows при выходе новой версии с защитой от спама (cooldown).
  - Возможность ручной проверки в один клик и опциональная тихая еженедельная автопроверка.
- **Удаление случайного срабатывания**: упразднен экспериментальный режим открытия кармана по смещению мыши (Auto on Drag), вызывавший ложные срабатывания при выделении текста.

---

### 🇬🇧 What's New:
- **4 Independent Activation Methods (Issue #4)**:
  1. **Mouse Shake (Shake to Show)**: reliable gesture with left mouse button held while dragging.
  2. **Global Hotkey (Global Shortcut)**: instant toggle at mouse cursor via customizable keyboard combinations (default `Ctrl + Shift + Space`, selectable `Alt + S`, `Ctrl + Alt + S`, `Win + Alt + S`, `Ctrl + Shift + X`).
  3. **Double-Tap Modifier**: summon shelf by double-tapping `Ctrl` or `Shift` within 350ms (cancels on other keys to prevent false triggers during `Ctrl+C` / `Ctrl+V`).
  4. **Screen Edge Dock**: shelf automatically slides in when cursor moves to the screen edge; auto-collapses on mouse leave if shelf is empty.
- **Settings Tabs & "About" Section (Kobalt Tools Standard)**:
  - Settings panel organized into two tabs: **[Controls]** (triggers, autostart, auto-clear, quit) and **[About]** (version badge, release updater, ecosystem links).
  - Added direct **"About"** entry to the system tray context menu.
- **Built-in GitHub Release Updater**:
  - Zero-dependency update checker using background PowerShell `Invoke-RestMethod` (preserving strict <25 MB RAM ceiling).
  - SemVer comparison, auto-detecting direct download links for installer (`Setup.exe`/`msi`) and portable builds (`Portable.exe`/`zip`).
  - Native Windows Toast notifications on new releases with intelligent cooldown logic.
  - One-click manual check button and optional silent weekly auto-check.
- **Removed False-Trigger Drag Mode**: removed experimental distance-based "Auto on Drag" trigger to eliminate false positives during text selection.

---

## [1.1.0] - 2026-09-26
### 🇷🇺 Что нового:
- **Умный двойной буфер обмена (`CF_UNICODETEXT` + `CF_HDROP`)**: при копировании текстовой заметки или ссылки из StashIt в системный буфер помещаются одновременно текст и файл. При вставке (`Ctrl+V`) в текстовые редакторы (VS Code, Блокнот, Word, браузер, Telegram) вставляется чистый текст, а при вставке в Проводник Windows или на Рабочий стол создаётся готовый файл `.txt` (или `.url`).
- **Нативный OLE Drag-and-Drop текста в файлы**: перетаскивание текстовых заметок мышью напрямую в окна папок теперь создаёт реальные `.txt` файлы на диске. Реализован `EnumFormatEtc` через API Windows Shell `SHCreateStdEnumFmtEtc` и поддержка разрешений `DROPEFFECT_COPY | DROPEFFECT_MOVE | DROPEFFECT_LINK`.
- **Безопасная очистка временных файлов**: временные файлы в `%TEMP%\StashIt\` очищаются с задержкой 5 секунд, гарантируя отсутствие конфликтов при копировании Проводником.
- **Автоматическая CI/CD сборка релизов**: настроен GitHub Actions workflow для автоматической сборки дистрибутивов при выпуске тегов версий:
  - **Setup.exe** (NSIS инсталлятор)
  - **MSI** (Windows Installer)
  - **Portable.exe** (Автономная портативная версия)

---

### 🇬🇧 What's New:
- **Smart Dual Clipboard (`CF_UNICODETEXT` + `CF_HDROP`)**: copying a text note or link from StashIt puts both plain text and a virtual file into the Windows clipboard simultaneously. Pasting (`Ctrl+V`) into text editors, IDEs, and messengers pastes text, while pasting into Windows Explorer or Desktop creates a `.txt` (or `.url`) file.
- **Native OLE Drag-and-Drop of Text into Files**: dragging text notes directly into folder windows creates actual `.txt` files on disk. Powered by `EnumFormatEtc` via Windows Shell `SHCreateStdEnumFmtEtc` and `DROPEFFECT_COPY | DROPEFFECT_MOVE | DROPEFFECT_LINK`.
- **Safe Delayed Temp Cleanup**: files in `%TEMP%\StashIt\` are cleaned up with a 5-second grace period, preventing Explorer copy race conditions.
- **Automated CI/CD Release Builds**: GitHub Actions workflow automatically builds and publishes 3 distribution flavors for every version tag:
  - **Setup.exe** (NSIS installer)
  - **MSI** (Windows Installer package)
  - **Portable.exe** (Standalone portable executable)

---

## [1.0.2] - 2026-09-08
### Исправлено
- **Зависание кармана при перетаскивании нескольких абзацев текста**: при дропе многострочного текста с веб-страницы или текстового редактора карман теперь создаёт **одну текстовую заметку** вместо того, чтобы нарезать текст на отдельные строки. Если перетаскивается список URL (каждая строка — ссылка), они по-прежнему добавляются как отдельные карточки.
- **Дублирование ID элементов**: атомарный счётчик `ID_COUNTER` в Rust гарантирует уникальность идентификаторов даже при одновременном добавлении нескольких элементов в течение одной миллисекунды. Устранена ошибка `keyed_each_duplicate` в Svelte 5 и последующее зависание реактивного дерева.

## [1.0.1] - 2026-09-06
### Добавлено
- **Поддержка переключения языка интерфейса (RU / EN)**: кнопка переключения языка в заголовке окна с сохранением выбранного языка в `localStorage`. Полный перевод всех подсказок, тултипов, панели настроек, дроп-зоны, карточек файлов, сводки пакета и кнопок действий.

## [1.0.0] - 2026-09-06
### Добавлено
- **Нативное ядро Rust 2021 + Tauri v2**: потребление оперативной памяти менее 25 МБ RAM, мгновенный отклик.
- **Интерфейс Svelte 5**: полностью на синтаксисе Runes (`$state`, `$derived`, `$props`), адаптивный акрил Fluent с поддержкой системной, тёмной и светлой тем.
- **Встряска мыши (Shake to Show)**: низкоуровневый хук Win32 `WH_MOUSE_LL` с распознаванием быстрых колебаний курсора (dx > 25px, 3+ смены направления за 650 мс) и защитой от ложных срабатываний при обычном выделении текста.
- **Опциональный режим Auto on Drag**: мгновенное появление кармана при начале перетаскивания.
- **Захват всей пачки файлов («Перетащить всё»)**: интерактивная мастер-ручка перетаскивания стопки файлов в целевые окна (Проводник, Telegram, графические редакторы).
- **Мультивыделение файлов**: все файлы по умолчанию активны, поддержка быстрого снятия/выделения чекбоксами и горячей клавишей `Ctrl+A`.
- **Гибридный таймер автоочистки кнопки (5 сек)**: после переноса файлов кнопка «Очистить» мягко пульсирует с обратным отсчетом. Если пользователь не вмешивается, карман автоматически очищается и закрывается; при клике или переключении отсчет отменяется.
- **Нативное копирование файлов в буфер Windows**: интеграция с Win32 `CF_HDROP` для прямого копирования файлов и последующей вставки через `Ctrl+V`.
- **Чистый автозапуск**: прямое управление ключом в реестре `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` без UAC-запросов.
- **Поддержка сброса веб-ссылок, текста и картинок из браузера**: свободный прием URL (`text/uri-list`), текста и изображений с автоматическим предотвращением курсора 🚫 (знак стоп) на всей площади окна.
- **Корректное завершение работы без ошибок Win32 (Error 1412)**: корректная остановка низкоуровневого хука мыши `stop_mouse_monitor` с отправкой `WM_QUIT` и предварительное уничтожение окон WebView2 перед выходом процесса.
- **Презентационный сайт на Flutter Web**: двуязычный лендинг (RU/EN) в `website/` с полным описанием возможностей.
