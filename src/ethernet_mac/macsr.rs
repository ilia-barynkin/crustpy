#[doc = "Register `MACSR` reader"]
pub type R = crate::R<MacsrSpec>;
#[doc = "Register `MACSR` writer"]
pub type W = crate::W<MacsrSpec>;
#[doc = "Field `PMTS` reader - PMT status"]
pub type PmtsR = crate::BitReader;
#[doc = "Field `PMTS` writer - PMT status"]
pub type PmtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMCS` reader - MMC status"]
pub type MmcsR = crate::BitReader;
#[doc = "Field `MMCS` writer - MMC status"]
pub type MmcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMCRS` reader - MMC receive status"]
pub type MmcrsR = crate::BitReader;
#[doc = "Field `MMCRS` writer - MMC receive status"]
pub type MmcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMCTS` reader - MMC transmit status"]
pub type MmctsR = crate::BitReader;
#[doc = "Field `MMCTS` writer - MMC transmit status"]
pub type MmctsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub type TstsR = crate::BitReader;
#[doc = "Field `TSTS` writer - Time stamp trigger status"]
pub type TstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PmtsR {
        PmtsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MmcsR {
        MmcsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC receive status"]
    #[inline(always)]
    pub fn mmcrs(&self) -> MmcrsR {
        MmcrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC transmit status"]
    #[inline(always)]
    pub fn mmcts(&self) -> MmctsR {
        MmctsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TstsR {
        TstsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT status"]
    #[inline(always)]
    #[must_use]
    pub fn pmts(&mut self) -> PmtsW<MacsrSpec> {
        PmtsW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC status"]
    #[inline(always)]
    #[must_use]
    pub fn mmcs(&mut self) -> MmcsW<MacsrSpec> {
        MmcsW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC receive status"]
    #[inline(always)]
    #[must_use]
    pub fn mmcrs(&mut self) -> MmcrsW<MacsrSpec> {
        MmcrsW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn mmcts(&mut self) -> MmctsW<MacsrSpec> {
        MmctsW::new(self, 6)
    }
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tsts(&mut self) -> TstsW<MacsrSpec> {
        TstsW::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacsrSpec;
impl crate::RegisterSpec for MacsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macsr::R`](R) reader structure"]
impl crate::Readable for MacsrSpec {}
#[doc = "`write(|w| ..)` method takes [`macsr::W`](W) writer structure"]
impl crate::Writable for MacsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSR to value 0"]
impl crate::Resettable for MacsrSpec {
    const RESET_VALUE: u32 = 0;
}
