#[doc = "Register `DR27` reader"]
pub type R = crate::R<Dr27Spec>;
#[doc = "Register `DR27` writer"]
pub type W = crate::W<Dr27Spec>;
#[doc = "Field `D27` reader - Backup data"]
pub type D27R = crate::FieldReader<u16>;
#[doc = "Field `D27` writer - Backup data"]
pub type D27W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d27(&self) -> D27R {
        D27R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d27(&mut self) -> D27W<Dr27Spec> {
        D27W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr27Spec;
impl crate::RegisterSpec for Dr27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr27::R`](R) reader structure"]
impl crate::Readable for Dr27Spec {}
#[doc = "`write(|w| ..)` method takes [`dr27::W`](W) writer structure"]
impl crate::Writable for Dr27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR27 to value 0"]
impl crate::Resettable for Dr27Spec {
    const RESET_VALUE: u32 = 0;
}
