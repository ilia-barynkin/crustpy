#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub type FlitfenR = crate::BitReader;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub type FlitfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMCEN` reader - FSMC clock enable"]
pub type FsmcenR = crate::BitReader;
#[doc = "Field `FSMCEN` writer - FSMC clock enable"]
pub type FsmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub type SdioenR = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub type SdioenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FlitfenR {
        FlitfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FsmcenR {
        FsmcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> Dma1enW<AhbenrSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> Dma2enW<AhbenrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SramenW<AhbenrSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FlitfenW<AhbenrSpec> {
        FlitfenW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<AhbenrSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FsmcenW<AhbenrSpec> {
        FsmcenW::new(self, 8)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SdioenW<AhbenrSpec> {
        SdioenW::new(self, 10)
    }
}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AhbenrSpec {
    const RESET_VALUE: u32 = 0x14;
}
