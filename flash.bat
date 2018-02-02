xargo build --release
arm-none-eabi-gdb.exe -iex "set auto-load safe-path /" .\target\thumbv7em-none-eabihf\release\blue-pill-rtfm-demo