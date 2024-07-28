#[doc = "Register `CAN_FM1R` reader"]
pub type R = crate::R<CanFm1rSpec>;
#[doc = "Register `CAN_FM1R` writer"]
pub type W = crate::W<CanFm1rSpec>;
#[doc = "Field `FBM0` reader - Filter mode"]
pub type Fbm0R = crate::BitReader;
#[doc = "Field `FBM0` writer - Filter mode"]
pub type Fbm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM1` reader - Filter mode"]
pub type Fbm1R = crate::BitReader;
#[doc = "Field `FBM1` writer - Filter mode"]
pub type Fbm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM2` reader - Filter mode"]
pub type Fbm2R = crate::BitReader;
#[doc = "Field `FBM2` writer - Filter mode"]
pub type Fbm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM3` reader - Filter mode"]
pub type Fbm3R = crate::BitReader;
#[doc = "Field `FBM3` writer - Filter mode"]
pub type Fbm3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM4` reader - Filter mode"]
pub type Fbm4R = crate::BitReader;
#[doc = "Field `FBM4` writer - Filter mode"]
pub type Fbm4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM5` reader - Filter mode"]
pub type Fbm5R = crate::BitReader;
#[doc = "Field `FBM5` writer - Filter mode"]
pub type Fbm5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM6` reader - Filter mode"]
pub type Fbm6R = crate::BitReader;
#[doc = "Field `FBM6` writer - Filter mode"]
pub type Fbm6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM7` reader - Filter mode"]
pub type Fbm7R = crate::BitReader;
#[doc = "Field `FBM7` writer - Filter mode"]
pub type Fbm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM8` reader - Filter mode"]
pub type Fbm8R = crate::BitReader;
#[doc = "Field `FBM8` writer - Filter mode"]
pub type Fbm8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM9` reader - Filter mode"]
pub type Fbm9R = crate::BitReader;
#[doc = "Field `FBM9` writer - Filter mode"]
pub type Fbm9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM10` reader - Filter mode"]
pub type Fbm10R = crate::BitReader;
#[doc = "Field `FBM10` writer - Filter mode"]
pub type Fbm10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM11` reader - Filter mode"]
pub type Fbm11R = crate::BitReader;
#[doc = "Field `FBM11` writer - Filter mode"]
pub type Fbm11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM12` reader - Filter mode"]
pub type Fbm12R = crate::BitReader;
#[doc = "Field `FBM12` writer - Filter mode"]
pub type Fbm12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM13` reader - Filter mode"]
pub type Fbm13R = crate::BitReader;
#[doc = "Field `FBM13` writer - Filter mode"]
pub type Fbm13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&self) -> Fbm0R {
        Fbm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&self) -> Fbm1R {
        Fbm1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&self) -> Fbm2R {
        Fbm2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&self) -> Fbm3R {
        Fbm3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&self) -> Fbm4R {
        Fbm4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&self) -> Fbm5R {
        Fbm5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&self) -> Fbm6R {
        Fbm6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&self) -> Fbm7R {
        Fbm7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&self) -> Fbm8R {
        Fbm8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&self) -> Fbm9R {
        Fbm9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&self) -> Fbm10R {
        Fbm10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&self) -> Fbm11R {
        Fbm11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&self) -> Fbm12R {
        Fbm12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&self) -> Fbm13R {
        Fbm13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm0(&mut self) -> Fbm0W<CanFm1rSpec> {
        Fbm0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm1(&mut self) -> Fbm1W<CanFm1rSpec> {
        Fbm1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm2(&mut self) -> Fbm2W<CanFm1rSpec> {
        Fbm2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm3(&mut self) -> Fbm3W<CanFm1rSpec> {
        Fbm3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm4(&mut self) -> Fbm4W<CanFm1rSpec> {
        Fbm4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm5(&mut self) -> Fbm5W<CanFm1rSpec> {
        Fbm5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm6(&mut self) -> Fbm6W<CanFm1rSpec> {
        Fbm6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm7(&mut self) -> Fbm7W<CanFm1rSpec> {
        Fbm7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm8(&mut self) -> Fbm8W<CanFm1rSpec> {
        Fbm8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm9(&mut self) -> Fbm9W<CanFm1rSpec> {
        Fbm9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm10(&mut self) -> Fbm10W<CanFm1rSpec> {
        Fbm10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm11(&mut self) -> Fbm11W<CanFm1rSpec> {
        Fbm11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm12(&mut self) -> Fbm12W<CanFm1rSpec> {
        Fbm12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm13(&mut self) -> Fbm13W<CanFm1rSpec> {
        Fbm13W::new(self, 13)
    }
}
#[doc = "CAN_FM1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fm1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fm1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFm1rSpec;
impl crate::RegisterSpec for CanFm1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fm1r::R`](R) reader structure"]
impl crate::Readable for CanFm1rSpec {}
#[doc = "`write(|w| ..)` method takes [`can_fm1r::W`](W) writer structure"]
impl crate::Writable for CanFm1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_FM1R to value 0"]
impl crate::Resettable for CanFm1rSpec {
    const RESET_VALUE: u32 = 0;
}
