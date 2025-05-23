#[doc = "Register `CAN_RDL0R` reader"]
pub type R = crate::R<CanRdl0rSpec>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA2` reader - DATA2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA3` reader - DATA3"]
pub type Data3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CAN_RDL0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdl0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRdl0rSpec;
impl crate::RegisterSpec for CanRdl0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rdl0r::R`](R) reader structure"]
impl crate::Readable for CanRdl0rSpec {}
#[doc = "`reset()` method sets CAN_RDL0R to value 0"]
impl crate::Resettable for CanRdl0rSpec {
    const RESET_VALUE: u32 = 0;
}
