#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f103::Peripherals;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    // Настройка тактирования
    let rcc = &dp.rcc;
    rcc.apb2enr().modify(|_, w| w.iopaen().set_bit()); // Включаем тактирование GPIOA
    rcc.apb1enr().modify(|_, w| w.tim2en().set_bit()); // Включаем тактирование TIM2

    // Настройка GPIOA
    let gpioa = &dp.gpioa;
    gpioa.crl().modify(|_, w| unsafe {
        w.mode0().bits(0b10) // Output mode, max speed 2 MHz
         .cnf0().bits(0b10) // Alternate function output Push-pull
    });

    // Настройка TIM2 для генерации PWM
    let tim2 = &dp.tim2;

    // Сброс TIM2
    rcc.apb1rstr().modify(|_, w| w.tim2rst().set_bit());
    rcc.apb1rstr().modify(|_, w| w.tim2rst().clear_bit());

    // Настройка таймера
    tim2.psc().write(|w| unsafe { w.psc().bits(72 - 1) }); // Предделитель
    tim2.arr().write(|w| unsafe { w.arr().bits(1000 - 1) }); // Автоперезагрузка

    // Настройка канала 1 для PWM
    tim2.ccmr1_output().modify(|_, w| unsafe {
        w.oc1m().bits(0b110) // PWM mode 1
         .oc1pe().set_bit()
    });
    tim2.ccer().modify(|_, w| w.cc1e().set_bit()); // Включение канала 1

    // Устанавливаем duty cycle на 50%
    tim2.ccr1().write(|w| unsafe { w.bits(500) });

    // Включение таймера
    tim2.cr1().modify(|_, w| w.cen().set_bit());

    loop {
        // Основной цикл
    }
}