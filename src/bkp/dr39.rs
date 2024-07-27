#[doc = "Register `DR39` reader"]
pub type R = crate::R<Dr39Spec>;
#[doc = "Register `DR39` writer"]
pub type W = crate::W<Dr39Spec>;
#[doc = "Field `D39` reader - Backup data"]
pub type D39R = crate::FieldReader<u16>;
#[doc = "Field `D39` writer - Backup data"]
pub type D39W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d39(&self) -> D39R {
        D39R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d39(&mut self) -> D39W<Dr39Spec> {
        D39W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr39Spec;
impl crate::RegisterSpec for Dr39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr39::R`](R) reader structure"]
impl crate::Readable for Dr39Spec {}
#[doc = "`write(|w| ..)` method takes [`dr39::W`](W) writer structure"]
impl crate::Writable for Dr39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR39 to value 0"]
impl crate::Resettable for Dr39Spec {
    const RESET_VALUE: u32 = 0;
}
