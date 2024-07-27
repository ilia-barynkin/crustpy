#[doc = "Register `DR10` reader"]
pub type R = crate::R<Dr10Spec>;
#[doc = "Register `DR10` writer"]
pub type W = crate::W<Dr10Spec>;
#[doc = "Field `D10` reader - Backup data"]
pub type D10R = crate::FieldReader<u16>;
#[doc = "Field `D10` writer - Backup data"]
pub type D10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d10(&self) -> D10R {
        D10R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d10(&mut self) -> D10W<Dr10Spec> {
        D10W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr10Spec;
impl crate::RegisterSpec for Dr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr10::R`](R) reader structure"]
impl crate::Readable for Dr10Spec {}
#[doc = "`write(|w| ..)` method takes [`dr10::W`](W) writer structure"]
impl crate::Writable for Dr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR10 to value 0"]
impl crate::Resettable for Dr10Spec {
    const RESET_VALUE: u32 = 0;
}
