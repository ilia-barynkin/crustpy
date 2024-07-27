#[doc = "Register `CAN_RF0R` reader"]
pub type R = crate::R<CanRf0rSpec>;
#[doc = "Register `CAN_RF0R` writer"]
pub type W = crate::W<CanRf0rSpec>;
#[doc = "Field `FMP0` reader - FMP0"]
pub type Fmp0R = crate::FieldReader;
#[doc = "Field `FULL0` reader - FULL0"]
pub type Full0R = crate::BitReader;
#[doc = "Field `FULL0` writer - FULL0"]
pub type Full0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVR0` reader - FOVR0"]
pub type Fovr0R = crate::BitReader;
#[doc = "Field `FOVR0` writer - FOVR0"]
pub type Fovr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOM0` reader - RFOM0"]
pub type Rfom0R = crate::BitReader;
#[doc = "Field `RFOM0` writer - RFOM0"]
pub type Rfom0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp0(&self) -> Fmp0R {
        Fmp0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&self) -> Full0R {
        Full0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&self) -> Fovr0R {
        Fovr0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&self) -> Rfom0R {
        Rfom0R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    #[must_use]
    pub fn full0(&mut self) -> Full0W<CanRf0rSpec> {
        Full0W::new(self, 3)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    #[must_use]
    pub fn fovr0(&mut self) -> Fovr0W<CanRf0rSpec> {
        Fovr0W::new(self, 4)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    #[must_use]
    pub fn rfom0(&mut self) -> Rfom0W<CanRf0rSpec> {
        Rfom0W::new(self, 5)
    }
}
#[doc = "CAN_RF0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rf0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_rf0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRf0rSpec;
impl crate::RegisterSpec for CanRf0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rf0r::R`](R) reader structure"]
impl crate::Readable for CanRf0rSpec {}
#[doc = "`write(|w| ..)` method takes [`can_rf0r::W`](W) writer structure"]
impl crate::Writable for CanRf0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_RF0R to value 0"]
impl crate::Resettable for CanRf0rSpec {
    const RESET_VALUE: u32 = 0;
}
