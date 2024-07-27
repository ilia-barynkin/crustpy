#[doc = "Register `CAN_FMR` reader"]
pub type R = crate::R<CanFmrSpec>;
#[doc = "Register `CAN_FMR` writer"]
pub type W = crate::W<CanFmrSpec>;
#[doc = "Field `FINIT` reader - FINIT"]
pub type FinitR = crate::BitReader;
#[doc = "Field `FINIT` writer - FINIT"]
pub type FinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    pub fn finit(&self) -> FinitR {
        FinitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    #[must_use]
    pub fn finit(&mut self) -> FinitW<CanFmrSpec> {
        FinitW::new(self, 0)
    }
}
#[doc = "CAN_FMR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFmrSpec;
impl crate::RegisterSpec for CanFmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fmr::R`](R) reader structure"]
impl crate::Readable for CanFmrSpec {}
#[doc = "`write(|w| ..)` method takes [`can_fmr::W`](W) writer structure"]
impl crate::Writable for CanFmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_FMR to value 0"]
impl crate::Resettable for CanFmrSpec {
    const RESET_VALUE: u32 = 0;
}
