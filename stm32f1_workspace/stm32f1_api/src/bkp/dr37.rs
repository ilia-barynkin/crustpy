#[doc = "Register `DR37` reader"]
pub type R = crate::R<Dr37Spec>;
#[doc = "Register `DR37` writer"]
pub type W = crate::W<Dr37Spec>;
#[doc = "Field `D37` reader - Backup data"]
pub type D37R = crate::FieldReader<u16>;
#[doc = "Field `D37` writer - Backup data"]
pub type D37W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d37(&self) -> D37R {
        D37R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d37(&mut self) -> D37W<Dr37Spec> {
        D37W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr37Spec;
impl crate::RegisterSpec for Dr37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr37::R`](R) reader structure"]
impl crate::Readable for Dr37Spec {}
#[doc = "`write(|w| ..)` method takes [`dr37::W`](W) writer structure"]
impl crate::Writable for Dr37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR37 to value 0"]
impl crate::Resettable for Dr37Spec {
    const RESET_VALUE: u32 = 0;
}
