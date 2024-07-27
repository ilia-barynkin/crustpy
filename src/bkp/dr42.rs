#[doc = "Register `DR42` reader"]
pub type R = crate::R<Dr42Spec>;
#[doc = "Register `DR42` writer"]
pub type W = crate::W<Dr42Spec>;
#[doc = "Field `D42` reader - Backup data"]
pub type D42R = crate::FieldReader<u16>;
#[doc = "Field `D42` writer - Backup data"]
pub type D42W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d42(&self) -> D42R {
        D42R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d42(&mut self) -> D42W<Dr42Spec> {
        D42W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr42Spec;
impl crate::RegisterSpec for Dr42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr42::R`](R) reader structure"]
impl crate::Readable for Dr42Spec {}
#[doc = "`write(|w| ..)` method takes [`dr42::W`](W) writer structure"]
impl crate::Writable for Dr42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR42 to value 0"]
impl crate::Resettable for Dr42Spec {
    const RESET_VALUE: u32 = 0;
}
