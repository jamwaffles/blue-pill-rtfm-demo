xargo build --release --target thumbv7em-none-eabihf
arm-none-eabi-objcopy -O ihex .\target\thumbv7em-none-eabihf\release\blue-pill-rtfm-demo .\target\thumbv7em-none-eabihf\release\blue-pill-rtfm-demo.ihex
.\stm32flash-0.5-win64\stm32flash.exe -b 115200 -w .\target\thumbv7em-none-eabihf\release\blue-pill-rtfm-demo.ihex -g 0x8000000 COM7