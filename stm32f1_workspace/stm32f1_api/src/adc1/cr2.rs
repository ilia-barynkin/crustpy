#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADON` reader - A/D converter ON / OFF"]
pub type AdonR = crate::BitReader;
#[doc = "Field `ADON` writer - A/D converter ON / OFF"]
pub type AdonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - Continuous conversion"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous conversion"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL` reader - A/D calibration"]
pub type CalR = crate::BitReader;
#[doc = "Field `CAL` writer - A/D calibration"]
pub type CalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTCAL` reader - Reset calibration"]
pub type RstcalR = crate::BitReader;
#[doc = "Field `RSTCAL` writer - Reset calibration"]
pub type RstcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - Direct memory access mode"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - Direct memory access mode"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type AlignR = crate::BitReader;
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEXTSEL` reader - External event select for injected group"]
pub type JextselR = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - External event select for injected group"]
pub type JextselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEXTTRIG` reader - External trigger conversion mode for injected channels"]
pub type JexttrigR = crate::BitReader;
#[doc = "Field `JEXTTRIG` writer - External trigger conversion mode for injected channels"]
pub type JexttrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub type ExtselR = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub type ExtselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTTRIG` reader - External trigger conversion mode for regular channels"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `EXTTRIG` writer - External trigger conversion mode for regular channels"]
pub type ExttrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSWSTART` reader - Start conversion of injected channels"]
pub type JswstartR = crate::BitReader;
#[doc = "Field `JSWSTART` writer - Start conversion of injected channels"]
pub type JswstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSTART` reader - Start conversion of regular channels"]
pub type SwstartR = crate::BitReader;
#[doc = "Field `SWSTART` writer - Start conversion of regular channels"]
pub type SwstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TsvrefeR = crate::BitReader;
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TsvrefeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> AdonR {
        AdonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&self) -> RstcalR {
        RstcalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JextselR {
        JextselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn jexttrig(&self) -> JexttrigR {
        JexttrigR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> ExtselR {
        ExtselR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JswstartR {
        JswstartR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SwstartR {
        SwstartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TsvrefeR {
        TsvrefeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> AdonW<Cr2Spec> {
        AdonW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<Cr2Spec> {
        ContW::new(self, 1)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CalW<Cr2Spec> {
        CalW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstcal(&mut self) -> RstcalW<Cr2Spec> {
        RstcalW::new(self, 3)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Cr2Spec> {
        DmaW::new(self, 8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> AlignW<Cr2Spec> {
        AlignW::new(self, 11)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JextselW<Cr2Spec> {
        JextselW::new(self, 12)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jexttrig(&mut self) -> JexttrigW<Cr2Spec> {
        JexttrigW::new(self, 15)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> ExtselW<Cr2Spec> {
        ExtselW::new(self, 17)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn exttrig(&mut self) -> ExttrigW<Cr2Spec> {
        ExttrigW::new(self, 20)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JswstartW<Cr2Spec> {
        JswstartW::new(self, 21)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SwstartW<Cr2Spec> {
        SwstartW::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsvrefe(&mut self) -> TsvrefeW<Cr2Spec> {
        TsvrefeW::new(self, 23)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
