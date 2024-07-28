#[doc = "Register `DR36` reader"]
pub type R = crate::R<Dr36Spec>;
#[doc = "Register `DR36` writer"]
pub type W = crate::W<Dr36Spec>;
#[doc = "Field `D36` reader - Backup data"]
pub type D36R = crate::FieldReader<u16>;
#[doc = "Field `D36` writer - Backup data"]
pub type D36W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d36(&self) -> D36R {
        D36R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d36(&mut self) -> D36W<Dr36Spec> {
        D36W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr36Spec;
impl crate::RegisterSpec for Dr36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr36::R`](R) reader structure"]
impl crate::Readable for Dr36Spec {}
#[doc = "`write(|w| ..)` method takes [`dr36::W`](W) writer structure"]
impl crate::Writable for Dr36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR36 to value 0"]
impl crate::Resettable for Dr36Spec {
    const RESET_VALUE: u32 = 0;
}
