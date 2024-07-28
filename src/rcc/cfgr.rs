#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - System clock Switch"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System clock Switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type Ppre1R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type Ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB High speed prescaler (APB2)"]
pub type Ppre2R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB High speed prescaler (APB2)"]
pub type Ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADCPRE` reader - ADC prescaler"]
pub type AdcpreR = crate::FieldReader;
#[doc = "Field `ADCPRE` writer - ADC prescaler"]
pub type AdcpreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PllxtpreR = crate::BitReader;
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PllxtpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub type PllmulR = crate::FieldReader;
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub type PllmulW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OTGFSPRE` reader - USB OTG FS prescaler"]
pub type OtgfspreR = crate::BitReader;
#[doc = "Field `OTGFSPRE` writer - USB OTG FS prescaler"]
pub type OtgfspreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type McoR = crate::FieldReader;
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type McoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> Ppre1R {
        Ppre1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> Ppre2R {
        Ppre2R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> AdcpreR {
        AdcpreR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PllxtpreR {
        PllxtpreR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PllmulR {
        PllmulR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - USB OTG FS prescaler"]
    #[inline(always)]
    pub fn otgfspre(&self) -> OtgfspreR {
        OtgfspreR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> McoR {
        McoR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HpreW<CfgrSpec> {
        HpreW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> Ppre1W<CfgrSpec> {
        Ppre1W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> Ppre2W<CfgrSpec> {
        Ppre2W::new(self, 11)
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre(&mut self) -> AdcpreW<CfgrSpec> {
        AdcpreW::new(self, 14)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PllsrcW<CfgrSpec> {
        PllsrcW::new(self, 16)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PllxtpreW<CfgrSpec> {
        PllxtpreW::new(self, 17)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PllmulW<CfgrSpec> {
        PllmulW::new(self, 18)
    }
    #[doc = "Bit 22 - USB OTG FS prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn otgfspre(&mut self) -> OtgfspreW<CfgrSpec> {
        OtgfspreW::new(self, 22)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> McoW<CfgrSpec> {
        McoW::new(self, 24)
    }
}
#[doc = "Clock configuration register (RCC_CFGR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
