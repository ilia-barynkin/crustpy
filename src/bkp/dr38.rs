#[doc = "Register `DR38` reader"]
pub type R = crate::R<Dr38Spec>;
#[doc = "Register `DR38` writer"]
pub type W = crate::W<Dr38Spec>;
#[doc = "Field `D38` reader - Backup data"]
pub type D38R = crate::FieldReader<u16>;
#[doc = "Field `D38` writer - Backup data"]
pub type D38W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d38(&self) -> D38R {
        D38R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d38(&mut self) -> D38W<Dr38Spec> {
        D38W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr38Spec;
impl crate::RegisterSpec for Dr38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr38::R`](R) reader structure"]
impl crate::Readable for Dr38Spec {}
#[doc = "`write(|w| ..)` method takes [`dr38::W`](W) writer structure"]
impl crate::Writable for Dr38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR38 to value 0"]
impl crate::Resettable for Dr38Spec {
    const RESET_VALUE: u32 = 0;
}
