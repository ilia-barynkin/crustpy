#[doc = "Register `CAN_ESR` reader"]
pub type R = crate::R<CanEsrSpec>;
#[doc = "Register `CAN_ESR` writer"]
pub type W = crate::W<CanEsrSpec>;
#[doc = "Field `EWGF` reader - EWGF"]
pub type EwgfR = crate::BitReader;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EpvfR = crate::BitReader;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BoffR = crate::BitReader;
#[doc = "Field `LEC` reader - LEC"]
pub type LecR = crate::FieldReader;
#[doc = "Field `LEC` writer - LEC"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TEC` reader - TEC"]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - REC"]
pub type RecR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EwgfR {
        EwgfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EpvfR {
        EpvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<CanEsrSpec> {
        LecW::new(self, 4)
    }
}
#[doc = "CAN_ESR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_esr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_esr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanEsrSpec;
impl crate::RegisterSpec for CanEsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_esr::R`](R) reader structure"]
impl crate::Readable for CanEsrSpec {}
#[doc = "`write(|w| ..)` method takes [`can_esr::W`](W) writer structure"]
impl crate::Writable for CanEsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_ESR to value 0"]
impl crate::Resettable for CanEsrSpec {
    const RESET_VALUE: u32 = 0;
}
