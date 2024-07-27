#[doc = "Register `DR34` reader"]
pub type R = crate::R<Dr34Spec>;
#[doc = "Register `DR34` writer"]
pub type W = crate::W<Dr34Spec>;
#[doc = "Field `D34` reader - Backup data"]
pub type D34R = crate::FieldReader<u16>;
#[doc = "Field `D34` writer - Backup data"]
pub type D34W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d34(&self) -> D34R {
        D34R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d34(&mut self) -> D34W<Dr34Spec> {
        D34W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr34Spec;
impl crate::RegisterSpec for Dr34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr34::R`](R) reader structure"]
impl crate::Readable for Dr34Spec {}
#[doc = "`write(|w| ..)` method takes [`dr34::W`](W) writer structure"]
impl crate::Writable for Dr34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR34 to value 0"]
impl crate::Resettable for Dr34Spec {
    const RESET_VALUE: u32 = 0;
}
