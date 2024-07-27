#[doc = "Register `CAN_TDT2R` reader"]
pub type R = crate::R<CanTdt2rSpec>;
#[doc = "Register `CAN_TDT2R` writer"]
pub type W = crate::W<CanTdt2rSpec>;
#[doc = "Field `DLC` reader - DLC"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - DLC"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGT` reader - TGT"]
pub type TgtR = crate::BitReader;
#[doc = "Field `TGT` writer - TGT"]
pub type TgtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME` reader - TIME"]
pub type TimeR = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - TIME"]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&self) -> TgtR {
        TgtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<CanTdt2rSpec> {
        DlcW::new(self, 0)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    #[must_use]
    pub fn tgt(&mut self) -> TgtW<CanTdt2rSpec> {
        TgtW::new(self, 8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TimeW<CanTdt2rSpec> {
        TimeW::new(self, 16)
    }
}
#[doc = "CAN_TDT2R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdt2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdt2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTdt2rSpec;
impl crate::RegisterSpec for CanTdt2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tdt2r::R`](R) reader structure"]
impl crate::Readable for CanTdt2rSpec {}
#[doc = "`write(|w| ..)` method takes [`can_tdt2r::W`](W) writer structure"]
impl crate::Writable for CanTdt2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_TDT2R to value 0"]
impl crate::Resettable for CanTdt2rSpec {
    const RESET_VALUE: u32 = 0;
}
