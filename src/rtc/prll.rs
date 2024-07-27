#[doc = "Register `PRLL` writer"]
pub type W = crate::W<PrllSpec>;
#[doc = "Field `PRLL` writer - RTC Prescaler Divider Register Low"]
pub type PrllW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    #[must_use]
    pub fn prll(&mut self) -> PrllW<PrllSpec> {
        PrllW::new(self, 0)
    }
}
#[doc = "RTC Prescaler Load Register Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prll::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrllSpec;
impl crate::RegisterSpec for PrllSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prll::W`](W) writer structure"]
impl crate::Writable for PrllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRLL to value 0x8000"]
impl crate::Resettable for PrllSpec {
    const RESET_VALUE: u32 = 0x8000;
}
