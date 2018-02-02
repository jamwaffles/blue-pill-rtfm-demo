#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;
// extern crate mpu9250;
extern crate stm32f103xx_hal as hal;
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;

use cortex_m::asm;
use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::{ Spi };
use embedded_hal::spi::{ Mode, Phase, Polarity };
use hal::stm32f103xx;
use embedded_hal::blocking::spi::Transfer;
use hal::timer::Timer;
// use mpu9250::Mpu9250;

use core::fmt::Write;

use sh::hio;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let mut spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        6_400_000.hz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    const ON_BYTE: u8 = 0b1111_1100;
    const OFF_BYTE: u8 = 0b1100_0000;

    // loop {
        spi.transfer(&mut [
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
            OFF_BYTE,
            ON_BYTE,
        ]);

    //     block!(timer.wait()).unwrap();
    // }

    // loop {
    //     // spi.send(0b1111_0000);
    //     spi.transfer(&mut [ 0b1111_0000 ]);
    // }

    // let mut delay = Delay::new(cp.SYST, clocks);

    // let mut mpu9250 = Mpu9250::new(spi, nss, &mut delay).unwrap();

    // // sanity checks
    // assert_eq!(mpu9250.who_am_i().unwrap(), 0x71);
    // assert_eq!(mpu9250.ak8963_who_am_i().unwrap(), 0x48);

    // let _a = mpu9250.all().unwrap();

    asm::bkpt();
}
