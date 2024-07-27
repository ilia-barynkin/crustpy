#[doc = "Register `EVCR` reader"]
pub type R = crate::R<EvcrSpec>;
#[doc = "Register `EVCR` writer"]
pub type W = crate::W<EvcrSpec>;
#[doc = "Field `PIN` reader - Pin selection"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - Pin selection"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PORT` reader - Port selection"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - Port selection"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EVOE` reader - Event Output Enable"]
pub type EvoeR = crate::BitReader;
#[doc = "Field `EVOE` writer - Event Output Enable"]
pub type EvoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EvoeR {
        EvoeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<EvcrSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<EvcrSpec> {
        PortW::new(self, 4)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoe(&mut self) -> EvoeW<EvcrSpec> {
        EvoeW::new(self, 7)
    }
}
#[doc = "Event Control Register (AFIO_EVCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`evcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvcrSpec;
impl crate::RegisterSpec for EvcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evcr::R`](R) reader structure"]
impl crate::Readable for EvcrSpec {}
#[doc = "`write(|w| ..)` method takes [`evcr::W`](W) writer structure"]
impl crate::Writable for EvcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVCR to value 0"]
impl crate::Resettable for EvcrSpec {
    const RESET_VALUE: u32 = 0;
}
