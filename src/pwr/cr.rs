#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LPDS` reader - Low Power Deep Sleep"]
pub type LpdsR = crate::BitReader;
#[doc = "Field `LPDS` writer - Low Power Deep Sleep"]
pub type LpdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS` reader - Power Down Deep Sleep"]
pub type PddsR = crate::BitReader;
#[doc = "Field `PDDS` writer - Power Down Deep Sleep"]
pub type PddsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF` reader - Clear Wake-up Flag"]
pub type CwufR = crate::BitReader;
#[doc = "Field `CWUF` writer - Clear Wake-up Flag"]
pub type CwufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` reader - Clear STANDBY Flag"]
pub type CsbfR = crate::BitReader;
#[doc = "Field `CSBF` writer - Clear STANDBY Flag"]
pub type CsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Power Voltage Detector Enable"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - Power Voltage Detector Enable"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD Level Selection"]
pub type PlsR = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD Level Selection"]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBP` reader - Disable Backup Domain write protection"]
pub type DbpR = crate::BitReader;
#[doc = "Field `DBP` writer - Disable Backup Domain write protection"]
pub type DbpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LpdsR {
        LpdsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PddsR {
        PddsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CwufR {
        CwufR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CsbfR {
        CsbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DbpR {
        DbpR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LpdsW<CrSpec> {
        LpdsW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PddsW<CrSpec> {
        PddsW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CwufW<CrSpec> {
        CwufW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CsbfW<CrSpec> {
        CsbfW::new(self, 3)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PvdeW<CrSpec> {
        PvdeW::new(self, 4)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PlsW<CrSpec> {
        PlsW::new(self, 5)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DbpW<CrSpec> {
        DbpW::new(self, 8)
    }
}
#[doc = "Power control register (PWR_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
