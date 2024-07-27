#[doc = "Register `DR25` reader"]
pub type R = crate::R<Dr25Spec>;
#[doc = "Register `DR25` writer"]
pub type W = crate::W<Dr25Spec>;
#[doc = "Field `D25` reader - Backup data"]
pub type D25R = crate::FieldReader<u16>;
#[doc = "Field `D25` writer - Backup data"]
pub type D25W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d25(&self) -> D25R {
        D25R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d25(&mut self) -> D25W<Dr25Spec> {
        D25W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr25Spec;
impl crate::RegisterSpec for Dr25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr25::R`](R) reader structure"]
impl crate::Readable for Dr25Spec {}
#[doc = "`write(|w| ..)` method takes [`dr25::W`](W) writer structure"]
impl crate::Writable for Dr25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR25 to value 0"]
impl crate::Resettable for Dr25Spec {
    const RESET_VALUE: u32 = 0;
}
