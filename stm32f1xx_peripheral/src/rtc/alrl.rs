#[doc = "Register `ALRL` writer"]
pub type W = crate::W<AlrlSpec>;
#[doc = "Field `ALRL` writer - RTC alarm register low"]
pub type AlrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    #[must_use]
    pub fn alrl(&mut self) -> AlrlW<AlrlSpec> {
        AlrlW::new(self, 0)
    }
}
#[doc = "RTC Alarm Register Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlrlSpec;
impl crate::RegisterSpec for AlrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrl::W`](W) writer structure"]
impl crate::Writable for AlrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRL to value 0xffff"]
impl crate::Resettable for AlrlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
