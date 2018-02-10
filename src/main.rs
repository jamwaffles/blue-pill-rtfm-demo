#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]
#![feature(slice_patterns)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate cortex_m_semihosting as sh;
extern crate stm32f103xx_hal as blue_pill;
extern crate embedded_hal as hal;

extern crate ssd1306;
extern crate embedded_graphics;

// use cortex_m::asm;
use blue_pill::prelude::*;
use cortex_m_rtfm_macros::app;
use rtfm::{ Threshold};
use blue_pill::spi::{ Spi };
use hal::spi::{ Mode, Phase, Polarity };
use blue_pill::gpio::{ Input, Output, PushPull, Floating, Alternate };
use blue_pill::gpio::gpioa::{ PA5, PA6, PA7 };
use blue_pill::gpio::gpiob::{ PB0, PB1 };
use blue_pill::stm32f103xx::SPI1;
use core::fmt::Write;
use sh::hio;
use sh::hio::{ HStdout };

use ssd1306::{ SSD1306, Drawing };
use embedded_graphics::image::{ Image1BPP };

pub type OledDisplay = SSD1306<
    Spi<
        SPI1,
        (
            PA5<Alternate<PushPull>>,
            PA6<Input<Floating>>,
            PA7<Alternate<PushPull>>,
        ),
    >,
    PB0<Output<PushPull>>,  // B0 -> RST
    PB1<Output<PushPull>>,  // B1 -> DC
>;

// TASKS AND RESOURCES
app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static DISP: OledDisplay;
        static DBG: HStdout;
        static COUNTER: u32 = 0;
    },

    idle: {
        resources: [
            DISP,
            DBG,
            COUNTER,
        ],
    },
}

fn init(p: init::Peripherals, _r: init::Resources) -> init::LateResources {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Init start...").unwrap();

    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut disp = SSD1306::new(spi, rst, dc);

    disp.reset();

    disp.init();

    let image = Image1BPP {
        width: 62,
        height: 48,
        imagedata: include_bytes!("../samuel_1bpp.raw")
    };

    disp.draw_image_1bpp(&image, (128 / 2) - (image.width / 2), 16);

    disp.draw_text_1bpp("#((", 0, 0);

    disp.flush();

    writeln!(hstdout, "Init success").unwrap();

    init::LateResources {
        DISP: disp,
        DBG: hstdout,
    }
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    let hstdout: &'static mut HStdout = r.DBG;
    let count: &'static mut u32 = r.COUNTER;
    let disp: &'static mut OledDisplay = r.DISP;

    loop {
        writeln!(hstdout, "Idle").unwrap();

        *count += 1;

        // disp.set_index(*count);

        // disp.flush();
    }
}
