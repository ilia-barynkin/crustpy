#[doc = "Register `CAN_IER` reader"]
pub type R = crate::R<CanIerSpec>;
#[doc = "Register `CAN_IER` writer"]
pub type W = crate::W<CanIerSpec>;
#[doc = "Field `TMEIE` reader - TMEIE"]
pub type TmeieR = crate::BitReader;
#[doc = "Field `TMEIE` writer - TMEIE"]
pub type TmeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMPIE0` reader - FMPIE0"]
pub type Fmpie0R = crate::BitReader;
#[doc = "Field `FMPIE0` writer - FMPIE0"]
pub type Fmpie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFIE0` reader - FFIE0"]
pub type Ffie0R = crate::BitReader;
#[doc = "Field `FFIE0` writer - FFIE0"]
pub type Ffie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVIE0` reader - FOVIE0"]
pub type Fovie0R = crate::BitReader;
#[doc = "Field `FOVIE0` writer - FOVIE0"]
pub type Fovie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMPIE1` reader - FMPIE1"]
pub type Fmpie1R = crate::BitReader;
#[doc = "Field `FMPIE1` writer - FMPIE1"]
pub type Fmpie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFIE1` reader - FFIE1"]
pub type Ffie1R = crate::BitReader;
#[doc = "Field `FFIE1` writer - FFIE1"]
pub type Ffie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVIE1` reader - FOVIE1"]
pub type Fovie1R = crate::BitReader;
#[doc = "Field `FOVIE1` writer - FOVIE1"]
pub type Fovie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWGIE` reader - EWGIE"]
pub type EwgieR = crate::BitReader;
#[doc = "Field `EWGIE` writer - EWGIE"]
pub type EwgieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPVIE` reader - EPVIE"]
pub type EpvieR = crate::BitReader;
#[doc = "Field `EPVIE` writer - EPVIE"]
pub type EpvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFIE` reader - BOFIE"]
pub type BofieR = crate::BitReader;
#[doc = "Field `BOFIE` writer - BOFIE"]
pub type BofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LECIE` reader - LECIE"]
pub type LecieR = crate::BitReader;
#[doc = "Field `LECIE` writer - LECIE"]
pub type LecieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - ERRIE"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - ERRIE"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUIE` reader - WKUIE"]
pub type WkuieR = crate::BitReader;
#[doc = "Field `WKUIE` writer - WKUIE"]
pub type WkuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLKIE` reader - SLKIE"]
pub type SlkieR = crate::BitReader;
#[doc = "Field `SLKIE` writer - SLKIE"]
pub type SlkieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&self) -> TmeieR {
        TmeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&self) -> Fmpie0R {
        Fmpie0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&self) -> Ffie0R {
        Ffie0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&self) -> Fovie0R {
        Fovie0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&self) -> Fmpie1R {
        Fmpie1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&self) -> Ffie1R {
        Ffie1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&self) -> Fovie1R {
        Fovie1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&self) -> EwgieR {
        EwgieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&self) -> EpvieR {
        EpvieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&self) -> BofieR {
        BofieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&self) -> LecieR {
        LecieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&self) -> WkuieR {
        WkuieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&self) -> SlkieR {
        SlkieR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TmeieW<CanIerSpec> {
        TmeieW::new(self, 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie0(&mut self) -> Fmpie0W<CanIerSpec> {
        Fmpie0W::new(self, 1)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    #[must_use]
    pub fn ffie0(&mut self) -> Ffie0W<CanIerSpec> {
        Ffie0W::new(self, 2)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    #[must_use]
    pub fn fovie0(&mut self) -> Fovie0W<CanIerSpec> {
        Fovie0W::new(self, 3)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie1(&mut self) -> Fmpie1W<CanIerSpec> {
        Fmpie1W::new(self, 4)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    #[must_use]
    pub fn ffie1(&mut self) -> Ffie1W<CanIerSpec> {
        Ffie1W::new(self, 5)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    #[must_use]
    pub fn fovie1(&mut self) -> Fovie1W<CanIerSpec> {
        Fovie1W::new(self, 6)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    #[must_use]
    pub fn ewgie(&mut self) -> EwgieW<CanIerSpec> {
        EwgieW::new(self, 8)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    #[must_use]
    pub fn epvie(&mut self) -> EpvieW<CanIerSpec> {
        EpvieW::new(self, 9)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn bofie(&mut self) -> BofieW<CanIerSpec> {
        BofieW::new(self, 10)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LecieW<CanIerSpec> {
        LecieW::new(self, 11)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<CanIerSpec> {
        ErrieW::new(self, 15)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    #[must_use]
    pub fn wkuie(&mut self) -> WkuieW<CanIerSpec> {
        WkuieW::new(self, 16)
    }
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    #[must_use]
    pub fn slkie(&mut self) -> SlkieW<CanIerSpec> {
        SlkieW::new(self, 17)
    }
}
#[doc = "CAN_IER\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanIerSpec;
impl crate::RegisterSpec for CanIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ier::R`](R) reader structure"]
impl crate::Readable for CanIerSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ier::W`](W) writer structure"]
impl crate::Writable for CanIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_IER to value 0"]
impl crate::Resettable for CanIerSpec {
    const RESET_VALUE: u32 = 0;
}
