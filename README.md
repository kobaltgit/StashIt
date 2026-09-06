# 🛡️ StashIt

<p align="center">
  <strong>Легковесный плавающий карман (Drag-and-Drop Shelf в стиле macOS Dropover / Yoink) для Windows 10 & 11 на Rust и Tauri v2.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-blue.svg" alt="Version 1.0.0" />
  <a href="https://kobaltgit.github.io/stashit/"><img src="https://img.shields.io/badge/Website-Live_Demo-38bdf8.svg?logo=flutter" alt="Live Website" /></a>
  <img src="https://img.shields.io/badge/Rust-Tauri_v2-orange.svg" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/Frontend-Svelte_5-ff3e00.svg" alt="Svelte 5" />
  <img src="https://img.shields.io/badge/Platform-Windows_10%2F11-0078d7.svg" alt="Windows" />
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License MIT" />
</p>

---

## 💡 О проекте

**StashIt** — это легковесный плавающий карман для временного хранения файлов, картинок, ссылок и текста. В состоянии покоя приложение **100% невидимо** и не загромождает рабочий стол. При перетаскивании контента карман материализуется прямо у курсора по встряске мыши или началу перемещения.

Потребляет **менее 25 МБ RAM** благодаря нативному ядру на **Rust 2021** и ультрасовременному фронтенду на **Svelte 5** под движком **Tauri v2**.

---

## ✨ Ключевые возможности

- 🫨 **Встряска мыши (Shake to Show)**: зажмите файл и качните мышь — карман появится прямо рядом с курсором (как в Dropover).
- 🎯 **ЭКСПЕРИМЕНТ** **Автопоявление при перетаскивании (Auto on Drag)**: режим мгновенного открытия при старте Drag & Drop (как в Yoink).
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

### Запуск промо-сайта

```bash
cd website
flutter run -d chrome
```

---

## 📄 Лицензия

Распространяется под лицензией [MIT](LICENSE).
