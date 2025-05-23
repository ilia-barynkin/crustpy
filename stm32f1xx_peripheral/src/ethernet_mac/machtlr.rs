#[doc = "Register `MACHTLR` reader"]
pub type R = crate::R<MachtlrSpec>;
#[doc = "Register `MACHTLR` writer"]
pub type W = crate::W<MachtlrSpec>;
#[doc = "Field `HTL` reader - Hash table low"]
pub type HtlR = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash table low"]
pub type HtlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    pub fn htl(&self) -> HtlR {
        HtlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HtlW<MachtlrSpec> {
        HtlW::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::Reg::read) this register and get [`machtlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machtlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MachtlrSpec;
impl crate::RegisterSpec for MachtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machtlr::R`](R) reader structure"]
impl crate::Readable for MachtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`machtlr::W`](W) writer structure"]
impl crate::Writable for MachtlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHTLR to value 0"]
impl crate::Resettable for MachtlrSpec {
    const RESET_VALUE: u32 = 0;
}
