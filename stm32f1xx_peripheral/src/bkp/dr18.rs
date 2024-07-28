#[doc = "Register `DR18` reader"]
pub type R = crate::R<Dr18Spec>;
#[doc = "Register `DR18` writer"]
pub type W = crate::W<Dr18Spec>;
#[doc = "Field `D18` reader - Backup data"]
pub type D18R = crate::FieldReader<u16>;
#[doc = "Field `D18` writer - Backup data"]
pub type D18W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d18(&self) -> D18R {
        D18R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d18(&mut self) -> D18W<Dr18Spec> {
        D18W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr18Spec;
impl crate::RegisterSpec for Dr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr18::R`](R) reader structure"]
impl crate::Readable for Dr18Spec {}
#[doc = "`write(|w| ..)` method takes [`dr18::W`](W) writer structure"]
impl crate::Writable for Dr18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR18 to value 0"]
impl crate::Resettable for Dr18Spec {
    const RESET_VALUE: u32 = 0;
}
