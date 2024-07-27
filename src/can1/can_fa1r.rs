#[doc = "Register `CAN_FA1R` reader"]
pub type R = crate::R<CanFa1rSpec>;
#[doc = "Register `CAN_FA1R` writer"]
pub type W = crate::W<CanFa1rSpec>;
#[doc = "Field `FACT0` reader - Filter active"]
pub type Fact0R = crate::BitReader;
#[doc = "Field `FACT0` writer - Filter active"]
pub type Fact0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT1` reader - Filter active"]
pub type Fact1R = crate::BitReader;
#[doc = "Field `FACT1` writer - Filter active"]
pub type Fact1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT2` reader - Filter active"]
pub type Fact2R = crate::BitReader;
#[doc = "Field `FACT2` writer - Filter active"]
pub type Fact2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT3` reader - Filter active"]
pub type Fact3R = crate::BitReader;
#[doc = "Field `FACT3` writer - Filter active"]
pub type Fact3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT4` reader - Filter active"]
pub type Fact4R = crate::BitReader;
#[doc = "Field `FACT4` writer - Filter active"]
pub type Fact4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT5` reader - Filter active"]
pub type Fact5R = crate::BitReader;
#[doc = "Field `FACT5` writer - Filter active"]
pub type Fact5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT6` reader - Filter active"]
pub type Fact6R = crate::BitReader;
#[doc = "Field `FACT6` writer - Filter active"]
pub type Fact6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT7` reader - Filter active"]
pub type Fact7R = crate::BitReader;
#[doc = "Field `FACT7` writer - Filter active"]
pub type Fact7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT8` reader - Filter active"]
pub type Fact8R = crate::BitReader;
#[doc = "Field `FACT8` writer - Filter active"]
pub type Fact8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT9` reader - Filter active"]
pub type Fact9R = crate::BitReader;
#[doc = "Field `FACT9` writer - Filter active"]
pub type Fact9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT10` reader - Filter active"]
pub type Fact10R = crate::BitReader;
#[doc = "Field `FACT10` writer - Filter active"]
pub type Fact10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT11` reader - Filter active"]
pub type Fact11R = crate::BitReader;
#[doc = "Field `FACT11` writer - Filter active"]
pub type Fact11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT12` reader - Filter active"]
pub type Fact12R = crate::BitReader;
#[doc = "Field `FACT12` writer - Filter active"]
pub type Fact12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACT13` reader - Filter active"]
pub type Fact13R = crate::BitReader;
#[doc = "Field `FACT13` writer - Filter active"]
pub type Fact13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&self) -> Fact0R {
        Fact0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&self) -> Fact1R {
        Fact1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&self) -> Fact2R {
        Fact2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&self) -> Fact3R {
        Fact3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&self) -> Fact4R {
        Fact4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&self) -> Fact5R {
        Fact5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&self) -> Fact6R {
        Fact6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&self) -> Fact7R {
        Fact7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&self) -> Fact8R {
        Fact8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&self) -> Fact9R {
        Fact9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&self) -> Fact10R {
        Fact10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&self) -> Fact11R {
        Fact11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&self) -> Fact12R {
        Fact12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&self) -> Fact13R {
        Fact13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact0(&mut self) -> Fact0W<CanFa1rSpec> {
        Fact0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact1(&mut self) -> Fact1W<CanFa1rSpec> {
        Fact1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact2(&mut self) -> Fact2W<CanFa1rSpec> {
        Fact2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact3(&mut self) -> Fact3W<CanFa1rSpec> {
        Fact3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact4(&mut self) -> Fact4W<CanFa1rSpec> {
        Fact4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact5(&mut self) -> Fact5W<CanFa1rSpec> {
        Fact5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact6(&mut self) -> Fact6W<CanFa1rSpec> {
        Fact6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact7(&mut self) -> Fact7W<CanFa1rSpec> {
        Fact7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact8(&mut self) -> Fact8W<CanFa1rSpec> {
        Fact8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact9(&mut self) -> Fact9W<CanFa1rSpec> {
        Fact9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact10(&mut self) -> Fact10W<CanFa1rSpec> {
        Fact10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact11(&mut self) -> Fact11W<CanFa1rSpec> {
        Fact11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact12(&mut self) -> Fact12W<CanFa1rSpec> {
        Fact12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact13(&mut self) -> Fact13W<CanFa1rSpec> {
        Fact13W::new(self, 13)
    }
}
#[doc = "CAN_FA1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fa1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fa1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFa1rSpec;
impl crate::RegisterSpec for CanFa1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fa1r::R`](R) reader structure"]
impl crate::Readable for CanFa1rSpec {}
#[doc = "`write(|w| ..)` method takes [`can_fa1r::W`](W) writer structure"]
impl crate::Writable for CanFa1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_FA1R to value 0"]
impl crate::Resettable for CanFa1rSpec {
    const RESET_VALUE: u32 = 0;
}
