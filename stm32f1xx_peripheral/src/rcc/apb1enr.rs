#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<Apb1enrSpec>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<Apb1enrSpec>;
#[doc = "Field `TIM2EN` reader - Timer 2 clock enable"]
pub type Tim2enR = crate::BitReader;
#[doc = "Field `TIM2EN` writer - Timer 2 clock enable"]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3EN` reader - Timer 3 clock enable"]
pub type Tim3enR = crate::BitReader;
#[doc = "Field `TIM3EN` writer - Timer 3 clock enable"]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4EN` reader - Timer 4 clock enable"]
pub type Tim4enR = crate::BitReader;
#[doc = "Field `TIM4EN` writer - Timer 4 clock enable"]
pub type Tim4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5EN` reader - Timer 5 clock enable"]
pub type Tim5enR = crate::BitReader;
#[doc = "Field `TIM5EN` writer - Timer 5 clock enable"]
pub type Tim5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable"]
pub type Tim6enR = crate::BitReader;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable"]
pub type Tim6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7EN` reader - Timer 7 clock enable"]
pub type Tim7enR = crate::BitReader;
#[doc = "Field `TIM7EN` writer - Timer 7 clock enable"]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12EN` reader - Timer 12 clock enable"]
pub type Tim12enR = crate::BitReader;
#[doc = "Field `TIM12EN` writer - Timer 12 clock enable"]
pub type Tim12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13EN` reader - Timer 13 clock enable"]
pub type Tim13enR = crate::BitReader;
#[doc = "Field `TIM13EN` writer - Timer 13 clock enable"]
pub type Tim13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14EN` reader - Timer 14 clock enable"]
pub type Tim14enR = crate::BitReader;
#[doc = "Field `TIM14EN` writer - Timer 14 clock enable"]
pub type Tim14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub type WwdgenR = crate::BitReader;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub type WwdgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI 2 clock enable"]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI 2 clock enable"]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - SPI 3 clock enable"]
pub type Spi3enR = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI 3 clock enable"]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3EN` reader - USART 3 clock enable"]
pub type Usart3enR = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART 3 clock enable"]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4EN` reader - UART 4 clock enable"]
pub type Uart4enR = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART 4 clock enable"]
pub type Uart4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5EN` reader - UART 5 clock enable"]
pub type Uart5enR = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART 5 clock enable"]
pub type Uart5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C 1 clock enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C 1 clock enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C 2 clock enable"]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C 2 clock enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEN` reader - USB clock enable"]
pub type UsbenR = crate::BitReader;
#[doc = "Field `USBEN` writer - USB clock enable"]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANEN` reader - CAN clock enable"]
pub type CanenR = crate::BitReader;
#[doc = "Field `CANEN` writer - CAN clock enable"]
pub type CanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPEN` reader - Backup interface clock enable"]
pub type BkpenR = crate::BitReader;
#[doc = "Field `BKPEN` writer - Backup interface clock enable"]
pub type BkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PwrenR = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub type DacenR = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> Tim4enR {
        Tim4enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> Tim5enR {
        Tim5enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> Tim6enR {
        Tim6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 12 clock enable"]
    #[inline(always)]
    pub fn tim12en(&self) -> Tim12enR {
        Tim12enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 13 clock enable"]
    #[inline(always)]
    pub fn tim13en(&self) -> Tim13enR {
        Tim13enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 14 clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> Tim14enR {
        Tim14enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WwdgenR {
        WwdgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART 4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> Uart4enR {
        Uart4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> Uart5enR {
        Uart5enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN clock enable"]
    #[inline(always)]
    pub fn canen(&self) -> CanenR {
        CanenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    pub fn bkpen(&self) -> BkpenR {
        BkpenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> Tim2enW<Apb1enrSpec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> Tim3enW<Apb1enrSpec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> Tim4enW<Apb1enrSpec> {
        Tim4enW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> Tim5enW<Apb1enrSpec> {
        Tim5enW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> Tim6enW<Apb1enrSpec> {
        Tim6enW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> Tim7enW<Apb1enrSpec> {
        Tim7enW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 12 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim12en(&mut self) -> Tim12enW<Apb1enrSpec> {
        Tim12enW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer 13 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim13en(&mut self) -> Tim13enW<Apb1enrSpec> {
        Tim13enW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer 14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> Tim14enW<Apb1enrSpec> {
        Tim14enW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WwdgenW<Apb1enrSpec> {
        WwdgenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> Spi2enW<Apb1enrSpec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> Spi3enW<Apb1enrSpec> {
        Spi3enW::new(self, 15)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> Usart2enW<Apb1enrSpec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> Usart3enW<Apb1enrSpec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19 - UART 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> Uart4enW<Apb1enrSpec> {
        Uart4enW::new(self, 19)
    }
    #[doc = "Bit 20 - UART 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> Uart5enW<Apb1enrSpec> {
        Uart5enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2c1enW<Apb1enrSpec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2c2enW<Apb1enrSpec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> UsbenW<Apb1enrSpec> {
        UsbenW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CanenW<Apb1enrSpec> {
        CanenW::new(self, 25)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpen(&mut self) -> BkpenW<Apb1enrSpec> {
        BkpenW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PwrenW<Apb1enrSpec> {
        PwrenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DacenW<Apb1enrSpec> {
        DacenW::new(self, 29)
    }
}
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enrSpec;
impl crate::RegisterSpec for Apb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr::R`](R) reader structure"]
impl crate::Readable for Apb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure"]
impl crate::Writable for Apb1enrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for Apb1enrSpec {
    const RESET_VALUE: u32 = 0;
}
