xargo build
if %errorlevel% neq 0 exit /b %errorlevel%
arm-none-eabi-gdb.exe -iex "set auto-load safe-path /" .\target\thumbv7em-none-eabihf\debug\blue-pill-rtfm-demo