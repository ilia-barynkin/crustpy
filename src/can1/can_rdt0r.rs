#[doc = "Register `CAN_RDT0R` reader"]
pub type R = crate::R<CanRdt0rSpec>;
#[doc = "Field `DLC` reader - DLC"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `FMI` reader - FMI"]
pub type FmiR = crate::FieldReader;
#[doc = "Field `TIME` reader - TIME"]
pub type TimeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FmiR {
        FmiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "CAN_RDT0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdt0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRdt0rSpec;
impl crate::RegisterSpec for CanRdt0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rdt0r::R`](R) reader structure"]
impl crate::Readable for CanRdt0rSpec {}
#[doc = "`reset()` method sets CAN_RDT0R to value 0"]
impl crate::Resettable for CanRdt0rSpec {
    const RESET_VALUE: u32 = 0;
}
