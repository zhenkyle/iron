//! examples/task.rs
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// extern crate stm32f3;
// extern crate stm32f30x;
//use cortex_m_semihosting::{debug, hprintln};
// use stm32f30x_hal::stm32f30x as stm32;
use stm32f3xx_hal::stm32;
//use stm32f30x as stm32;
use cortex_m_semihosting::{hprintln};
use panic_semihosting as _;

// #[rtfm::app(device = stm32f3::stm32f303)]
// #[rtfm::app(device = stm32)]
// #[rtfm::app(device = stm32f30x)]
#[rtfm::app(device = stm32)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        c.spawn.foo().unwrap();
    }

    #[task(spawn = [bar, baz])]
    fn foo(c: foo::Context) {
        hprintln!("foo - start").unwrap();

        // spawns `bar` onto the task scheduler
        // `foo` and `bar` have the same priority so `bar` will not run until
        // after `foo` terminates
        c.spawn.bar().unwrap();

        hprintln!("foo - middle").unwrap();

        // spawns `baz` onto the task scheduler
        // `baz` has higher priority than `foo` so it immediately preempts `foo`
        c.spawn.baz().unwrap();

        hprintln!("foo - end").unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();

//        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(priority = 2)]
    fn baz(_: baz::Context) {
        hprintln!("baz").unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn TIM3();
        fn TIM4();
    }
};
