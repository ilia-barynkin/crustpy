#[doc = "Register `DR22` reader"]
pub type R = crate::R<Dr22Spec>;
#[doc = "Register `DR22` writer"]
pub type W = crate::W<Dr22Spec>;
#[doc = "Field `D22` reader - Backup data"]
pub type D22R = crate::FieldReader<u16>;
#[doc = "Field `D22` writer - Backup data"]
pub type D22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d22(&self) -> D22R {
        D22R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d22(&mut self) -> D22W<Dr22Spec> {
        D22W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr22Spec;
impl crate::RegisterSpec for Dr22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr22::R`](R) reader structure"]
impl crate::Readable for Dr22Spec {}
#[doc = "`write(|w| ..)` method takes [`dr22::W`](W) writer structure"]
impl crate::Writable for Dr22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR22 to value 0"]
impl crate::Resettable for Dr22Spec {
    const RESET_VALUE: u32 = 0;
}
