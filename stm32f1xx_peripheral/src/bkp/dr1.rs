#[doc = "Register `DR1` reader"]
pub type R = crate::R<Dr1Spec>;
#[doc = "Register `DR1` writer"]
pub type W = crate::W<Dr1Spec>;
#[doc = "Field `D1` reader - Backup data"]
pub type D1R = crate::FieldReader<u16>;
#[doc = "Field `D1` writer - Backup data"]
pub type D1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d1(&self) -> D1R {
        D1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d1(&mut self) -> D1W<Dr1Spec> {
        D1W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr1Spec;
impl crate::RegisterSpec for Dr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr1::R`](R) reader structure"]
impl crate::Readable for Dr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dr1::W`](W) writer structure"]
impl crate::Writable for Dr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for Dr1Spec {
    const RESET_VALUE: u32 = 0;
}
