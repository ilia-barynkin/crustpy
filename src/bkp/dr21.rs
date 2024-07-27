#[doc = "Register `DR21` reader"]
pub type R = crate::R<Dr21Spec>;
#[doc = "Register `DR21` writer"]
pub type W = crate::W<Dr21Spec>;
#[doc = "Field `D21` reader - Backup data"]
pub type D21R = crate::FieldReader<u16>;
#[doc = "Field `D21` writer - Backup data"]
pub type D21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d21(&self) -> D21R {
        D21R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d21(&mut self) -> D21W<Dr21Spec> {
        D21W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr21Spec;
impl crate::RegisterSpec for Dr21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr21::R`](R) reader structure"]
impl crate::Readable for Dr21Spec {}
#[doc = "`write(|w| ..)` method takes [`dr21::W`](W) writer structure"]
impl crate::Writable for Dr21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR21 to value 0"]
impl crate::Resettable for Dr21Spec {
    const RESET_VALUE: u32 = 0;
}
