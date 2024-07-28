#[doc = "Register `MAPR2` reader"]
pub type R = crate::R<Mapr2Spec>;
#[doc = "Register `MAPR2` writer"]
pub type W = crate::W<Mapr2Spec>;
#[doc = "Field `TIM9_REMAP` reader - TIM9 remapping"]
pub type Tim9RemapR = crate::BitReader;
#[doc = "Field `TIM9_REMAP` writer - TIM9 remapping"]
pub type Tim9RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10_REMAP` reader - TIM10 remapping"]
pub type Tim10RemapR = crate::BitReader;
#[doc = "Field `TIM10_REMAP` writer - TIM10 remapping"]
pub type Tim10RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11_REMAP` reader - TIM11 remapping"]
pub type Tim11RemapR = crate::BitReader;
#[doc = "Field `TIM11_REMAP` writer - TIM11 remapping"]
pub type Tim11RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13_REMAP` reader - TIM13 remapping"]
pub type Tim13RemapR = crate::BitReader;
#[doc = "Field `TIM13_REMAP` writer - TIM13 remapping"]
pub type Tim13RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14_REMAP` reader - TIM14 remapping"]
pub type Tim14RemapR = crate::BitReader;
#[doc = "Field `TIM14_REMAP` writer - TIM14 remapping"]
pub type Tim14RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMC_NADV` reader - NADV connect/disconnect"]
pub type FsmcNadvR = crate::BitReader;
#[doc = "Field `FSMC_NADV` writer - NADV connect/disconnect"]
pub type FsmcNadvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&self) -> Tim9RemapR {
        Tim9RemapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&self) -> Tim10RemapR {
        Tim10RemapR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    pub fn tim11_remap(&self) -> Tim11RemapR {
        Tim11RemapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> Tim13RemapR {
        Tim13RemapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> Tim14RemapR {
        Tim14RemapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FsmcNadvR {
        FsmcNadvR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim9_remap(&mut self) -> Tim9RemapW<Mapr2Spec> {
        Tim9RemapW::new(self, 5)
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim10_remap(&mut self) -> Tim10RemapW<Mapr2Spec> {
        Tim10RemapW::new(self, 6)
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim11_remap(&mut self) -> Tim11RemapW<Mapr2Spec> {
        Tim11RemapW::new(self, 7)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim13_remap(&mut self) -> Tim13RemapW<Mapr2Spec> {
        Tim13RemapW::new(self, 8)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim14_remap(&mut self) -> Tim14RemapW<Mapr2Spec> {
        Tim14RemapW::new(self, 9)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_nadv(&mut self) -> FsmcNadvW<Mapr2Spec> {
        FsmcNadvW::new(self, 10)
    }
}
#[doc = "AF remap and debug I/O configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mapr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mapr2Spec;
impl crate::RegisterSpec for Mapr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mapr2::R`](R) reader structure"]
impl crate::Readable for Mapr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mapr2::W`](W) writer structure"]
impl crate::Writable for Mapr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAPR2 to value 0"]
impl crate::Resettable for Mapr2Spec {
    const RESET_VALUE: u32 = 0;
}
