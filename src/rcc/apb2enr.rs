#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<Apb2enrSpec>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<Apb2enrSpec>;
#[doc = "Field `AFIOEN` reader - Alternate function I/O clock enable"]
pub type AfioenR = crate::BitReader;
#[doc = "Field `AFIOEN` writer - Alternate function I/O clock enable"]
pub type AfioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub type IopaenR = crate::BitReader;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub type IopaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub type IopbenR = crate::BitReader;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub type IopbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub type IopcenR = crate::BitReader;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub type IopcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub type IopdenR = crate::BitReader;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub type IopdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPEEN` reader - I/O port E clock enable"]
pub type IopeenR = crate::BitReader;
#[doc = "Field `IOPEEN` writer - I/O port E clock enable"]
pub type IopeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub type IopfenR = crate::BitReader;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub type IopfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPGEN` reader - I/O port G clock enable"]
pub type IopgenR = crate::BitReader;
#[doc = "Field `IOPGEN` writer - I/O port G clock enable"]
pub type IopgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1EN` reader - ADC 1 interface clock enable"]
pub type Adc1enR = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC 1 interface clock enable"]
pub type Adc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2EN` reader - ADC 2 interface clock enable"]
pub type Adc2enR = crate::BitReader;
#[doc = "Field `ADC2EN` writer - ADC 2 interface clock enable"]
pub type Adc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub type Tim1enR = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - TIM8 Timer clock enable"]
pub type Tim8enR = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8 Timer clock enable"]
pub type Tim8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - ADC3 interface clock enable"]
pub type Adc3enR = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 interface clock enable"]
pub type Adc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9EN` reader - TIM9 Timer clock enable"]
pub type Tim9enR = crate::BitReader;
#[doc = "Field `TIM9EN` writer - TIM9 Timer clock enable"]
pub type Tim9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10EN` reader - TIM10 Timer clock enable"]
pub type Tim10enR = crate::BitReader;
#[doc = "Field `TIM10EN` writer - TIM10 Timer clock enable"]
pub type Tim10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11EN` reader - TIM11 Timer clock enable"]
pub type Tim11enR = crate::BitReader;
#[doc = "Field `TIM11EN` writer - TIM11 Timer clock enable"]
pub type Tim11enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&self) -> AfioenR {
        AfioenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IopaenR {
        IopaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IopbenR {
        IopbenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IopcenR {
        IopcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IopdenR {
        IopdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&self) -> IopeenR {
        IopeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IopfenR {
        IopfenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&self) -> IopgenR {
        IopgenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> Adc2enR {
        Adc2enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC3 interface clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> Tim9enR {
        Tim9enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> Tim10enR {
        Tim10enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIM11 Timer clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> Tim11enR {
        Tim11enR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AfioenW<Apb2enrSpec> {
        AfioenW::new(self, 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IopaenW<Apb2enrSpec> {
        IopaenW::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IopbenW<Apb2enrSpec> {
        IopbenW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IopcenW<Apb2enrSpec> {
        IopcenW::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IopdenW<Apb2enrSpec> {
        IopdenW::new(self, 5)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopeen(&mut self) -> IopeenW<Apb2enrSpec> {
        IopeenW::new(self, 6)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IopfenW<Apb2enrSpec> {
        IopfenW::new(self, 7)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopgen(&mut self) -> IopgenW<Apb2enrSpec> {
        IopgenW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> Adc1enW<Apb2enrSpec> {
        Adc1enW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2en(&mut self) -> Adc2enW<Apb2enrSpec> {
        Adc2enW::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> Tim1enW<Apb2enrSpec> {
        Tim1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> Spi1enW<Apb2enrSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> Tim8enW<Apb2enrSpec> {
        Tim8enW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> Usart1enW<Apb2enrSpec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 15 - ADC3 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> Adc3enW<Apb2enrSpec> {
        Adc3enW::new(self, 15)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim9en(&mut self) -> Tim9enW<Apb2enrSpec> {
        Tim9enW::new(self, 19)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim10en(&mut self) -> Tim10enW<Apb2enrSpec> {
        Tim10enW::new(self, 20)
    }
    #[doc = "Bit 21 - TIM11 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim11en(&mut self) -> Tim11enW<Apb2enrSpec> {
        Tim11enW::new(self, 21)
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enrSpec;
impl crate::RegisterSpec for Apb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for Apb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for Apb2enrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for Apb2enrSpec {
    const RESET_VALUE: u32 = 0;
}
