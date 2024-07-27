#[doc = "Register `DMARDLAR` reader"]
pub type R = crate::R<DmardlarSpec>;
#[doc = "Register `DMARDLAR` writer"]
pub type W = crate::W<DmardlarSpec>;
#[doc = "Field `SRL` reader - Start of receive list"]
pub type SrlR = crate::FieldReader<u32>;
#[doc = "Field `SRL` writer - Start of receive list"]
pub type SrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn srl(&self) -> SrlR {
        SrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SrlW<DmardlarSpec> {
        SrlW::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmardlarSpec;
impl crate::RegisterSpec for DmardlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardlar::R`](R) reader structure"]
impl crate::Readable for DmardlarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmardlar::W`](W) writer structure"]
impl crate::Writable for DmardlarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARDLAR to value 0"]
impl crate::Resettable for DmardlarSpec {
    const RESET_VALUE: u32 = 0;
}
