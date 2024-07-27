#[doc = "Register `CAN_FFA1R` reader"]
pub type R = crate::R<CanFfa1rSpec>;
#[doc = "Register `CAN_FFA1R` writer"]
pub type W = crate::W<CanFfa1rSpec>;
#[doc = "Field `FFA0` reader - Filter FIFO assignment for filter 0"]
pub type Ffa0R = crate::BitReader;
#[doc = "Field `FFA0` writer - Filter FIFO assignment for filter 0"]
pub type Ffa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA1` reader - Filter FIFO assignment for filter 1"]
pub type Ffa1R = crate::BitReader;
#[doc = "Field `FFA1` writer - Filter FIFO assignment for filter 1"]
pub type Ffa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA2` reader - Filter FIFO assignment for filter 2"]
pub type Ffa2R = crate::BitReader;
#[doc = "Field `FFA2` writer - Filter FIFO assignment for filter 2"]
pub type Ffa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA3` reader - Filter FIFO assignment for filter 3"]
pub type Ffa3R = crate::BitReader;
#[doc = "Field `FFA3` writer - Filter FIFO assignment for filter 3"]
pub type Ffa3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA4` reader - Filter FIFO assignment for filter 4"]
pub type Ffa4R = crate::BitReader;
#[doc = "Field `FFA4` writer - Filter FIFO assignment for filter 4"]
pub type Ffa4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA5` reader - Filter FIFO assignment for filter 5"]
pub type Ffa5R = crate::BitReader;
#[doc = "Field `FFA5` writer - Filter FIFO assignment for filter 5"]
pub type Ffa5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA6` reader - Filter FIFO assignment for filter 6"]
pub type Ffa6R = crate::BitReader;
#[doc = "Field `FFA6` writer - Filter FIFO assignment for filter 6"]
pub type Ffa6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA7` reader - Filter FIFO assignment for filter 7"]
pub type Ffa7R = crate::BitReader;
#[doc = "Field `FFA7` writer - Filter FIFO assignment for filter 7"]
pub type Ffa7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA8` reader - Filter FIFO assignment for filter 8"]
pub type Ffa8R = crate::BitReader;
#[doc = "Field `FFA8` writer - Filter FIFO assignment for filter 8"]
pub type Ffa8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA9` reader - Filter FIFO assignment for filter 9"]
pub type Ffa9R = crate::BitReader;
#[doc = "Field `FFA9` writer - Filter FIFO assignment for filter 9"]
pub type Ffa9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA10` reader - Filter FIFO assignment for filter 10"]
pub type Ffa10R = crate::BitReader;
#[doc = "Field `FFA10` writer - Filter FIFO assignment for filter 10"]
pub type Ffa10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA11` reader - Filter FIFO assignment for filter 11"]
pub type Ffa11R = crate::BitReader;
#[doc = "Field `FFA11` writer - Filter FIFO assignment for filter 11"]
pub type Ffa11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA12` reader - Filter FIFO assignment for filter 12"]
pub type Ffa12R = crate::BitReader;
#[doc = "Field `FFA12` writer - Filter FIFO assignment for filter 12"]
pub type Ffa12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFA13` reader - Filter FIFO assignment for filter 13"]
pub type Ffa13R = crate::BitReader;
#[doc = "Field `FFA13` writer - Filter FIFO assignment for filter 13"]
pub type Ffa13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> Ffa0R {
        Ffa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> Ffa1R {
        Ffa1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> Ffa2R {
        Ffa2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> Ffa3R {
        Ffa3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> Ffa4R {
        Ffa4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> Ffa5R {
        Ffa5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> Ffa6R {
        Ffa6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> Ffa7R {
        Ffa7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> Ffa8R {
        Ffa8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> Ffa9R {
        Ffa9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> Ffa10R {
        Ffa10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> Ffa11R {
        Ffa11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> Ffa12R {
        Ffa12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> Ffa13R {
        Ffa13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    #[must_use]
    pub fn ffa0(&mut self) -> Ffa0W<CanFfa1rSpec> {
        Ffa0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    #[must_use]
    pub fn ffa1(&mut self) -> Ffa1W<CanFfa1rSpec> {
        Ffa1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    #[must_use]
    pub fn ffa2(&mut self) -> Ffa2W<CanFfa1rSpec> {
        Ffa2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    #[must_use]
    pub fn ffa3(&mut self) -> Ffa3W<CanFfa1rSpec> {
        Ffa3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    #[must_use]
    pub fn ffa4(&mut self) -> Ffa4W<CanFfa1rSpec> {
        Ffa4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    #[must_use]
    pub fn ffa5(&mut self) -> Ffa5W<CanFfa1rSpec> {
        Ffa5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    #[must_use]
    pub fn ffa6(&mut self) -> Ffa6W<CanFfa1rSpec> {
        Ffa6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    #[must_use]
    pub fn ffa7(&mut self) -> Ffa7W<CanFfa1rSpec> {
        Ffa7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    #[must_use]
    pub fn ffa8(&mut self) -> Ffa8W<CanFfa1rSpec> {
        Ffa8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    #[must_use]
    pub fn ffa9(&mut self) -> Ffa9W<CanFfa1rSpec> {
        Ffa9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    #[must_use]
    pub fn ffa10(&mut self) -> Ffa10W<CanFfa1rSpec> {
        Ffa10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    #[must_use]
    pub fn ffa11(&mut self) -> Ffa11W<CanFfa1rSpec> {
        Ffa11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    #[must_use]
    pub fn ffa12(&mut self) -> Ffa12W<CanFfa1rSpec> {
        Ffa12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    #[must_use]
    pub fn ffa13(&mut self) -> Ffa13W<CanFfa1rSpec> {
        Ffa13W::new(self, 13)
    }
}
#[doc = "CAN_FFA1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ffa1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ffa1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFfa1rSpec;
impl crate::RegisterSpec for CanFfa1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ffa1r::R`](R) reader structure"]
impl crate::Readable for CanFfa1rSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ffa1r::W`](W) writer structure"]
impl crate::Writable for CanFfa1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_FFA1R to value 0"]
impl crate::Resettable for CanFfa1rSpec {
    const RESET_VALUE: u32 = 0;
}
