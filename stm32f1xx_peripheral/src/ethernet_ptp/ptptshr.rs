#[doc = "Register `PTPTSHR` reader"]
pub type R = crate::R<PtptshrSpec>;
#[doc = "Field `STS` reader - System time second"]
pub type StsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - System time second"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptshr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptshrSpec;
impl crate::RegisterSpec for PtptshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshr::R`](R) reader structure"]
impl crate::Readable for PtptshrSpec {}
#[doc = "`reset()` method sets PTPTSHR to value 0"]
impl crate::Resettable for PtptshrSpec {
    const RESET_VALUE: u32 = 0;
}
