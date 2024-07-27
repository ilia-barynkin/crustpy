#[doc = "Register `DR24` reader"]
pub type R = crate::R<Dr24Spec>;
#[doc = "Register `DR24` writer"]
pub type W = crate::W<Dr24Spec>;
#[doc = "Field `D24` reader - Backup data"]
pub type D24R = crate::FieldReader<u16>;
#[doc = "Field `D24` writer - Backup data"]
pub type D24W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d24(&self) -> D24R {
        D24R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d24(&mut self) -> D24W<Dr24Spec> {
        D24W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr24Spec;
impl crate::RegisterSpec for Dr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr24::R`](R) reader structure"]
impl crate::Readable for Dr24Spec {}
#[doc = "`write(|w| ..)` method takes [`dr24::W`](W) writer structure"]
impl crate::Writable for Dr24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR24 to value 0"]
impl crate::Resettable for Dr24Spec {
    const RESET_VALUE: u32 = 0;
}
