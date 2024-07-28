#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IdcodeSpec>;
#[doc = "Field `DEV_ID` reader - DEV_ID"]
pub type DevIdR = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - REV_ID"]
pub type RevIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DEV_ID"]
    #[inline(always)]
    pub fn dev_id(&self) -> DevIdR {
        DevIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - REV_ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU_IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcodeSpec;
impl crate::RegisterSpec for IdcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IdcodeSpec {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IdcodeSpec {
    const RESET_VALUE: u32 = 0;
}
