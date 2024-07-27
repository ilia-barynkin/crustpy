#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `CTE` writer - Clear Tamper event"]
pub type CteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTI` writer - Clear Tamper Interrupt"]
pub type CtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - Tamper Pin interrupt enable"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - Tamper Pin interrupt enable"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF` reader - Tamper Event Flag"]
pub type TefR = crate::BitReader;
#[doc = "Field `TIF` reader - Tamper Interrupt Flag"]
pub type TifR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper Event Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Tamper event"]
    #[inline(always)]
    #[must_use]
    pub fn cte(&mut self) -> CteW<CsrSpec> {
        CteW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Tamper Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cti(&mut self) -> CtiW<CsrSpec> {
        CtiW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TpieW<CsrSpec> {
        TpieW::new(self, 2)
    }
}
#[doc = "BKP_CSR control/status register (BKP_CSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
