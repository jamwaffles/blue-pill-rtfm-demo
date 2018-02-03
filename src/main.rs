#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate cortex_m_semihosting as sh;
extern crate stm32f103xx_hal as hal;
extern crate embedded_hal;
extern crate aligned;

use aligned::Aligned;
use hal::prelude::*;
use hal::time::{Hertz};
use cortex_m_rtfm_macros::app;
use rtfm::{ Threshold};

use core::fmt::Write;

use sh::hio;
use sh::hio::{ HStdout };

const _0: u8 = 3;
const _1: u8 = 7;
// const LATCH_DELAY: Microseconds = Microseconds(50);
const WS2812B_FREQUENCY: Hertz = Hertz(800_000);

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

        // Num LEDs * 3
        // static RGB_ARRAY: Aligned<u32, [u8; 3]> = Aligned([0; 3]);
        // Num LEDs * 3 * 24? One byte for each bit?
        // static WS2812B_BUFFER: Buffer<[u8; 72], Dma1Channel2> = Buffer::new([0; 72]);

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
    writeln!(hstdout, "Init start...").unwrap();

    let timer1 = p.device.TIM1;

    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);

    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let c4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);

    writeln!(hstdout, "Init").unwrap();

    let mut pwm = p.device.TIM2.pwm(
        (c1, c2, c3, c4),
        &mut afio.mapr,
        WS2812B_FREQUENCY,
        clocks,
        &mut rcc.apb1,
    )
    .3;
    pwm.enable();

    writeln!(hstdout, "Init success").unwrap();

    init::LateResources {
        DBG: hstdout,
    }
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    // late resources can be used at this point
    let hstdout: &'static mut HStdout = r.DBG;

    loop {
        writeln!(hstdout, "Idle").unwrap();
    }
}