<img width="200px" src="public/icon.svg" align="left"/>

# Pot (милый переводчик)

> 🌈 Кроссплатформенное приложение для перевода выделенного текста ([Telegram-группа](https://t.me/pot_app))

![License](https://img.shields.io/github/license/pot-app/pot-desktop.svg)
![Tauri](https://img.shields.io/badge/Tauri-1.6.8-blue?logo=tauri)
![JavaScript](https://img.shields.io/badge/-JavaScript-yellow?logo=javascript&logoColor=white)
![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/-Windows-blue?logo=windows&logoColor=white)
![MacOS](https://img.shields.io/badge/-macOS-black?&logo=apple&logoColor=white)
![Linux](https://img.shields.io/badge/-Linux-yellow?logo=linux&logoColor=white)

<br/>
<hr/>
<div align="center">

<h3><a href='./README_CH.md'>中文</a> | <a href='./README_EN.md'>English</a> | Русский | <a href='./README_KR.md'> 한글 </a></h3>

<table>
<tr>
    <td> <img src="asset/1.png">
    <td> <img src="asset/2.png">
    <td> <img src="asset/3.png">
</table>

# Содержание

</div>

-   [Использование](#использование)
-   [Возможности](#возможности)
-   [Поддерживаемые сервисы](#поддерживаемые-сервисы)
-   [Система плагинов](#система-плагинов)
-   [Установка](#установка)
-   [Внешние вызовы](#внешние-вызовы)
-   [Поддержка Wayland](#поддержка-wayland)
-   [Интернационализация (Weblate)](#интернационализация-weblate)
-   [Участники](#участники)
-   [Благодарности](#благодарности)

<div align="center">

# Использование

</div>

| Перевод выделенного текста                        | Перевод вводимого текста                                                                  | Внешние вызовы                                                                                                          |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Выделите текст и нажмите настроенную горячую клавишу перевода | Нажмите горячую клавишу, чтобы открыть окно перевода, введите текст и нажмите Enter | Более удобная и эффективная работа за счёт вызова из других приложений, подробнее в разделе [Внешние вызовы](#внешние-вызовы) |
| <img src="asset/eg1.gif"/>                        | <img src="asset/eg2.gif"/>                                                                | <img src="asset/eg3.gif"/>                                                                                               |

| Режим слежения за буфером обмена                                                                                             | Скриншот OCR                              | Перевод скриншота                              |
| ---------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- | ---------------------------------------------- |
| Нажмите иконку в левом верхнем углу любой панели перевода, чтобы включить слежение за буфером — скопированный текст будет переведён автоматически | Нажмите горячую клавишу и выделите область для распознавания | Нажмите горячую клавишу и выделите область для перевода |
| <img src="asset/eg4.gif"/>                                                                                                   | <img src="asset/eg5.gif"/>                | <img src="asset/eg6.gif"/>                     |

<div align="center">

# Возможности

</div>

-   [x] Параллельный перевод через несколько сервисов ([Поддерживаемые сервисы](#поддерживаемые-сервисы))
-   [x] Распознавание текста через несколько сервисов ([Поддерживаемые сервисы](#поддерживаемые-сервисы))
-   [x] Синтез речи через несколько сервисов ([Поддерживаемые сервисы](#поддерживаемые-сервисы))
-   [x] Экспорт слов в приложения для изучения языка ([Поддерживаемые сервисы](#поддерживаемые-сервисы))
-   [x] Внешние вызовы ([Подробнее](#внешние-вызовы))
-   [x] Система плагинов ([Плагины](#система-плагинов))
-   [x] Поддержка всех ПК-платформ (Windows, macOS, Linux)
-   [x] Поддержка Wayland (протестировано на KDE, Gnome и Hyprland)
-   [x] Многоязычный интерфейс

<div align="center">

# Поддерживаемые сервисы

</div>

## Перевод

-   [x] [OpenAI](https://platform.openai.com/)
-   [x] [Zhipu AI (ChatGLM)](https://www.zhipuai.cn/)
-   [x] [Gemini Pro](https://gemini.google.com/)
-   [x] [Ollama](https://www.ollama.com/) (офлайн)
-   [x] [Ali Translate](https://www.aliyun.com/product/ai/alimt)
-   [x] [Baidu Translate](https://fanyi.baidu.com/)
-   [x] [Caiyun](https://fanyi.caiyunapp.com/)
-   [x] [Tencent Fanyijun](https://fanyi.qq.com/)
-   [x] [Tencent Transmart](https://transmart.qq.com/)
-   [x] [Volcengine Translate](https://translate.volcengine.com/)
-   [x] [NiuTrans](https://niutrans.com/)
-   [x] [Google Translate](https://translate.google.com)
-   [x] [Bing Translate](https://learn.microsoft.com/zh-cn/azure/cognitive-services/translator/)
-   [x] [Словарь Bing](https://www.bing.com/dict)
-   [x] [DeepL](https://www.deepl.com/)
-   [x] [Youdao](https://ai.youdao.com/)
-   [x] [Кембриджский словарь](https://dictionary.cambridge.org/)
-   [x] [Yandex](https://translate.yandex.com/)
-   [x] [Lingva](https://github.com/TheDavidDelta/lingva-translate) ([плагин](https://github.com/pot-app/pot-app-translate-plugin-template))
-   [x] [Tatoeba](https://tatoeba.org/) ([плагин](https://github.com/pot-app/pot-app-translate-plugin-tatoeba))
-   [x] [ECDICT](https://github.com/skywind3000/ECDICT) ([плагин](https://github.com/pot-app/pot-app-translate-plugin-ecdict))

Больше сервисов — см. [Систему плагинов](#система-плагинов)

## Распознавание текста

-   [x] Системный OCR (офлайн)
    -   [x] [Windows.Media.OCR](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrengine?view=winrt-22621) на Windows
    -   [x] [Apple Vision Framework](https://developer.apple.com/documentation/vision/recognizing_text_in_images) на macOS
    -   [x] [Tesseract OCR](https://github.com/tesseract-ocr) на Linux
-   [x] [Tesseract.js](https://tesseract.projectnaptha.com/) (офлайн)
-   [x] [Baidu](https://ai.baidu.com/tech/ocr/general)
-   [x] [Tencent](https://cloud.tencent.com/product/ocr-catalog)
-   [x] [Volcengine](https://www.volcengine.com/product/OCR)
-   [x] [iFlytek](https://www.xfyun.cn/services/common-ocr)
-   [x] [Перевод изображений Tencent](https://cloud.tencent.com/document/product/551/17232)
-   [x] [Перевод изображений Baidu](https://fanyi-api.baidu.com/product/22)
-   [x] [Simple LaTeX](https://simpletex.cn/)
-   [x] [OCRSpace](https://ocr.space/) ([плагин](https://github.com/pot-app/pot-app-recognize-plugin-template))
-   [x] [Rapid](https://github.com/RapidAI/RapidOcrOnnx) (офлайн, [плагин](https://github.com/pot-app/pot-app-recognize-plugin-rapid))
-   [x] [Paddle](https://github.com/hiroi-sora/PaddleOCR-json) (офлайн, [плагин](https://github.com/pot-app/pot-app-recognize-plugin-paddle))

Больше сервисов — см. [Систему плагинов](#система-плагинов)

## Синтез речи

-   [x] [Lingva](https://github.com/thedaviddelta/lingva-translate)

Больше сервисов — см. [Систему плагинов](#система-плагинов)

## Коллекция слов

-   [x] [Anki](https://apps.ankiweb.net/)
-   [x] [Eudic](https://dict.eudic.net/)
-   [x] [Youdao](https://www.youdao.com/) ([плагин](https://github.com/pot-app/pot-app-collection-plugin-youdao))
-   [x] [ShanBay](https://web.shanbay.com/web/main) ([плагин](https://github.com/pot-app/pot-app-collection-plugin-shanbay))

Больше сервисов — см. [Систему плагинов](#система-плагинов)

<div align="center">

# Система плагинов

</div>

Количество встроенных сервисов ограничено, но функциональность приложения можно расширить с помощью системы плагинов.

## Установка плагинов

Нужный плагин можно найти в [списке плагинов](https://pot-app.com/plugin.html), а затем скачать его из репозитория плагина.

Расширение файлов плагинов pot — `.potext`. После скачивания `.potext`-файла откройте Настройки — Настройки сервисов — Добавить внешний плагин — Установить внешний плагин и выберите соответствующий `.potext`-файл. После установки плагин появится в списке сервисов и будет работать как встроенный.

### Устранение неполадок

-   «Не найден указанный модуль» (Windows)

    Такая ошибка возникает, если в системе отсутствуют библиотеки C++. Установите их [отсюда](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022), и проблема будет решена.

-   «Не является приложением Win32» (Windows)

    Такая ошибка означает, что вы скачали плагин не для своей системы или архитектуры. Скачайте из репозитория плагинов правильный файл, и проблема будет решена.

## Разработка плагинов

В разделе [Шаблоны](https://pot-app.com/plugin.html#%E6%A8%A1%E6%9D%BF) [списка плагинов](https://pot-app.com/plugin.html) представлены шаблоны для разработки плагинов разных типов; документацию по разработке смотрите в соответствующих репозиториях шаблонов.

<div align="center">

# Установка

</div>

## Windows

### Установка через Winget

```powershell
winget install Pylogmon.pot
```

### Установка вручную

1. Скачайте последнюю `exe`-сборку со страницы [Release](https://github.com/pot-app/pot-desktop/releases/latest).

    - для 64-битных систем — `pot_{version}_x64-setup.exe`
    - для 32-битных систем — `pot_{version}_x86-setup.exe`
    - для arm64 — `pot_{version}_arm64-setup.exe`

2. Дважды щёлкните установщик, чтобы установить приложение.

### Устранение неполадок

-   После запуска нет окна, щелчок по иконке в трее ничего не делает

    Проверьте, не удалён или не отключён ли WebView2. Если WebView2 удалён или отключён, установите его вручную или восстановите.

    Если в корпоративной редакции системы установка WebView2 неудобна или невозможна, попробуйте скачать со страницы [Release](https://github.com/pot-app/pot-desktop/releases/latest) версию со встроенным WebView2: `pot_{version}_{arch}_fix_webview2_runtime-setup.exe`

    Если проблема сохраняется, попробуйте запустить приложение в режиме совместимости с Windows 7.

## MacOS

### Установка через Brew

1. Добавьте наш tap:

```bash
brew tap pot-app/homebrew-tap
```

2. Установите pot:

```bash
brew install --cask pot
```

3. Обновление pot:

```bash
brew upgrade --cask pot
```

### Установка вручную

1. Скачайте последнюю `dmg`-сборку со страницы [Release](https://github.com/pot-app/pot-desktop/releases/latest). (Если у вас чип M1, скачайте файл `pot_{version}_aarch64.dmg`, иначе — `pot_{version}_x64.dmg`)
2. Дважды щёлкните скачанный файл и перетащите pot в папку Applications — установка завершена.

### Устранение неполадок

-   «"pot" не может быть открыт, так как не удаётся проверить его разработчика».

    Нажмите «Отменить», затем перейдите в Настройки → Защита и безопасность и нажмите «Открыть всё равно», после чего в появившемся окне нажмите «Открыть». После этого при запуске pot предупреждения появляться больше не будут.

    Если указанных пунктов в разделе «Защита и безопасность» нет или при запуске появляется сообщение о повреждённом файле, откройте Terminal.app, выполните следующую команду и перезапустите pot:

    ```bash
    sudo xattr -d com.apple.quarantine /Applications/pot.app
    ```

-   Если при каждом запуске появляется запрос разрешения из раздела «Универсальный доступ» или не работает перевод выделенного текста, перейдите в Настройки → Защита и безопасность → Универсальный доступ, удалите «pot» и добавьте его заново.

## Linux

### Debian/Ubuntu

1. Скачайте последнюю `deb`-сборку для своей архитектуры со страницы [Release](https://github.com/pot-app/pot-desktop/releases/latest).

2. Установите её с помощью `apt-get`

    ```bash
    sudo apt-get install ./pot_{version}_amd64.deb
    ```

### Arch/Manjaro

> [!WARNING]
> В новых версиях [Webkit2Gtk](https://archlinux.org/packages/extra/x86_64/webkit2gtk) (2.42.0) из-за неполной реализации DMABUF в проприетарном драйвере Nvidia возможен отказ запуска и падения приложения.<br>
> Откатите пакет на предыдущую версию или добавьте переменную окружения `WEBKIT_DISABLE_DMABUF_RENDERER=1` в `/etc/environment` (или туда, где задаются переменные окружения), чтобы отключить использование DMABUF.

1. Смотрите на [AUR](https://aur.archlinux.org/packages?O=0&K=pot-translation)

Установка через AUR helper:

```bash
yay -S pot-translation # или pot-translation-bin

# paru -S pot-translation # или pot-translation-bin
```

2. Если вы используете репозиторий `archlinuxcn`, можно установить напрямую через pacman:

```bash
sudo pacman -S pot-translation
```

### Flatpak

> [!WARNING]
> В версии Flatpak отсутствует иконка в трее.

<a href='https://flathub.org/apps/com.pot_app.pot'>
    <img width='240' alt='Download on Flathub' src='https://flathub.org/api/badge?locale=en'/>
</a>

<div align="center">

# Внешние вызовы

</div>

Pot предоставляет полноценный HTTP-интерфейс, чтобы его могли вызывать другие приложения. Для вызова pot отправляйте HTTP-запросы на `127.0.0.1:port`, где `port` — порт, который слушает pot, по умолчанию `60828`; его можно изменить в настройках приложения.

## Документация API:

```bash
POST "/" => перевести указанный текст (body — переводимый текст),
GET "/config" => открыть настройки,
POST "/translate" => перевести указанный текст (аналогично "/"),
GET "/selection_translate" => перевод выделенного текста,
GET "/input_translate" => перевод вводимого текста,
GET "/ocr_recognize" => распознавание текста со скриншота,
GET "/ocr_translate" => перевод текста со скриншота,
GET "/ocr_recognize?screenshot=false" => распознавание текста (без встроенного скриншота),
GET "/ocr_translate?screenshot=false" => перевод скриншота (без встроенного скриншота),
GET "/ocr_recognize?screenshot=true" => распознавание текста со скриншота,
GET "/ocr_translate?screenshot=true" => перевод текста со скриншота,
```

## Пример:

-   Вызов перевода выделенного текста:

    Чтобы вызвать перевод выделенного текста, достаточно отправить запрос на `127.0.0.1:port`.

    Например, через curl:

    ```bash
    curl "127.0.0.1:60828/selection_translate"
    ```

## Без использования встроенного скриншота

Эта возможность позволяет вызывать OCR/перевод скриншота без встроенного инструмента скриншотов: можно использовать любой любимый инструмент, а заодно решается проблема неработающего встроенного скриншота на некоторых платформах.

### Порядок вызова

1. Сделайте скриншот другим инструментом
2. Сохраните его в `$CACHE/com.pot-app.desktop/pot_screenshot_cut.png`
3. Отправьте запрос на `127.0.0.1:port/ocr_recognize?screenshot=false`

> `$CACHE` — системный каталог кеша, например на Windows: `C:\Users\{имя_пользователя}\AppData\Local\com.pot-app.desktop\pot_screenshot_cut.png`

### Пример

OCR через Flameshot на Linux:

```bash
rm ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && flameshot gui -s -p ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

## Существующие интеграции (быстрый перевод выделенного текста)

### SnipDo (Windows)

1. Скачайте и установите SnipDo из [Microsoft Store](https://apps.microsoft.com/store/detail/snipdo/9NPZ2TVKJVT7).
2. Скачайте расширение SnipDo для pot со страницы [Release](https://github.com/pot-app/pot-desktop/releases/latest) (pot.pbar).
3. Дважды щёлкните скачанный файл расширения, чтобы установить его.
4. Выделите текст — появится панель SnipDo; нажмите кнопку перевода.

### PopClip (MacOS)

1. Скачайте и установите PopClip из [App Store](https://apps.apple.com/us/app/popclip/id445189367?mt=12).
2. Скачайте расширение PopClip для pot со страницы [Release](https://github.com/pot-app/pot-desktop/releases/latest) (pot.popclipextz).
3. Дважды щёлкните скачанный файл расширения, чтобы установить его.
4. Включите расширение pot в настройках PopClip — после этого можно выделять текст и нажимать кнопку перевода.

### Starry (Linux)

> Starry пока находится в стадии разработки, поэтому собрать его можно только вручную.

Github: [ccslykx/Starry](https://github.com/ccslykx/Starry)

<div align="center">

# Поддержка Wayland

</div>

Поскольку разные дистрибутивы поддерживают Wayland по-разному, pot не может обеспечить идеальную работу «из коробки»; ниже приведены решения типичных проблем — после соответствующей настройки pot отлично работает и под Wayland.

## Горячие клавиши не работают

Схема горячих клавиш Tauri не поддерживает Wayland, поэтому настройки горячих клавиш внутри pot не работают под Wayland. Можно назначить системные горячие клавиши, которые отправляют запросы через curl, — подробнее в разделе [Внешние вызовы](#внешние-вызовы).

## Скриншот не работает

В некоторых чистых Wayland-окружениях/оконных менеджерах (например, Hyprland) встроенный скриншот pot не работает. В этом случае можно воспользоваться другим инструментом скриншотов, см. раздел [Без использования встроенного скриншота](#без-использования-встроенного-скриншота).

Пример конфигурации для Hyprland (скриншот через grim и slurp):

```conf
bind = ALT, X, exec, grim -g "$(slurp)" ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
bind = ALT, C, exec, grim -g "$(slurp)" ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_translate?screenshot=false"
```

В других окружениях/оконных менеджерах действия аналогичны.

## Окно перевода следует за курсором

Поскольку pot пока не может получить корректные координаты курсора под Wayland, внутренний механизм не работает. Для некоторых окружений/оконных менеджеров можно настроить правила окон так, чтобы окно следовало за курсором; пример для Hyprland:

```conf
windowrulev2 = float, class:(pot), title:(Translator|OCR|PopClip|Screenshot Translate) # Окно перевода плавающее
windowrulev2 = move cursor 0 0, class:(pot), title:(Translator|PopClip|Screenshot Translate) # Окно перевода следует за курсором
```

<div align="center">

# Интернационализация ([Weblate](https://hosted.weblate.org/engage/pot-app/))

[![](https://hosted.weblate.org/widget/pot-app/pot-desktop/svg-badge.svg)](https://hosted.weblate.org/engage/pot-app/)

[![](https://hosted.weblate.org/widget/pot-app/pot-desktop/multi-auto.svg)](https://hosted.weblate.org/engage/pot-app/)

</div>

<div align="center">

# Участники

</div>

<img src="https://github.com/pot-app/.github/blob/master/pot-desktop-contributions.svg?raw=true" width="100%"/>

## Сборка вручную

### Требования

Node.js >= 18.0.0

pnpm >= 8.5.0

Rust >= 1.80.0

### Сборка

1. Клонируйте репозиторий

    ```bash
    git clone https://github.com/pot-app/pot-desktop.git
    ```

2. Установите зависимости

    ```bash
    cd pot-desktop
    pnpm install
    ```

3. Установите зависимости (только для Linux)

    ```bash
    sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev patchelf libxdo-dev libxcb1 libxrandr2 libdbus-1-3
    ```

4. Отладка в режиме разработки

    ```bash
    pnpm tauri dev # Запустить приложение в режиме разработки
    ```

5. Сборка установочного пакета

    ```bash
    pnpm tauri build # Собрать установочный пакет
    ```

<div align="center">

# Благодарности

</div>

-   [Bob](https://github.com/ripperhe/Bob) — источник вдохновения
-   [bob-plugin-openai-translator](https://github.com/yetone/bob-plugin-openai-translator) — референс OpenAI API
-   [@uiYzzi](https://github.com/uiYzzi) — идея реализации
-   [@Lichenkass](https://github.com/Lichenkass) — поддержка pot в магазине приложений Deepin
-   [Tauri](https://github.com/tauri-apps/tauri) — удобный GUI-фреймворк

<div align="center">
