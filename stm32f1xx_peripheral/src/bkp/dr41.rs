#[doc = "Register `DR41` reader"]
pub type R = crate::R<Dr41Spec>;
#[doc = "Register `DR41` writer"]
pub type W = crate::W<Dr41Spec>;
#[doc = "Field `D41` reader - Backup data"]
pub type D41R = crate::FieldReader<u16>;
#[doc = "Field `D41` writer - Backup data"]
pub type D41W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d41(&self) -> D41R {
        D41R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d41(&mut self) -> D41W<Dr41Spec> {
        D41W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr41Spec;
impl crate::RegisterSpec for Dr41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr41::R`](R) reader structure"]
impl crate::Readable for Dr41Spec {}
#[doc = "`write(|w| ..)` method takes [`dr41::W`](W) writer structure"]
impl crate::Writable for Dr41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR41 to value 0"]
impl crate::Resettable for Dr41Spec {
    const RESET_VALUE: u32 = 0;
}
