//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{hprintln};
use stm32f3::stm32f303 as stm32;
use panic_semihosting as _;

#[rtfm::app(device = stm32f3::stm32f303, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        static mut X: u32 = 0;

        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let _device: stm32::Peripherals = cx.device;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("init").unwrap();

        loop {}
    }
};
