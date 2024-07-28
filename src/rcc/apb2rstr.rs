#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `AFIORST` reader - Alternate function I/O reset"]
pub type AfiorstR = crate::BitReader;
#[doc = "Field `AFIORST` writer - Alternate function I/O reset"]
pub type AfiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPARST` reader - IO port A reset"]
pub type IoparstR = crate::BitReader;
#[doc = "Field `IOPARST` writer - IO port A reset"]
pub type IoparstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBRST` reader - IO port B reset"]
pub type IopbrstR = crate::BitReader;
#[doc = "Field `IOPBRST` writer - IO port B reset"]
pub type IopbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCRST` reader - IO port C reset"]
pub type IopcrstR = crate::BitReader;
#[doc = "Field `IOPCRST` writer - IO port C reset"]
pub type IopcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDRST` reader - IO port D reset"]
pub type IopdrstR = crate::BitReader;
#[doc = "Field `IOPDRST` writer - IO port D reset"]
pub type IopdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPERST` reader - IO port E reset"]
pub type IoperstR = crate::BitReader;
#[doc = "Field `IOPERST` writer - IO port E reset"]
pub type IoperstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFRST` reader - IO port F reset"]
pub type IopfrstR = crate::BitReader;
#[doc = "Field `IOPFRST` writer - IO port F reset"]
pub type IopfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPGRST` reader - IO port G reset"]
pub type IopgrstR = crate::BitReader;
#[doc = "Field `IOPGRST` writer - IO port G reset"]
pub type IopgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - ADC 1 interface reset"]
pub type Adc1rstR = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC 1 interface reset"]
pub type Adc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2RST` reader - ADC 2 interface reset"]
pub type Adc2rstR = crate::BitReader;
#[doc = "Field `ADC2RST` writer - ADC 2 interface reset"]
pub type Adc2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type Tim1rstR = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - TIM8 timer reset"]
pub type Tim8rstR = crate::BitReader;
#[doc = "Field `TIM8RST` writer - TIM8 timer reset"]
pub type Tim8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3RST` reader - ADC 3 interface reset"]
pub type Adc3rstR = crate::BitReader;
#[doc = "Field `ADC3RST` writer - ADC 3 interface reset"]
pub type Adc3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9RST` reader - TIM9 timer reset"]
pub type Tim9rstR = crate::BitReader;
#[doc = "Field `TIM9RST` writer - TIM9 timer reset"]
pub type Tim9rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10RST` reader - TIM10 timer reset"]
pub type Tim10rstR = crate::BitReader;
#[doc = "Field `TIM10RST` writer - TIM10 timer reset"]
pub type Tim10rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11RST` reader - TIM11 timer reset"]
pub type Tim11rstR = crate::BitReader;
#[doc = "Field `TIM11RST` writer - TIM11 timer reset"]
pub type Tim11rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afiorst(&self) -> AfiorstR {
        AfiorstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IoparstR {
        IoparstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IopbrstR {
        IopbrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IopcrstR {
        IopcrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IopdrstR {
        IopdrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IoperstR {
        IoperstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IopfrstR {
        IopfrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port G reset"]
    #[inline(always)]
    pub fn iopgrst(&self) -> IopgrstR {
        IopgrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC 1 interface reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> Adc1rstR {
        Adc1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 2 interface reset"]
    #[inline(always)]
    pub fn adc2rst(&self) -> Adc2rstR {
        Adc2rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        Tim8rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC 3 interface reset"]
    #[inline(always)]
    pub fn adc3rst(&self) -> Adc3rstR {
        Adc3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM9 timer reset"]
    #[inline(always)]
    pub fn tim9rst(&self) -> Tim9rstR {
        Tim9rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM10 timer reset"]
    #[inline(always)]
    pub fn tim10rst(&self) -> Tim10rstR {
        Tim10rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIM11 timer reset"]
    #[inline(always)]
    pub fn tim11rst(&self) -> Tim11rstR {
        Tim11rstR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn afiorst(&mut self) -> AfiorstW<Apb2rstrSpec> {
        AfiorstW::new(self, 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IoparstW<Apb2rstrSpec> {
        IoparstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IopbrstW<Apb2rstrSpec> {
        IopbrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IopcrstW<Apb2rstrSpec> {
        IopcrstW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IopdrstW<Apb2rstrSpec> {
        IopdrstW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IoperstW<Apb2rstrSpec> {
        IoperstW::new(self, 6)
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IopfrstW<Apb2rstrSpec> {
        IopfrstW::new(self, 7)
    }
    #[doc = "Bit 8 - IO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopgrst(&mut self) -> IopgrstW<Apb2rstrSpec> {
        IopgrstW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC 1 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> Adc1rstW<Apb2rstrSpec> {
        Adc1rstW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC 2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc2rst(&mut self) -> Adc2rstW<Apb2rstrSpec> {
        Adc2rstW::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> Tim1rstW<Apb2rstrSpec> {
        Tim1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apb2rstrSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> Tim8rstW<Apb2rstrSpec> {
        Tim8rstW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apb2rstrSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 15 - ADC 3 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc3rst(&mut self) -> Adc3rstW<Apb2rstrSpec> {
        Adc3rstW::new(self, 15)
    }
    #[doc = "Bit 19 - TIM9 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim9rst(&mut self) -> Tim9rstW<Apb2rstrSpec> {
        Tim9rstW::new(self, 19)
    }
    #[doc = "Bit 20 - TIM10 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim10rst(&mut self) -> Tim10rstW<Apb2rstrSpec> {
        Tim10rstW::new(self, 20)
    }
    #[doc = "Bit 21 - TIM11 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim11rst(&mut self) -> Tim11rstW<Apb2rstrSpec> {
        Tim11rstW::new(self, 21)
    }
}
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {
    const RESET_VALUE: u32 = 0;
}
