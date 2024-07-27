#[doc = "Register `CRL` reader"]
pub type R = crate::R<CrlSpec>;
#[doc = "Register `CRL` writer"]
pub type W = crate::W<CrlSpec>;
#[doc = "Field `SECF` reader - Second Flag"]
pub type SecfR = crate::BitReader;
#[doc = "Field `SECF` writer - Second Flag"]
pub type SecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRF` reader - Alarm Flag"]
pub type AlrfR = crate::BitReader;
#[doc = "Field `ALRF` writer - Alarm Flag"]
pub type AlrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWF` reader - Overflow Flag"]
pub type OwfR = crate::BitReader;
#[doc = "Field `OWF` writer - Overflow Flag"]
pub type OwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Registers Synchronized Flag"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Registers Synchronized Flag"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNF` reader - Configuration Flag"]
pub type CnfR = crate::BitReader;
#[doc = "Field `CNF` writer - Configuration Flag"]
pub type CnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub type RtoffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secf(&self) -> SecfR {
        SecfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrf(&self) -> AlrfR {
        AlrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn owf(&self) -> OwfR {
        OwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CnfR {
        CnfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RtoffR {
        RtoffR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    #[must_use]
    pub fn secf(&mut self) -> SecfW<CrlSpec> {
        SecfW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrf(&mut self) -> AlrfW<CrlSpec> {
        AlrfW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn owf(&mut self) -> OwfW<CrlSpec> {
        OwfW::new(self, 2)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<CrlSpec> {
        RsfW::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self) -> CnfW<CrlSpec> {
        CnfW::new(self, 4)
    }
}
#[doc = "RTC Control Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrlSpec;
impl crate::RegisterSpec for CrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crl::R`](R) reader structure"]
impl crate::Readable for CrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crl::W`](W) writer structure"]
impl crate::Writable for CrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRL to value 0x20"]
impl crate::Resettable for CrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
