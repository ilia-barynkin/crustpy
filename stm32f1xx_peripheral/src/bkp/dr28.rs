#[doc = "Register `DR28` reader"]
pub type R = crate::R<Dr28Spec>;
#[doc = "Register `DR28` writer"]
pub type W = crate::W<Dr28Spec>;
#[doc = "Field `D28` reader - Backup data"]
pub type D28R = crate::FieldReader<u16>;
#[doc = "Field `D28` writer - Backup data"]
pub type D28W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d28(&self) -> D28R {
        D28R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d28(&mut self) -> D28W<Dr28Spec> {
        D28W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr28Spec;
impl crate::RegisterSpec for Dr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr28::R`](R) reader structure"]
impl crate::Readable for Dr28Spec {}
#[doc = "`write(|w| ..)` method takes [`dr28::W`](W) writer structure"]
impl crate::Writable for Dr28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR28 to value 0"]
impl crate::Resettable for Dr28Spec {
    const RESET_VALUE: u32 = 0;
}
