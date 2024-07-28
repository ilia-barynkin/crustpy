#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CMDINDEX` reader - CMDINDEX"]
pub type CmdindexR = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - CMDINDEX"]
pub type CmdindexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WAITRESP` reader - WAITRESP"]
pub type WaitrespR = crate::FieldReader;
#[doc = "Field `WAITRESP` writer - WAITRESP"]
pub type WaitrespW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITINT` reader - WAITINT"]
pub type WaitintR = crate::BitReader;
#[doc = "Field `WAITINT` writer - WAITINT"]
pub type WaitintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPEND` reader - WAITPEND"]
pub type WaitpendR = crate::BitReader;
#[doc = "Field `WAITPEND` writer - WAITPEND"]
pub type WaitpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSMEN` reader - CPSMEN"]
pub type CpsmenR = crate::BitReader;
#[doc = "Field `CPSMEN` writer - CPSMEN"]
pub type CpsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOSuspend` reader - SDIOSuspend"]
pub type SdiosuspendR = crate::BitReader;
#[doc = "Field `SDIOSuspend` writer - SDIOSuspend"]
pub type SdiosuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCMDcompl` reader - ENCMDcompl"]
pub type EncmdcomplR = crate::BitReader;
#[doc = "Field `ENCMDcompl` writer - ENCMDcompl"]
pub type EncmdcomplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nIEN` reader - nIEN"]
pub type NIenR = crate::BitReader;
#[doc = "Field `nIEN` writer - nIEN"]
pub type NIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE_ATACMD` reader - CE_ATACMD"]
pub type CeAtacmdR = crate::BitReader;
#[doc = "Field `CE_ATACMD` writer - CE_ATACMD"]
pub type CeAtacmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CmdindexR {
        CmdindexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WaitrespR {
        WaitrespR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&self) -> WaitintR {
        WaitintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WaitpendR {
        WaitpendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CpsmenR {
        CpsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SdiosuspendR {
        SdiosuspendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ENCMDcompl"]
    #[inline(always)]
    pub fn encmdcompl(&self) -> EncmdcomplR {
        EncmdcomplR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - nIEN"]
    #[inline(always)]
    pub fn n_ien(&self) -> NIenR {
        NIenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CE_ATACMD"]
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CeAtacmdR {
        CeAtacmdR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CmdindexW<CmdSpec> {
        CmdindexW::new(self, 0)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WaitrespW<CmdSpec> {
        WaitrespW::new(self, 6)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WaitintW<CmdSpec> {
        WaitintW::new(self, 8)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WaitpendW<CmdSpec> {
        WaitpendW::new(self, 9)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CpsmenW<CmdSpec> {
        CpsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosuspend(&mut self) -> SdiosuspendW<CmdSpec> {
        SdiosuspendW::new(self, 11)
    }
    #[doc = "Bit 12 - ENCMDcompl"]
    #[inline(always)]
    #[must_use]
    pub fn encmdcompl(&mut self) -> EncmdcomplW<CmdSpec> {
        EncmdcomplW::new(self, 12)
    }
    #[doc = "Bit 13 - nIEN"]
    #[inline(always)]
    #[must_use]
    pub fn n_ien(&mut self) -> NIenW<CmdSpec> {
        NIenW::new(self, 13)
    }
    #[doc = "Bit 14 - CE_ATACMD"]
    #[inline(always)]
    #[must_use]
    pub fn ce_atacmd(&mut self) -> CeAtacmdW<CmdSpec> {
        CeAtacmdW::new(self, 14)
    }
}
#[doc = "SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
