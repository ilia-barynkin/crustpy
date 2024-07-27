#[doc = "Register `DR32` reader"]
pub type R = crate::R<Dr32Spec>;
#[doc = "Register `DR32` writer"]
pub type W = crate::W<Dr32Spec>;
#[doc = "Field `D32` reader - Backup data"]
pub type D32R = crate::FieldReader<u16>;
#[doc = "Field `D32` writer - Backup data"]
pub type D32W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d32(&self) -> D32R {
        D32R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d32(&mut self) -> D32W<Dr32Spec> {
        D32W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr32Spec;
impl crate::RegisterSpec for Dr32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr32::R`](R) reader structure"]
impl crate::Readable for Dr32Spec {}
#[doc = "`write(|w| ..)` method takes [`dr32::W`](W) writer structure"]
impl crate::Writable for Dr32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR32 to value 0"]
impl crate::Resettable for Dr32Spec {
    const RESET_VALUE: u32 = 0;
}
