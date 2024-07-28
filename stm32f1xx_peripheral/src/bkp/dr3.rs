#[doc = "Register `DR3` reader"]
pub type R = crate::R<Dr3Spec>;
#[doc = "Register `DR3` writer"]
pub type W = crate::W<Dr3Spec>;
#[doc = "Field `D3` reader - Backup data"]
pub type D3R = crate::FieldReader<u16>;
#[doc = "Field `D3` writer - Backup data"]
pub type D3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d3(&self) -> D3R {
        D3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d3(&mut self) -> D3W<Dr3Spec> {
        D3W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr3Spec;
impl crate::RegisterSpec for Dr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr3::R`](R) reader structure"]
impl crate::Readable for Dr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dr3::W`](W) writer structure"]
impl crate::Writable for Dr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for Dr3Spec {
    const RESET_VALUE: u32 = 0;
}
