# STM32F103C8T6 Blue Pill [RTFM]() demo

This demo will flash the onboard LED (pin PC13) on a Blue Pill STM32 breakout board.

The board is programmed over the serial port on pins A9 and A10 using an FT232 USB to serial converter. There's no debugger, but if you don't have access to an ST-Link or other OpenOCD-compatible hardware, this is a good way to get started.

Tested in Windows and Windows Subsystem for Linux, highly likely to work in actual Linux, probably works on macOS with some changes.

## Prerequisites

Taken largely from [the cortex-m-quickstart docs](https://docs.rs/cortex-m-quickstart/0.2.4/cortex_m_quickstart/):

- `rustup default nightly`
- Install the ARM development toolchain
	- Linux (Ubuntu/WSL): `sudo apt-get install -y binutils-arm-none-eabi gdb-arm-none-eabi`
	- Windows: download and install the tooclhain from [here](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads).
- `rustup component add rust-src`
- `cargo install xargo`
- Install [stm32flash](https://sourceforge.net/projects/stm32flash/)
	- Linux (Ubuntu/WSL): `sudo apt-get install stm32flash`
	- Windows:
		- Download [the archive](https://sourceforge.net/projects/stm32flash/), extract it and place `stm32flash.exe` somewhere convenient.
		- Or just use the bundled binary in `./stm32flash-0.5-win64`

## Hardware setup

- Switch your FT232 board to 3v3 if it supports this option
- Ensure `BOOT0` is jumpered to `1`
- Connect pin `A9` to `RX` on your FT232
- Connect pin `A10` to `TX` on your FT232
- Connect power from the FT232 board to the 4 pin debug header on your Blue Pill

## Building and running

TL;DR:

```bash
# Build the project
xargo build --release --target thumbv7em-none-eabihf

# Convert output to Intel hex format for stm32flash to read properly
arm-none-eabi-objcopy -O ihex ./target/thumbv7em-none-eabihf/release/blue-pill-rtfm-demo ./target/thumbv7em-none-eabihf/release/blue-pill-rtfm-demo.ihex

# Press reset on your Blue Pill before running this
stm32flash -b 115200 -w ./target/thumbv7em-none-eabihf/release/blue-pill-rtfm-demo.ihex -g 0x8000000 /dev/ttyYourDeviceHere

# OR on Windows
./stm32flash-0.5-win64/stm32flash.exe -b 115200 -w ./target/thumbv7em-none-eabihf/release/blue-pill-rtfm-demo.ihex -g 0x8000000 COM7
```

If all goes well, the last step should show this output:

```
stm32flash 0.5

http://stm32flash.sourceforge.net/

Using Parser : Intel HEX
Interface serial_w32: 115200 8E1
Version      : 0x22
Option 1     : 0x00
Option 2     : 0x00
Device ID    : 0x0410 (STM32F10xxx Medium-density)
- RAM        : 20KiB  (512b reserved by bootloader)
- Flash      : 128KiB (size first sector: 4x1024)
- Option RAM : 16b
- System RAM : 2KiB
Write to memory
Erasing memory
Wrote address 0x08000374 (100.00%) Done.

Starting execution at address 0x08000000... done.
```

and the LED attached to PC13 should be blinking at 1Hz.

To program the board again, press the reset button before running `stm32flash`. Wash rinse repeat.
