#[doc = "Register `CAN_RI0R` reader"]
pub type R = crate::R<CanRi0rSpec>;
#[doc = "Field `RTR` reader - RTR"]
pub type RtrR = crate::BitReader;
#[doc = "Field `IDE` reader - IDE"]
pub type IdeR = crate::BitReader;
#[doc = "Field `EXID` reader - EXID"]
pub type ExidR = crate::FieldReader<u32>;
#[doc = "Field `STID` reader - STID"]
pub type StidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> ExidR {
        ExidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> StidR {
        StidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "CAN_RI0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ri0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRi0rSpec;
impl crate::RegisterSpec for CanRi0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ri0r::R`](R) reader structure"]
impl crate::Readable for CanRi0rSpec {}
#[doc = "`reset()` method sets CAN_RI0R to value 0"]
impl crate::Resettable for CanRi0rSpec {
    const RESET_VALUE: u32 = 0;
}
