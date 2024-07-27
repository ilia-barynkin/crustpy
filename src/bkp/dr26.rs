#[doc = "Register `DR26` reader"]
pub type R = crate::R<Dr26Spec>;
#[doc = "Register `DR26` writer"]
pub type W = crate::W<Dr26Spec>;
#[doc = "Field `D26` reader - Backup data"]
pub type D26R = crate::FieldReader<u16>;
#[doc = "Field `D26` writer - Backup data"]
pub type D26W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d26(&self) -> D26R {
        D26R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d26(&mut self) -> D26W<Dr26Spec> {
        D26W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr26Spec;
impl crate::RegisterSpec for Dr26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr26::R`](R) reader structure"]
impl crate::Readable for Dr26Spec {}
#[doc = "`write(|w| ..)` method takes [`dr26::W`](W) writer structure"]
impl crate::Writable for Dr26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR26 to value 0"]
impl crate::Resettable for Dr26Spec {
    const RESET_VALUE: u32 = 0;
}
