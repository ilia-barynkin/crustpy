#[doc = "Register `MACHTHR` reader"]
pub type R = crate::R<MachthrSpec>;
#[doc = "Register `MACHTHR` writer"]
pub type W = crate::W<MachthrSpec>;
#[doc = "Field `HTH` reader - Hash table high"]
pub type HthR = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash table high"]
pub type HthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    pub fn hth(&self) -> HthR {
        HthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HthW<MachthrSpec> {
        HthW::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::Reg::read) this register and get [`machthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MachthrSpec;
impl crate::RegisterSpec for MachthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machthr::R`](R) reader structure"]
impl crate::Readable for MachthrSpec {}
#[doc = "`write(|w| ..)` method takes [`machthr::W`](W) writer structure"]
impl crate::Writable for MachthrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHTHR to value 0"]
impl crate::Resettable for MachthrSpec {
    const RESET_VALUE: u32 = 0;
}
