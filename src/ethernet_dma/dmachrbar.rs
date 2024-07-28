#[doc = "Register `DMACHRBAR` reader"]
pub type R = crate::R<DmachrbarSpec>;
#[doc = "Field `HRBAP` reader - Host receive buffer address pointer"]
pub type HrbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive buffer address pointer"]
    #[inline(always)]
    pub fn hrbap(&self) -> HrbapR {
        HrbapR::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmachrbar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachrbarSpec;
impl crate::RegisterSpec for DmachrbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachrbar::R`](R) reader structure"]
impl crate::Readable for DmachrbarSpec {}
#[doc = "`reset()` method sets DMACHRBAR to value 0"]
impl crate::Resettable for DmachrbarSpec {
    const RESET_VALUE: u32 = 0;
}
