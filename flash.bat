xargo build --release
if %errorlevel% neq 0 exit /b %errorlevel%
arm-none-eabi-gdb.exe -iex "set auto-load safe-path /" .\target\thumbv7em-none-eabihf\release\blue-pill-rtfm-demo