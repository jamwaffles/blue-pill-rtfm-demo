#!/bin/sh

openocd -s ./tools/openocd-0.10.0/scripts -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
