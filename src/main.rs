//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]


use cortex_m_rt::{exception, ExceptionFrame};
use cortex_m_semihosting::{hprintln};
use panic_semihosting as _;
use embedded_graphics::primitives::{Circle, Line, Rectangle as Rect};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::style::PrimitiveStyleBuilder;
use stm32f3xx_hal as hal;
use hal::prelude::*;
use hal::delay::Delay;
use hal::spi::{Mode, Phase, Polarity, Spi};
use ssd1306::prelude::*;
use ssd1306::Builder;

#[rtfm::app(device = stm32f3xx_hal::stm32, peripherals = true)]
const APP: () = {
    #[init(spawn =[draw_things])]
    fn init(cx: init::Context) {
        static mut X: u32 = 0;

        // Cortex-M peripherals
        let cp: cortex_m::Peripherals = cx.core;
        // Device specific peripherals
        let dp: hal::stm32::Peripherals = cx.device;

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    
        // SPI
        let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
        let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
        let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

        let spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            8.mhz(),
            clocks,
            &mut rcc.apb2,
        );

        // rst and dc PIN
        let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

        let mut delay = Delay::new(cp.SYST, clocks);        
        disp.reset(&mut rst, &mut delay).unwrap();
        
        disp.init().unwrap();
        disp.flush().unwrap();

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();

        Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
            .into_styled(style)
            .into_iter().draw(&mut disp);

        Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
            .into_styled(style)
            .into_iter().draw(&mut disp);

        Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
            .into_styled(style)
            .into_iter().draw(&mut disp);

        Rect::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
            .into_styled(style)
            .into_iter().draw(&mut disp);


        Circle::new(Point::new(96, 16 + 8), 8)
            .into_styled(style)
            .into_iter().draw(&mut disp);

        disp.flush().unwrap();

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;
        cx.spawn.draw_things().unwrap();
        hprintln!("init").unwrap();

    }

    // Optional.
    //
    // https://rtfm.rs/0.5/book/en/by-example/app.html#idle
    // > When no idle function is declared, the runtime sets the SLEEPONEXIT bit and then
    // > sends the microcontroller to sleep after running init.
//    #[idle]
//    fn idle(_cx: idle::Context) -> ! {
//        loop {
//            cortex_m::asm::wfi();
//        }
//    }

    #[task(priority = 2)]
    fn draw_things(_cx: draw_things::Context) {
        hprintln!("draw_things!").unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn TIM4();
    }
};

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
