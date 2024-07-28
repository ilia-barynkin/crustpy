#[doc = "Register `LOAD_` reader"]
pub type R = crate::R<Load_Spec>;
#[doc = "Register `LOAD_` writer"]
pub type W = crate::W<Load_Spec>;
#[doc = "Field `RELOAD` reader - RELOAD value"]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - RELOAD value"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - RELOAD value"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - RELOAD value"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<Load_Spec> {
        ReloadW::new(self, 0)
    }
}
#[doc = "SysTick reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`load_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Load_Spec;
impl crate::RegisterSpec for Load_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_::R`](R) reader structure"]
impl crate::Readable for Load_Spec {}
#[doc = "`write(|w| ..)` method takes [`load_::W`](W) writer structure"]
impl crate::Writable for Load_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOAD_ to value 0"]
impl crate::Resettable for Load_Spec {
    const RESET_VALUE: u32 = 0;
}
