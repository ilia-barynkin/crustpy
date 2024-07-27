#[doc = "Register `MMCRIMR` reader"]
pub type R = crate::R<MmcrimrSpec>;
#[doc = "Register `MMCRIMR` writer"]
pub type W = crate::W<MmcrimrSpec>;
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub type RfcemR = crate::BitReader;
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub type RfcemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub type RfaemR = crate::BitReader;
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub type RfaemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUFM` reader - Received good unicast frames mask"]
pub type RgufmR = crate::BitReader;
#[doc = "Field `RGUFM` writer - Received good unicast frames mask"]
pub type RgufmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RfcemR {
        RfcemR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RfaemR {
        RfaemR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RgufmR {
        RgufmR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfcem(&mut self) -> RfcemW<MmcrimrSpec> {
        RfcemW::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfaem(&mut self) -> RfaemW<MmcrimrSpec> {
        RfaemW::new(self, 6)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    #[must_use]
    pub fn rgufm(&mut self) -> RgufmW<MmcrimrSpec> {
        RgufmW::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrimrSpec;
impl crate::RegisterSpec for MmcrimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrimr::R`](R) reader structure"]
impl crate::Readable for MmcrimrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcrimr::W`](W) writer structure"]
impl crate::Writable for MmcrimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCRIMR to value 0"]
impl crate::Resettable for MmcrimrSpec {
    const RESET_VALUE: u32 = 0;
}
