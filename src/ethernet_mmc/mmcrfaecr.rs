#[doc = "Register `MMCRFAECR` reader"]
pub type R = crate::R<MmcrfaecrSpec>;
#[doc = "Field `RFAEC` reader - Received frames with alignment error counter"]
pub type RfaecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with alignment error counter"]
    #[inline(always)]
    pub fn rfaec(&self) -> RfaecR {
        RfaecR::new(self.bits)
    }
}
#[doc = "Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrfaecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrfaecrSpec;
impl crate::RegisterSpec for MmcrfaecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfaecr::R`](R) reader structure"]
impl crate::Readable for MmcrfaecrSpec {}
#[doc = "`reset()` method sets MMCRFAECR to value 0"]
impl crate::Resettable for MmcrfaecrSpec {
    const RESET_VALUE: u32 = 0;
}
