#[doc = "Register `CFR` reader"]
pub type R = crate::R<CfrSpec>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CfrSpec>;
#[doc = "Field `W` reader - 7-bit window value"]
pub type WR = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value"]
pub type WW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGTB` reader - Timer Base"]
pub type WdgtbR = crate::FieldReader;
#[doc = "Field `WDGTB` writer - Timer Base"]
pub type WdgtbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EWI` reader - Early Wakeup Interrupt"]
pub type EwiR = crate::BitReader;
#[doc = "Field `EWI` writer - Early Wakeup Interrupt"]
pub type EwiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> WR {
        WR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Timer Base"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WdgtbR {
        WdgtbR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early Wakeup Interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EwiR {
        EwiR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> WW<CfrSpec> {
        WW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Timer Base"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WdgtbW<CfrSpec> {
        WdgtbW::new(self, 7)
    }
    #[doc = "Bit 9 - Early Wakeup Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EwiW<CfrSpec> {
        EwiW::new(self, 9)
    }
}
#[doc = "Configuration register (WWDG_CFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfrSpec;
impl crate::RegisterSpec for CfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CfrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CfrSpec {
    const RESET_VALUE: u32 = 0x7f;
}
