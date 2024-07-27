#[doc = "Register `DR31` reader"]
pub type R = crate::R<Dr31Spec>;
#[doc = "Register `DR31` writer"]
pub type W = crate::W<Dr31Spec>;
#[doc = "Field `D31` reader - Backup data"]
pub type D31R = crate::FieldReader<u16>;
#[doc = "Field `D31` writer - Backup data"]
pub type D31W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d31(&self) -> D31R {
        D31R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d31(&mut self) -> D31W<Dr31Spec> {
        D31W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr31Spec;
impl crate::RegisterSpec for Dr31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr31::R`](R) reader structure"]
impl crate::Readable for Dr31Spec {}
#[doc = "`write(|w| ..)` method takes [`dr31::W`](W) writer structure"]
impl crate::Writable for Dr31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR31 to value 0"]
impl crate::Resettable for Dr31Spec {
    const RESET_VALUE: u32 = 0;
}
