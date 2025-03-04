#[doc = "Register `DR19` reader"]
pub type R = crate::R<Dr19Spec>;
#[doc = "Register `DR19` writer"]
pub type W = crate::W<Dr19Spec>;
#[doc = "Field `D19` reader - Backup data"]
pub type D19R = crate::FieldReader<u16>;
#[doc = "Field `D19` writer - Backup data"]
pub type D19W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d19(&self) -> D19R {
        D19R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d19(&mut self) -> D19W<Dr19Spec> {
        D19W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr19Spec;
impl crate::RegisterSpec for Dr19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr19::R`](R) reader structure"]
impl crate::Readable for Dr19Spec {}
#[doc = "`write(|w| ..)` method takes [`dr19::W`](W) writer structure"]
impl crate::Writable for Dr19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR19 to value 0"]
impl crate::Resettable for Dr19Spec {
    const RESET_VALUE: u32 = 0;
}
