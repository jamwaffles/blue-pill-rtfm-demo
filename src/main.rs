#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate cortex_m_semihosting as sh;
extern crate stm32f103xx_hal as blue_pill;
extern crate embedded_hal as hal;

use cortex_m::asm;
use blue_pill::prelude::*;
use blue_pill::time::{Hertz};
use cortex_m_rtfm_macros::app;
use rtfm::{ Threshold};
use blue_pill::spi::{ Spi };
use hal::spi::{ Mode, Phase, Polarity };
use blue_pill::gpio::{ Input, Output, PushPull, Floating, Alternate };
use hal::digital::OutputPin;
use blue_pill::gpio::gpioa::{ PA5, PA6, PA7 };
use blue_pill::gpio::gpiob::{ PB0, PB1 };
use blue_pill::stm32f103xx::SPI1;
use blue_pill::delay::Delay;

use core::fmt::Write;

use sh::hio;
use sh::hio::{ HStdout };

struct SSD1306<SPI, RST, DC>
{
    spi: SPI,
    rst: RST,
    dc: DC,
    buffer: [u8; 1024],
}

impl<SPI, RST, DC> SSD1306<SPI, RST, DC> where
    SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
    RST: OutputPin,
    DC: OutputPin
    {
    pub fn new(spi: SPI, rst: RST, dc: DC) -> Self {
        SSD1306 {
            spi,
            rst,
            dc,
            buffer: [0b10101010; 1024],
        }
    }

    pub fn reset(&mut self) {
        self.rst.set_low();
        self.rst.set_high();
    }

    pub fn cmd(&mut self, cmd: u8) {
       self.dc.set_low();

       self.spi.write(&[ cmd ]);

       self.dc.set_high();
    }

    pub fn init(&mut self) {
        let init_commands: [ u8; 25 ] = [
            0xAe, // 0 disp off
            0xD5, // 1 clk div
            0x80, // 2 suggested ratio
            0xA8, 63, // 3 set multiplex, height-1
            0xD3, 0x0, // 5 display offset
            0x40, // 7 start line
            0x8D, 0x14, // 8 charge pump
            0x20, 0x0, // 10 memory mode
            0xA1, // 12 seg remap 1
            0xC8, // 13 comscandec
            0xDA, 0x12, // 14 set compins, height==64 ? 0x12:0x02,
            0x81, 0xCF, // 16 set contrast
            0xD9, 0xF1, // 18 set precharge
            0xDb, 0x40, // 20 set vcom detect
            0xA4, // 22 display all on
            0xA6, // 23 display normal (non-inverted)
            0xAf // 24 disp on
        ];

        for cmd in init_commands.iter() {
            self.cmd(*cmd);
        }
    }

    pub fn flush(&mut self) {
        let flush_commands: [ u8; 6 ] = [
             0x21, // columns
             0, 127,
             0x22, // pages
             0, 7 /* (height>>3)-1 */];

        for cmd in flush_commands.iter() {
            self.cmd(*cmd);
        }

        // Not a command
        self.dc.set_high();

        self.spi.write(&self.buffer);
    }
}

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
    },

    idle: {
        resources: [
            DISP,
            DBG,
        ],
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Init start...").unwrap();

    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let mut spi = Spi::spi1(
        p.device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        // https://github.com/adafruit/Adafruit_SSD1306/blob/master/Adafruit_SSD1306.cpp#L197
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut disp = SSD1306::new(spi, rst, dc);

    disp.reset();
    disp.init();

    disp.flush();

    // disp.cmd(0xA7);     // Invert

    writeln!(hstdout, "Init success").unwrap();

    init::LateResources {
        DISP: disp,
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