#[doc = "Register `DR40` reader"]
pub type R = crate::R<Dr40Spec>;
#[doc = "Register `DR40` writer"]
pub type W = crate::W<Dr40Spec>;
#[doc = "Field `D40` reader - Backup data"]
pub type D40R = crate::FieldReader<u16>;
#[doc = "Field `D40` writer - Backup data"]
pub type D40W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d40(&self) -> D40R {
        D40R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d40(&mut self) -> D40W<Dr40Spec> {
        D40W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr40Spec;
impl crate::RegisterSpec for Dr40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr40::R`](R) reader structure"]
impl crate::Readable for Dr40Spec {}
#[doc = "`write(|w| ..)` method takes [`dr40::W`](W) writer structure"]
impl crate::Writable for Dr40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR40 to value 0"]
impl crate::Resettable for Dr40Spec {
    const RESET_VALUE: u32 = 0;
}
