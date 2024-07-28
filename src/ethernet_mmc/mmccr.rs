#[doc = "Register `MMCCR` reader"]
pub type R = crate::R<MmccrSpec>;
#[doc = "Register `MMCCR` writer"]
pub type W = crate::W<MmccrSpec>;
#[doc = "Field `CR` reader - Counter reset"]
pub type CrR = crate::BitReader;
#[doc = "Field `CR` writer - Counter reset"]
pub type CrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROR` reader - Reset on read"]
pub type RorR = crate::BitReader;
#[doc = "Field `ROR` writer - Reset on read"]
pub type RorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub type McfR = crate::BitReader;
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub type McfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> RorR {
        RorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> McfR {
        McfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<MmccrSpec> {
        CrW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CsrW<MmccrSpec> {
        CsrW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> RorW<MmccrSpec> {
        RorW::new(self, 2)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> McfW<MmccrSpec> {
        McfW::new(self, 31)
    }
}
#[doc = "Ethernet MMC control register (ETH_MMCCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmccrSpec;
impl crate::RegisterSpec for MmccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmccr::R`](R) reader structure"]
impl crate::Readable for MmccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmccr::W`](W) writer structure"]
impl crate::Writable for MmccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MmccrSpec {
    const RESET_VALUE: u32 = 0;
}
