#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate cortex_m_semihosting as sh;
// extern crate mpu9250;
extern crate stm32f103xx_hal as hal;
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;

use cortex_m::asm;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::spi::{ Mode, Phase, Polarity };
use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::{ Spi };
use hal::stm32f103xx;
use hal::timer::Timer;
use cortex_m_rtfm_macros::app;
use rtfm::{/*app, */Resource, Threshold};
// use mpu9250::Mpu9250;

use core::fmt::Write;

use sh::hio;
use sh::hio::{ HStdout };

// TASKS AND RESOURCES
app! {
    device: hal::stm32f103xx,

    resources: {
        // static BUSY: bool = false;
        // static CONTEXT_SWITCHES: u16 = 0;
        // static FRAMES: u8 = 0;
        // static RGB_ARRAY: Aligned<u32, [u8; 72]> = Aligned([0; 72]);
        // static RX_BUFFER: Buffer<[u8; 72], Dma1Channel5> = Buffer::new([0; 72]);
        // static SLEEP_CYCLES: u32 = 0;
        // static TX_BUFFER: Buffer<[u8; 13], Dma1Channel4> = Buffer::new([0; 13]);
        // static WS2812B_BUFFER: Buffer<[u8; 577], Dma1Channel2> =
        //     Buffer::new([0; 577]);

        static DBG: HStdout;
    },

    idle: {
        resources: [
            DBG,
        ],
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Init").unwrap();

    init::LateResources {
        DBG: hstdout,
    }
}

fn idle(t: &mut Threshold, r: idle::Resources) -> ! {
    // late resources can be used at this point
    let mut hstdout: &'static mut HStdout = r.DBG;

    loop {
        writeln!(hstdout, "Idle").unwrap();
    }
}