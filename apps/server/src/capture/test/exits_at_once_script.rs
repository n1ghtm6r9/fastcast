pub const EXITS_AT_ONCE_SCRIPT: &str =
    "@echo off\r\necho %* | find \"pipe:1\" >nul && exit /b 1\r\nexit /b 0\r\n";
