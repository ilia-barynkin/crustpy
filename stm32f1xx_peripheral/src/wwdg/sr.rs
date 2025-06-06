#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `EWI` reader - Early Wakeup Interrupt"]
pub type EwiR = crate::BitReader;
#[doc = "Field `EWI` writer - Early Wakeup Interrupt"]
pub type EwiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Early Wakeup Interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EwiR {
        EwiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early Wakeup Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EwiW<SrSpec> {
        EwiW::new(self, 0)
    }
}
#[doc = "Status register (WWDG_SR)\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
