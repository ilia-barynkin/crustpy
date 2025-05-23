#[doc = "Register `DR14` reader"]
pub type R = crate::R<Dr14Spec>;
#[doc = "Register `DR14` writer"]
pub type W = crate::W<Dr14Spec>;
#[doc = "Field `D14` reader - Backup data"]
pub type D14R = crate::FieldReader<u16>;
#[doc = "Field `D14` writer - Backup data"]
pub type D14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d14(&self) -> D14R {
        D14R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d14(&mut self) -> D14W<Dr14Spec> {
        D14W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr14Spec;
impl crate::RegisterSpec for Dr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr14::R`](R) reader structure"]
impl crate::Readable for Dr14Spec {}
#[doc = "`write(|w| ..)` method takes [`dr14::W`](W) writer structure"]
impl crate::Writable for Dr14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR14 to value 0"]
impl crate::Resettable for Dr14Spec {
    const RESET_VALUE: u32 = 0;
}
