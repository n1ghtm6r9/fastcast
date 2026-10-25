# fastcast

Трансляция экрана ПК на другие устройства с минимальной задержкой. Первая цель — Google TV (4K), дальше 4K 120 fps. Клиенты только нативные, без браузера.

## Структура

```
apps/
  server/    Rust, пакет app, бинарь fastcast: захват экрана, кодирование и раздача по WebSocket
  client/    нативное приложение для Android TV / Google TV на Kotlin (MediaCodec → SurfaceView)
packages/
  media/     Rust: FLV-демуксер, разбор SPS (размер кадра), строка кодека avc1.*
```

Модули сервера устроены как в `price-percent-trading`: `src/<модуль>/{constants,services,types,test}`, на каждую сущность свой файл.

### Как идёт кадр

1. `ffmpeg` захватывает экран фильтром `ddagrab` (Desktop Duplication, кадр остаётся на GPU) и кодирует аппаратным H.264. Режим `auto` пробует кодировщики по порядку: NVENC → AMF → QSV → x264. Настройки задержки: `-tune ull`, без B-кадров, CBR.
2. Сервер читает FLV из stdout ffmpeg и рассылает кадры через `hub`: `broadcast` плюс кэш GOP от последнего ключевого кадра. Новый зритель сразу получает config и ключевой кадр, не дожидаясь следующего.
3. Захват стартует при подключении первого зрителя и останавливается через 5 с после ухода последнего.
4. Клиент отдаёт кадры аппаратному MediaCodec, тот декодирует прямо в `SurfaceView`, в режиме низкой задержки. Рисуется только самый свежий кадр. Если клиент отстаёт, он пропускает кадры до следующего ключевого. Если от сервера 5 с ничего не приходит (например, ПК пропал из сети), клиент сам переподключается.

### Протокол (`ws://<host>:3278/ws`, только сервер → клиент)

- Текстовое сообщение с конфигом:
  `{"type":"config","codec":"avc1.640033","width":2560,"height":1440,"fps":60,"description":"<base64 avcC>"}`.
  Приходит первым и повторяется перед первым ключевым кадром новой конфигурации кодировщика.
- Бинарный кадр: `[flags u8: бит 0 = ключевой][timestamp_ms u32 LE][access unit H.264 в формате AVCC]`.

### Автопоиск (UDP 3278)

Клиент раз в секунду рассылает `{"type":"discover","version":1}` на 255.255.255.255 и на broadcast-адрес каждой своей сети. Сервер отвечает ему напрямую: `{"type":"server","version":1,"name":"KOMPUTER","port":3278}`. Адрес сервера клиент берёт из отправителя ответа, вводить ничего не нужно.

## Сервер (Windows)

Нужны Rust и ffmpeg версии 6 или новее: `winget install Gyan.FFmpeg`.

```powershell
copy .env.example .env   # по желанию
cargo server             # = cargo run -p app --release
```

Готовая сборка лежит в каждом релизе: `fastcast-server-windows-x64.zip`.

Клиенты в той же сети находят сервер сами. При первом запуске Windows Firewall спросит разрешение: разреши доступ для частной сети, иначе ТВ не увидит ПК. Нужны TCP 3278 для видео и UDP 3278 для автопоиска.

Настройки в `.env`:

| Переменная | По умолчанию | Что задаёт |
|---|---|---|
| `WEB_HOST` / `WEB_PORT` | `0.0.0.0` / `3278` | адрес и порт сервера |
| `DISCOVERY_PORT` | `3278` | UDP-порт автопоиска |
| `SERVER_NAME` | имя компьютера | имя сервера в списке на ТВ |
| `FFMPEG_PATH` | `ffmpeg` | путь к ffmpeg |
| `VIDEO_ENCODER` | `auto` | `auto`, `nvenc`, `amf`, `qsv`, `x264` |
| `CAPTURE_OUTPUT` | `0` | номер монитора |
| `CAPTURE_FPS` | `60` | частота кадров; 120 включает 120 fps |
| `VIDEO_BITRATE_KBPS` | `20000` | битрейт |
| `KEYFRAME_INTERVAL_SECONDS` | `2` | интервал ключевых кадров |

## Клиент (Google TV / Android TV)

Нужны JDK 17 и Android SDK: переменные `JAVA_HOME` и `ANDROID_HOME`.

```powershell
cd apps/client
.\gradlew.bat assembleRelease
```

Все сборки подписываются одним ключом: `%USERPROFILE%\.fastcast\fastcast-release.jks`, параметры в `keystore.properties` рядом с ним. Путь к `keystore.properties` можно задать переменной `FASTCAST_SIGNING`. Ключ хранится вне репозитория, держи его копию: без него обновить приложение на ТВ можно только через удаление.

Установка на телевизор, один раз:

1. На телевизоре открой Настройки → Система → Об устройстве и 7 раз нажми «Сборка ОС Android TV». Затем в «Для разработчиков» включи отладку по Wi‑Fi и подключи adb: `adb pair`, затем `adb connect`.
2. Выполни:
   ```powershell
   adb install app\build\outputs\apk\release\app-release.apk
   adb shell appops set dev.fastcast.client REQUEST_INSTALL_PACKAGES allow
   adb shell appops set dev.fastcast.client SYSTEM_ALERT_WINDOW allow
   ```
3. Открой fastcast из лаунчера. Приложение само найдёт ПК и подключится. Если серверов несколько, выбери нужный: в следующий раз приложение подключится к нему само.

Управление на ТВ: OK или Info показывает статистику (fps, Мбит/с, разрешение, декодер), «Назад» возвращает к списку серверов.

### Обновления

Дальше приложение обновляется само из GitHub Releases. Оно проверяет `releases/latest/download/fastcast-tv.json` при каждом открытии, во время трансляции проверок нет. Если вышла версия новее, приложение скачивает APK, сверяет размер и SHA-256, ставит его без диалога и запускается заново. Разрешения из шага 2 нужны, чтобы установка шла без подтверждения и приложение перезапускалось само.

## Релизы

Каждый пуш в `master` запускает `.github/workflows/release.yml`:

1. Клиент: тесты, lint и подписанная сборка, `versionCode` = 100 + номер запуска, `versionName` = `0.1.<versionCode>`.
2. Сервер на Windows: тесты и `cargo build --release`.
3. Релиз `v0.1.<versionCode>` с файлами `fastcast-tv.apk`, `fastcast-tv.json` (`versionCode`, `versionName`, `url`, `sha256`, `size`) и `fastcast-server-windows-x64.zip`.

Секреты репозитория: `ANDROID_KEYSTORE_BASE64` (ключ в base64) и `ANDROID_KEYSTORE_PASSWORD`.

## Тесты

```powershell
cargo t                                   # Rust: nextest по всему workspace
cd apps/client; .\gradlew.bat testDebugUnitTest lintDebug
```

## Дальше

- HEVC и AV1 для 4K 120 fps (NVENC на RTX 5070 умеет оба), выбор кодека по возможностям декодера ТВ.
- Транспорт поверх UDP или QUIC с FEC вместо TCP/WebSocket, свой захват (DXGI/WGC) и NVENC без ffmpeg.
- Ограничить очередь зрителя на сервере по времени, а не по числу кадров (сейчас при плохом Wi‑Fi может накопиться до ~4 с задержки).
- Звук: WASAPI loopback → Opus.
- Трей-приложение на ПК, нативные десктоп-клиенты.
