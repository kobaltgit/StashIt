<p align="center">
  <img src="icon.svg" width="96" height="96" alt="StashIt Logo" />
  <h1 align="center">StashIt</h1>
  <strong>Легковесный плавающий карман (Drag-and-Drop Shelf в стиле macOS Dropover / Yoink) для Windows 10 & 11 на Rust и Tauri v2.</strong><br/>
  <em>Lightweight drag-and-drop shelf (Dropover / Yoink alternative) for Windows 10 & 11 built with Rust & Tauri v2.</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.1-blue.svg" alt="Version 1.0.1" />
  <a href="https://kobaltgit.github.io/StashIt/"><img src="https://img.shields.io/badge/Website-Live_Demo-38bdf8.svg?logo=flutter" alt="Live Website" /></a>
  <img src="https://img.shields.io/badge/Rust-Tauri_v2-orange.svg" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/Frontend-Svelte_5-ff3e00.svg" alt="Svelte 5" />
  <img src="https://img.shields.io/badge/Platform-Windows_10%2F11-0078d7.svg" alt="Windows" />
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License MIT" />
</p>

<p align="center">
  <a href="#-о-проекте">🇷🇺 Русский</a> • <a href="#-about-the-project">🇬🇧 English</a>
</p>

---

## 💡 О проекте

**StashIt** — это легковесный плавающий карман для временного хранения файлов, картинок, ссылок и текста. В состоянии покоя приложение **100% невидимо** и не загромождает рабочий стол. При перетаскивании контента карман материализуется прямо у курсора по встряске мыши или началу перемещения.

Потребляет **менее 25 МБ RAM** благодаря нативному ядру на **Rust 2021** и ультрасовременному фронтенду на **Svelte 5** под движком **Tauri v2**.

---

## ✨ Ключевые возможности

- 🫨 **Встряска мыши (Shake to Show)**: зажмите файл и качните мышь — карман появится прямо рядом с курсором (как в Dropover).
- 🎯 **ЭКСПЕРИМЕНТ** **Автопоявление при перетаскивании (Auto on Drag)**: режим мгновенного открытия при старте Drag & Drop (как в Yoink).
- 🌐 **Двуязычный интерфейс (RU / EN)**: переключение языка в шапке с сохранением настроек.
- 🎨 **Тёмная, Светлая и Системная темы**: акриловый Fluent-интерфейс с поддержкой переключения оформления и автоматической адаптацией под Windows 11.
- 📦 **Захват всей пачки («Перетащить всё»)**: выделение всех файлов по умолчанию (`Ctrl+A`), возможность перетаскивать как отдельные файлы, так и всю стопку сразу через мастер-ручку.
- ⏳ **Умный таймер автоочистки (5 сек)**: после переноса файлов кнопка «Очистить» мягко пульсирует с обратным отсчетом. Если не вмешиваться — карман очищается и скрывается сам; при клике или наведении отсчет останавливается.
- 📋 **Нативное копирование файлов**: помещение реальных системных файлов в буфер Windows (`CF_HDROP`) для быстрой вставки через `Ctrl+V`.
- ⚡ **Высокая скорость и легкость**: потребление до 25 МБ RAM, 0% нагрузки на процессор в фоне, без Electron и Chromium-сервисов.
- 🚀 **Чистая автозагрузка**: запуск через ветку реестра `HKCU` без назойливых запросов UAC.

---

## 🛠️ Стек технологий

- **Backend**: Rust 2021, Tauri v2, Win32 API (`WH_MOUSE_LL` low-level hook, Windows Registry).
- **Frontend**: Svelte 5 (Runes `$state`, `$derived`, `$props`), TypeScript, Vite.
- **Landing Page**: Flutter Web (`website/`), промо-лендинг для публикации на GitHub Pages.

---

## 🚀 Сборка и запуск

### Требования

- Node.js 18+ и npm
- Rust 1.77+ и cargo

### Запуск в режиме разработки

```bash
npm install
npm run tauri dev
```

### Сборка релиза (Portable, Setup, MSI)

```bash
npm run build:release
```
Релизные файлы появятся в каталоге `.output/`.

### Запуск промо-сайта

```bash
cd website
flutter run -d chrome
```

---

<a name="-about-the-project"></a>
## 🇬🇧 English Description

### 💡 About The Project

**StashIt** is an ultra-lightweight drag-and-drop temporary shelf for Windows 10 & 11 (similar to macOS Dropover or Yoink). Stash files, images, URLs, and text snippets while you navigate between folders and workspaces. When idle, the app is **100% invisible** and never clutters your screen. Whenever you drag something, a quick shake of the mouse or drag movement summons the shelf directly under your cursor.

Consumes **under 25 MB RAM** thanks to a native **Rust 2021** core and modern **Svelte 5** frontend on **Tauri v2**.

---

### ✨ Key Features

- 🫨 **Shake to Show**: Hold and wiggle your cursor while dragging files — the shelf immediately appears right next to your mouse pointer.
- 🎯 **Auto on Drag (Experimental)**: Optional mode to automatically summon the shelf whenever a drag operation begins.
- 🌐 **Multilingual (RU / EN)**: Built-in instant language switcher in the header with persistent preferences.
- 🎨 **Dark, Light & System Themes**: Windows 11 Fluent Acrylic (`backdrop-filter: blur`) interface with automatic system theme adaptation.
- 📦 **Batch Drag Handle**: Grab and drag all stashed files or multi-selected items into target folders/apps at once.
- ⏳ **Smart Auto-Clear Timer (5s)**: Pulsing auto-clear countdown starts after dragging files out; automatically clears and hides unless manually stopped.
- 📋 **Native Clipboard Integration**: Places real file paths (`CF_HDROP`) directly onto the Windows clipboard for instant `Ctrl+V` pasting.
- ⚡ **Lightweight & High Performance**: Under 25 MB RAM footprint, zero background CPU drain, no Chromium overhead.
- 🚀 **Clean Autostart**: User-level `HKCU` registry startup without annoying UAC prompts.

---

### 🛠️ Tech Stack

- **Backend**: Rust 2021, Tauri v2, Win32 API (`WH_MOUSE_LL` hook, registry access).
- **Frontend**: Svelte 5 (Runes syntax), TypeScript, Vite.
- **Showcase Website**: Flutter Web (`website/`) published to GitHub Pages.

---

### 🚀 Quick Start & Build

```bash
# Install dependencies
npm install

# Run desktop dev environment
npm run tauri dev

# Build standalone release binaries (placed into .output/)
npm run build:release
```

---

## 📄 Лицензия / License

Distributed under the [MIT](LICENSE) License.
