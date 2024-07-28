#[doc = "Register `DR30` reader"]
pub type R = crate::R<Dr30Spec>;
#[doc = "Register `DR30` writer"]
pub type W = crate::W<Dr30Spec>;
#[doc = "Field `D30` reader - Backup data"]
pub type D30R = crate::FieldReader<u16>;
#[doc = "Field `D30` writer - Backup data"]
pub type D30W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d30(&self) -> D30R {
        D30R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d30(&mut self) -> D30W<Dr30Spec> {
        D30W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr30Spec;
impl crate::RegisterSpec for Dr30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr30::R`](R) reader structure"]
impl crate::Readable for Dr30Spec {}
#[doc = "`write(|w| ..)` method takes [`dr30::W`](W) writer structure"]
impl crate::Writable for Dr30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR30 to value 0"]
impl crate::Resettable for Dr30Spec {
    const RESET_VALUE: u32 = 0;
}
