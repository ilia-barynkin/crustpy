#[doc = "Register `MMCRFCECR` reader"]
pub type R = crate::R<MmcrfcecrSpec>;
#[doc = "Field `RFCFC` reader - Received frames with CRC error counter"]
pub type RfcfcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with CRC error counter"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RfcfcR {
        RfcfcR::new(self.bits)
    }
}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrfcecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrfcecrSpec;
impl crate::RegisterSpec for MmcrfcecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfcecr::R`](R) reader structure"]
impl crate::Readable for MmcrfcecrSpec {}
#[doc = "`reset()` method sets MMCRFCECR to value 0"]
impl crate::Resettable for MmcrfcecrSpec {
    const RESET_VALUE: u32 = 0;
}
