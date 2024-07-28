#[doc = "Register `DR29` reader"]
pub type R = crate::R<Dr29Spec>;
#[doc = "Register `DR29` writer"]
pub type W = crate::W<Dr29Spec>;
#[doc = "Field `D29` reader - Backup data"]
pub type D29R = crate::FieldReader<u16>;
#[doc = "Field `D29` writer - Backup data"]
pub type D29W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d29(&self) -> D29R {
        D29R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d29(&mut self) -> D29W<Dr29Spec> {
        D29W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr29Spec;
impl crate::RegisterSpec for Dr29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr29::R`](R) reader structure"]
impl crate::Readable for Dr29Spec {}
#[doc = "`write(|w| ..)` method takes [`dr29::W`](W) writer structure"]
impl crate::Writable for Dr29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR29 to value 0"]
impl crate::Resettable for Dr29Spec {
    const RESET_VALUE: u32 = 0;
}
