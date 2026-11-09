# Сборка и запуск форка pot-desktop

Краткая инструкция для этого форка (Windows). Отличия форка от апстрима:
русский README (`README.md`), китайский перенесён в `README_CH.md`,
горячая клавиша переключения мониторинга буфера обмена (`Ctrl+Alt+B`,
слот `hotkey_clipboard_monitor` в настройках).

## Требования

- Node.js >= 18, pnpm >= 8.5
- Rust >= 1.80 (stable-x86_64-pc-windows-msvc, ставится через [rustup](https://rustup.rs))
- MSVC Build Tools (Visual Studio 2019/2022 с компонентом C++)

Проверка окружения:

```bash
node --version && pnpm --version && cargo --version
```

Если `cargo` не находится в терминале — добавьте `%USERPROFILE%\.cargo\bin` в PATH.

## Сборка

```bash
pnpm install        # один раз
pnpm tauri build    # сборка release + установщики
```

Артефакты после сборки:

- бинарник: `src-tauri\target\release\pot.exe`
- установщик NSIS: `src-tauri\target\release\bundle\nsis\pot_3.0.7_x64-setup.exe`
- MSI: `src-tauri\target\release\bundle\msi\pot_3.0.7_x64_en-US.msi`

> Ошибка `A public key has been found, but no private key ... TAURI_PRIVATE_KEY`
> в самом конце сборки — это подпись артефактов автообновления для публикации.
> Установщик к этому моменту уже собран, для локального использования это не важно.

Быстрая проверка без установки — запустить бинарник прямо из сборки:

```bash
src-tauri\target\release\pot.exe
```

Режим разработки (горячая перезагрузка UI, порт 1420):

```bash
pnpm tauri dev
```

## Перезапуск / обновление установленной версии

1. Закройте pot: иконка в трее → Quit, или

    ```bash
    taskkill /F /IM pot.exe
    ```

    Это обязательно: работающий pot блокирует свой `exe` — сборка и установка
    упадут с `os error 5 (Отказано в доступе)`.

2. Варианты:

    - **Быстрый** — запустить свежий бинарник из сборки
      (`src-tauri\target\release\pot.exe`), установленная версия остаётся нетронутой.
    - **Постоянный** — запустить установщик `pot_3.0.7_x64-setup.exe`
      (потребует UAC), он обновит версию в `Program Files`.

Настройки при переустановке не теряются: конфиг живёт в
`%APPDATA%\com.pot-app.desktop\config.json` и удаляется только вручную.

## Проверка, что собралось то, что нужно

Лог приложения: `%APPDATA%\com.pot-app.desktop\logs\pot.log`.
При старте там видны все регистрации горячих клавиш, например:

```
Registered global shortcut: Ctrl+Alt+B for hotkey_clipboard_monitor
```

`Failed to register global shortcut` означает, что комбинация занята другим
приложением — смените её в настройках pot (Настройки → Горячие клавиши).

## Грабли

- **Мало места на диске** — линковка падает с `LNK1180: insufficient disk space`.
  Сборка требует пару ГБ свободных; артефакты отладочной проверки чистятся
  удалением `src-tauri\target\debug` (~1+ ГБ), кеш релизной сборки
  (`src-tauri\target\release\deps`) удалять не нужно — иначе пересборка пойдёт
  с нуля.
- Если после сборки поведение «старое» — убедитесь, что запускаете свежий
  `pot.exe` (дата файла) и что pot был закрыт во время сборки.
- git может показывать `Cargo.toml` изменённым без содержимого — это шум
  переноса строк (CRLF/LF), коммитить его не нужно.
