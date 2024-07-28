#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f103::Peripherals;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::Peripherals as CortexPeripherals;

#[entry]
fn main() -> ! {
    // Получаем доступ к периферийным устройствам
    let dp = Peripherals::take().unwrap();
    let cp = CortexPeripherals::take().unwrap();

    // Включаем тактирование GPIOC
    let rcc = dp.rcc;
    rcc.apb2enr().modify(|_, w| w.iopcen().set_bit());

    // Конфигурируем PC13 как выход
    let gpioc = dp.gpioc;
    unsafe {gpioc.crh().modify(|_, w| w.mode13().bits( 0b01 )); }

    // Получаем доступ к системному таймеру
    let mut syst = cp.SYST;

    // Устанавливаем источник тактирования для системного таймера
    syst.set_clock_source(SystClkSource::Core);

    // Задаем период таймера (1 секунда)
    syst.set_reload(8_000_000); // 8 MHz для простоты, настраиваем на 1 секунду
    syst.enable_counter();
    syst.enable_interrupt();

    loop {
        // Ждем срабатывания таймера
        while !syst.has_wrapped() {}

        // Переключаем состояние светодиода
        gpioc.odr().modify(|r, w| w.odr13().bit(!r.odr13().bit()));
    }
}