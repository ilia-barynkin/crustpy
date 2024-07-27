#[doc = "Register `MMCTGFMSCCR` reader"]
pub type R = crate::R<MmctgfmsccrSpec>;
#[doc = "Field `TGFMSCC` reader - Transmitted good frames after more than a single collision counter"]
pub type TgfmsccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after more than a single collision counter"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TgfmsccR {
        TgfmsccR::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfmsccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctgfmsccrSpec;
impl crate::RegisterSpec for MmctgfmsccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfmsccr::R`](R) reader structure"]
impl crate::Readable for MmctgfmsccrSpec {}
#[doc = "`reset()` method sets MMCTGFMSCCR to value 0"]
impl crate::Resettable for MmctgfmsccrSpec {
    const RESET_VALUE: u32 = 0;
}
