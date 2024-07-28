#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `DATA` reader - Regular data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `ADC2DATA` reader - ADC2 data"]
pub type Adc2dataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC2 data"]
    #[inline(always)]
    pub fn adc2data(&self) -> Adc2dataR {
        Adc2dataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "regular data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
