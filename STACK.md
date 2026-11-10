# Стек проекта

Состояние на текущий момент (форк pot-desktop 3.0.7).

## Фронтенд (webview, React)

- **React 18** + JSX (TypeScript точечно: типы сервисов и утилит)
- **Vite 5** — сборщик, два входа: `index.html` (основное UI) и `daemon.html`
  (скрытое окно-демон для мгновенной реакции глобальных хоткеев)
- **@tauri-apps/api 1.6** — мост в Rust: `invoke`, HTTP-клиент, буфер обмена,
  shell, глобальные хоткеи, апдейтер
- **NextUI 2** + **Tailwind CSS 3** — компоненты и стили
- **framer-motion** / **react-spring** — анимации
- **jotai** — состояние (атомы языков и пр.), **react-router-dom 6** — страницы настроек
- **i18next** — 21 локаль, наполнение через Weblate
- Специфика: **Tesseract.js 5** (WASM-OCR в webview), npm-пакет **ollama**
  (прямой чат с локальной Ollama), **jsqr** (QR-коды), **crypto-js / jose / md5**
  (подписи запросов к китайским API), **react-beautiful-dnd** (перетаскивание
  панелей сервисов)

## Бэкенд (Rust)

- **Tauri 1.8** (линия v1) c allowlist-фичами: fs, shell, clipboard, http,
  updater, notification, global-shortcut, window, tray, protocol-asset
- Плагины Tauri (v1-ветка plugins-workspace):
  - **store** — конфиг: JSON в `%APPDATA%\com.pot-app.desktop`
  - **sql** (sqlite) — история переводов `history.db`
  - **log** — лог `%APPDATA%\com.pot-app.desktop\logs\pot.log`
  - **fs-watch** — живая синхронизация конфига между окнами
  - **single-instance**, **autostart**
- Ключевые крейты:
  - **reqwest** — сетевой слой всех сервисов перевода
  - **lingua** — офлайн-детектор языка (22 языка), движок `local`
  - **windows** crate → **Windows.Media.OCR** — системный OCR на Windows
  - **screenshots** — скриншоты, **arboard** — буфер обмена с картинками
  - **tiny_http** — локальный API на `127.0.0.1:60828` для внешних вызовов
  - **zip** — установка плагинов и бэкапы
  - **reqwest_dav** — WebDAV-бэкап, **selection** — захват выделенного текста

## Архитектурные особенности

- Мультиоконный webview: панель перевода создаёт по панели результата
  на каждый включённый сервис (панели с флагом `enable: false` не рисуются)
- Система плагинов `.potext`: zip с `main.js`, исполняется через `eval`
  в webview; плагину доступны сеть, файлы, SQLite и запуск процессов
  (`run_binary`) — устанавливать только плагины из доверенных источников
- Локальный HTTP-сервер для интеграций (SnipDo, PopClip, curl): список
  эндпоинтов в README
- Автообновления: minisign-подпись, эндпоинты pot-app.com

## Отличия форка

- Русский README как основной, китайский перенесён в `README_CH.md`
- Слот `hotkey_clipboard_monitor` — горячая клавиша переключения мониторинга
  буфера обмена (см. `BUILD.md`)
- `BUILD.md` — сборка и перезапуск, `rebuild.bat` — скрипт пересборки

## Тулчейн сборки (Windows)

- Node.js >= 18 + pnpm, Rust stable-x86_64-pc-windows-msvc (rustup),
  MSVC Build Tools (VS 2019/2022)
- WiX и NSIS tauri-CLI скачивает сам при первой сборке бандлов
- Сборка: `pnpm tauri build` или `rebuild.bat` (подробности в `BUILD.md`)

## Заметка на будущее

Tauri 1.x больше не развивается: мир переехал на Tauri 2 (система разрешений
вместо allowlist, безопасная альтернатива eval-плагинам, новые плагины).
Если форк будет жить долго — миграция на Tauri 2 самый осмысленный апгрейд.
