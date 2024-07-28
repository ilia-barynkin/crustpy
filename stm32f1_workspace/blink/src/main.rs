#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1_api::stm32f103;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::Peripherals;

#[entry]
fn main() -> ! {
    // Получаем доступ к периферийным устройствам
    let dp = stm32f103::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Включаем тактирование GPIOC
    let rcc = &dp.RCC;
    rcc.apb2enr.modify(|_, w| w.iopcen().set_bit());

    // Конфигурируем PC13 как выход
    let gpioc = &dp.GPIOC;
    gpioc.crh.modify(|_, w| w.mode13().output().cnf13().push_pull());

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
        gpioc.odr.modify(|r, w| w.odr13().bit(!r.odr13().bit()));
    }
}