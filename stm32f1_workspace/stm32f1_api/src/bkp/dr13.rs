#[doc = "Register `DR13` reader"]
pub type R = crate::R<Dr13Spec>;
#[doc = "Register `DR13` writer"]
pub type W = crate::W<Dr13Spec>;
#[doc = "Field `DR13` reader - Backup data"]
pub type Dr13R = crate::FieldReader<u16>;
#[doc = "Field `DR13` writer - Backup data"]
pub type Dr13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn dr13(&self) -> Dr13R {
        Dr13R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn dr13(&mut self) -> Dr13W<Dr13Spec> {
        Dr13W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr13Spec;
impl crate::RegisterSpec for Dr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr13::R`](R) reader structure"]
impl crate::Readable for Dr13Spec {}
#[doc = "`write(|w| ..)` method takes [`dr13::W`](W) writer structure"]
impl crate::Writable for Dr13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR13 to value 0"]
impl crate::Resettable for Dr13Spec {
    const RESET_VALUE: u32 = 0;
}
