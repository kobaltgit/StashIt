# 🛡️ StashIt

<p align="center">
  <strong>Легковесный плавающий карман (Drag-and-Drop Shelf) для Windows 10 & 11 на Rust и Tauri v2.</strong>
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

**StashIt** — это легковесная нативная утилита для Windows 10 & 11, созданная на базе **Rust** и **Svelte 5** с использованием движка **Tauri v2**. Приложение работает в системном трее, потребляет **менее 25 МБ RAM** и запускается мгновенно.

---

## ✨ Ключевые возможности

* 🪟 **Fluent Glassmorphism**: Современное акриловое окно с размытием, адаптирующееся под тему Windows.
* ⚡ **Высокая скорость и легкость**: Нативный бинарный файл без Electron.
* 🔒 **100% Локально**: Данные не покидают ваш компьютер.
* 🚀 **Чистая автозагрузка**: Автостарт через реестр текущего пользователя (`HKCU`) без запросов прав администратора (UAC).

---

## 🛠️ Стек технологий

* **Backend**: Rust 2021, Tauri v2, Windows API.
* **Frontend**: Svelte 5 (Runes `$state`, `$derived`, `$effect`), TypeScript, Vite.
* **Landing Page**: Flutter Web (`website/`), промо-лендинг для публикации на GitHub Pages.

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