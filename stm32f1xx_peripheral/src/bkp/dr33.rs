#[doc = "Register `DR33` reader"]
pub type R = crate::R<Dr33Spec>;
#[doc = "Register `DR33` writer"]
pub type W = crate::W<Dr33Spec>;
#[doc = "Field `D33` reader - Backup data"]
pub type D33R = crate::FieldReader<u16>;
#[doc = "Field `D33` writer - Backup data"]
pub type D33W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d33(&self) -> D33R {
        D33R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d33(&mut self) -> D33W<Dr33Spec> {
        D33W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr33Spec;
impl crate::RegisterSpec for Dr33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr33::R`](R) reader structure"]
impl crate::Readable for Dr33Spec {}
#[doc = "`write(|w| ..)` method takes [`dr33::W`](W) writer structure"]
impl crate::Writable for Dr33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR33 to value 0"]
impl crate::Resettable for Dr33Spec {
    const RESET_VALUE: u32 = 0;
}
