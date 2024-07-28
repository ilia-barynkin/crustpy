#[doc = "Register `DMACHTBAR` reader"]
pub type R = crate::R<DmachtbarSpec>;
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HtbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HtbapR {
        HtbapR::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmachtbar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachtbarSpec;
impl crate::RegisterSpec for DmachtbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachtbar::R`](R) reader structure"]
impl crate::Readable for DmachtbarSpec {}
#[doc = "`reset()` method sets DMACHTBAR to value 0"]
impl crate::Resettable for DmachtbarSpec {
    const RESET_VALUE: u32 = 0;
}
