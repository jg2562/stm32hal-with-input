#![no_main]
#![no_std]

mod logger;
use crate::logger::init as logger_init;
use cortex_m_rt::entry;
use log::*;
use stm32h7xx_hal::{delay::Delay, pac, prelude::*, rcc::rec::AdcClkSel};

#[entry]
fn main() -> ! {
    logger_init();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let mut ccdr = rcc.sys_ck(400_u32.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::PER);

    let mut delay = Delay::new(cp.SYST, ccdr.clocks);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    let mut pin = gpioa.pa0.into_push_pull_output();

    delay.delay_ms(1_u32);
    pin.set_high();
    delay.delay_ms(1_u32);
    pin.set_low();
    delay.delay_ms(1_u32);
    info!("Pre");
    info!("MODER: {:b}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).moder.read().bits()
    });
    info!("OTYPER: {:X}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).otyper.read().bits()
    });
    info!("AFRL: {:X}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).afrl.read().bits()
    });
    info!("AFRH: {:X}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).afrh.read().bits()
    });
    let is_high = pin.with_input(|x| x.is_high());
    info!("Post");
    info!("MODER: {:b}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).moder.read().bits()
    });
    info!("OTYPER: {:X}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).otyper.read().bits()
    });
    info!("AFRL: {:X}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).afrl.read().bits()
    });
    info!("AFRH: {:X}", unsafe {
        (*stm32h7xx_hal::pac::GPIOA::ptr()).afrh.read().bits()
    });
    delay.delay_ms(1_u32);
    pin.set_high();
    delay.delay_ms(1_u32);
    pin.set_low();
    delay.delay_ms(1_u32);

    loop {}
}
