#![no_std]
#![no_main]
// Halt when the program panics.
extern crate panic_halt;
// Includes.
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f103;

#[entry]
fn main() -> ! {
    let p = stm32f103::Peripherals::take().unwrap();
//   let rcc = p.RCC;
//   rcc.iopenr.write( |w| w.iopben().set_bit() );
//   let gpioc = p.GPIOC;
//   unsafe { gpioc.moder.write( |w| w.mode13().bits( 0b01 ) ); }
//   gpioc.otyper.write( |w| w.ot13().clear_bit() );
//   // Restart the SysTick counter.
//   syst.clear_current();
//   // Main loop.
//   loop {
//     // Toggle the LED every SysTick tick.
//     while !syst.has_wrapped() {};
//     gpioc.odr.write( |w| w.od13().set_bit() );
//     while !syst.has_wrapped() {};
//     gpioc.odr.write( |w| w.od13().clear_bit() );
//   }

    loop {
    }
}