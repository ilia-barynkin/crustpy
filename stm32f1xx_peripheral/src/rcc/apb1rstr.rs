#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<Apb1rstrSpec>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<Apb1rstrSpec>;
#[doc = "Field `TIM2RST` reader - Timer 2 reset"]
pub type Tim2rstR = crate::BitReader;
#[doc = "Field `TIM2RST` writer - Timer 2 reset"]
pub type Tim2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3RST` reader - Timer 3 reset"]
pub type Tim3rstR = crate::BitReader;
#[doc = "Field `TIM3RST` writer - Timer 3 reset"]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4RST` reader - Timer 4 reset"]
pub type Tim4rstR = crate::BitReader;
#[doc = "Field `TIM4RST` writer - Timer 4 reset"]
pub type Tim4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5RST` reader - Timer 5 reset"]
pub type Tim5rstR = crate::BitReader;
#[doc = "Field `TIM5RST` writer - Timer 5 reset"]
pub type Tim5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6RST` reader - Timer 6 reset"]
pub type Tim6rstR = crate::BitReader;
#[doc = "Field `TIM6RST` writer - Timer 6 reset"]
pub type Tim6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7RST` reader - Timer 7 reset"]
pub type Tim7rstR = crate::BitReader;
#[doc = "Field `TIM7RST` writer - Timer 7 reset"]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12RST` reader - Timer 12 reset"]
pub type Tim12rstR = crate::BitReader;
#[doc = "Field `TIM12RST` writer - Timer 12 reset"]
pub type Tim12rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13RST` reader - Timer 13 reset"]
pub type Tim13rstR = crate::BitReader;
#[doc = "Field `TIM13RST` writer - Timer 13 reset"]
pub type Tim13rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14RST` reader - Timer 14 reset"]
pub type Tim14rstR = crate::BitReader;
#[doc = "Field `TIM14RST` writer - Timer 14 reset"]
pub type Tim14rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub type WwdgrstR = crate::BitReader;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub type WwdgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type Spi2rstR = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub type Spi3rstR = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3RST` reader - USART 3 reset"]
pub type Usart3rstR = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART 3 reset"]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4RST` reader - UART 4 reset"]
pub type Uart4rstR = crate::BitReader;
#[doc = "Field `UART4RST` writer - UART 4 reset"]
pub type Uart4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RST` reader - UART 5 reset"]
pub type Uart5rstR = crate::BitReader;
#[doc = "Field `UART5RST` writer - UART 5 reset"]
pub type Uart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANRST` reader - CAN reset"]
pub type CanrstR = crate::BitReader;
#[doc = "Field `CANRST` writer - CAN reset"]
pub type CanrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRST` reader - Backup interface reset"]
pub type BkprstR = crate::BitReader;
#[doc = "Field `BKPRST` writer - Backup interface reset"]
pub type BkprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PwrrstR = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PwrrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACRST` reader - DAC interface reset"]
pub type DacrstR = crate::BitReader;
#[doc = "Field `DACRST` writer - DAC interface reset"]
pub type DacrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> Tim2rstR {
        Tim2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> Tim4rstR {
        Tim4rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> Tim5rstR {
        Tim5rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> Tim6rstR {
        Tim6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 12 reset"]
    #[inline(always)]
    pub fn tim12rst(&self) -> Tim12rstR {
        Tim12rstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 13 reset"]
    #[inline(always)]
    pub fn tim13rst(&self) -> Tim13rstR {
        Tim13rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> Tim14rstR {
        Tim14rstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WwdgrstR {
        WwdgrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> Uart4rstR {
        Uart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> Uart5rstR {
        Uart5rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    pub fn canrst(&self) -> CanrstR {
        CanrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BkprstR {
        BkprstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PwrrstR {
        PwrrstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DacrstR {
        DacrstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> Tim2rstW<Apb1rstrSpec> {
        Tim2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> Tim3rstW<Apb1rstrSpec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> Tim4rstW<Apb1rstrSpec> {
        Tim4rstW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> Tim5rstW<Apb1rstrSpec> {
        Tim5rstW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> Tim6rstW<Apb1rstrSpec> {
        Tim6rstW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> Tim7rstW<Apb1rstrSpec> {
        Tim7rstW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 12 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim12rst(&mut self) -> Tim12rstW<Apb1rstrSpec> {
        Tim12rstW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer 13 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim13rst(&mut self) -> Tim13rstW<Apb1rstrSpec> {
        Tim13rstW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> Tim14rstW<Apb1rstrSpec> {
        Tim14rstW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WwdgrstW<Apb1rstrSpec> {
        WwdgrstW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> Spi2rstW<Apb1rstrSpec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> Spi3rstW<Apb1rstrSpec> {
        Spi3rstW::new(self, 15)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> Usart2rstW<Apb1rstrSpec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> Usart3rstW<Apb1rstrSpec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> Uart4rstW<Apb1rstrSpec> {
        Uart4rstW::new(self, 19)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> Uart5rstW<Apb1rstrSpec> {
        Uart5rstW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apb1rstrSpec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2c2rstW<Apb1rstrSpec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<Apb1rstrSpec> {
        UsbrstW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    #[must_use]
    pub fn canrst(&mut self) -> CanrstW<Apb1rstrSpec> {
        CanrstW::new(self, 25)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprst(&mut self) -> BkprstW<Apb1rstrSpec> {
        BkprstW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PwrrstW<Apb1rstrSpec> {
        PwrrstW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dacrst(&mut self) -> DacrstW<Apb1rstrSpec> {
        DacrstW::new(self, 29)
    }
}
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstrSpec;
impl crate::RegisterSpec for Apb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr::R`](R) reader structure"]
impl crate::Readable for Apb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure"]
impl crate::Writable for Apb1rstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for Apb1rstrSpec {
    const RESET_VALUE: u32 = 0;
}
