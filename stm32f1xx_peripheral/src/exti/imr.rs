#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `MR0` reader - Interrupt Mask on line 0"]
pub type Mr0R = crate::BitReader;
#[doc = "Field `MR0` writer - Interrupt Mask on line 0"]
pub type Mr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1` reader - Interrupt Mask on line 1"]
pub type Mr1R = crate::BitReader;
#[doc = "Field `MR1` writer - Interrupt Mask on line 1"]
pub type Mr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2` reader - Interrupt Mask on line 2"]
pub type Mr2R = crate::BitReader;
#[doc = "Field `MR2` writer - Interrupt Mask on line 2"]
pub type Mr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3` reader - Interrupt Mask on line 3"]
pub type Mr3R = crate::BitReader;
#[doc = "Field `MR3` writer - Interrupt Mask on line 3"]
pub type Mr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR4` reader - Interrupt Mask on line 4"]
pub type Mr4R = crate::BitReader;
#[doc = "Field `MR4` writer - Interrupt Mask on line 4"]
pub type Mr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR5` reader - Interrupt Mask on line 5"]
pub type Mr5R = crate::BitReader;
#[doc = "Field `MR5` writer - Interrupt Mask on line 5"]
pub type Mr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR6` reader - Interrupt Mask on line 6"]
pub type Mr6R = crate::BitReader;
#[doc = "Field `MR6` writer - Interrupt Mask on line 6"]
pub type Mr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR7` reader - Interrupt Mask on line 7"]
pub type Mr7R = crate::BitReader;
#[doc = "Field `MR7` writer - Interrupt Mask on line 7"]
pub type Mr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR8` reader - Interrupt Mask on line 8"]
pub type Mr8R = crate::BitReader;
#[doc = "Field `MR8` writer - Interrupt Mask on line 8"]
pub type Mr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR9` reader - Interrupt Mask on line 9"]
pub type Mr9R = crate::BitReader;
#[doc = "Field `MR9` writer - Interrupt Mask on line 9"]
pub type Mr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR10` reader - Interrupt Mask on line 10"]
pub type Mr10R = crate::BitReader;
#[doc = "Field `MR10` writer - Interrupt Mask on line 10"]
pub type Mr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR11` reader - Interrupt Mask on line 11"]
pub type Mr11R = crate::BitReader;
#[doc = "Field `MR11` writer - Interrupt Mask on line 11"]
pub type Mr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR12` reader - Interrupt Mask on line 12"]
pub type Mr12R = crate::BitReader;
#[doc = "Field `MR12` writer - Interrupt Mask on line 12"]
pub type Mr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR13` reader - Interrupt Mask on line 13"]
pub type Mr13R = crate::BitReader;
#[doc = "Field `MR13` writer - Interrupt Mask on line 13"]
pub type Mr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR14` reader - Interrupt Mask on line 14"]
pub type Mr14R = crate::BitReader;
#[doc = "Field `MR14` writer - Interrupt Mask on line 14"]
pub type Mr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR15` reader - Interrupt Mask on line 15"]
pub type Mr15R = crate::BitReader;
#[doc = "Field `MR15` writer - Interrupt Mask on line 15"]
pub type Mr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR16` reader - Interrupt Mask on line 16"]
pub type Mr16R = crate::BitReader;
#[doc = "Field `MR16` writer - Interrupt Mask on line 16"]
pub type Mr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR17` reader - Interrupt Mask on line 17"]
pub type Mr17R = crate::BitReader;
#[doc = "Field `MR17` writer - Interrupt Mask on line 17"]
pub type Mr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR18` reader - Interrupt Mask on line 18"]
pub type Mr18R = crate::BitReader;
#[doc = "Field `MR18` writer - Interrupt Mask on line 18"]
pub type Mr18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn mr0(&self) -> Mr0R {
        Mr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn mr1(&self) -> Mr1R {
        Mr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn mr2(&self) -> Mr2R {
        Mr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn mr3(&self) -> Mr3R {
        Mr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn mr4(&self) -> Mr4R {
        Mr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn mr5(&self) -> Mr5R {
        Mr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn mr6(&self) -> Mr6R {
        Mr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn mr7(&self) -> Mr7R {
        Mr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn mr8(&self) -> Mr8R {
        Mr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn mr9(&self) -> Mr9R {
        Mr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn mr10(&self) -> Mr10R {
        Mr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn mr11(&self) -> Mr11R {
        Mr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn mr12(&self) -> Mr12R {
        Mr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn mr13(&self) -> Mr13R {
        Mr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn mr14(&self) -> Mr14R {
        Mr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn mr15(&self) -> Mr15R {
        Mr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn mr16(&self) -> Mr16R {
        Mr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn mr17(&self) -> Mr17R {
        Mr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn mr18(&self) -> Mr18R {
        Mr18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> Mr0W<ImrSpec> {
        Mr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> Mr1W<ImrSpec> {
        Mr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> Mr2W<ImrSpec> {
        Mr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> Mr3W<ImrSpec> {
        Mr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> Mr4W<ImrSpec> {
        Mr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> Mr5W<ImrSpec> {
        Mr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> Mr6W<ImrSpec> {
        Mr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> Mr7W<ImrSpec> {
        Mr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> Mr8W<ImrSpec> {
        Mr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> Mr9W<ImrSpec> {
        Mr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> Mr10W<ImrSpec> {
        Mr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> Mr11W<ImrSpec> {
        Mr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> Mr12W<ImrSpec> {
        Mr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> Mr13W<ImrSpec> {
        Mr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> Mr14W<ImrSpec> {
        Mr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> Mr15W<ImrSpec> {
        Mr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn mr16(&mut self) -> Mr16W<ImrSpec> {
        Mr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn mr17(&mut self) -> Mr17W<ImrSpec> {
        Mr17W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn mr18(&mut self) -> Mr18W<ImrSpec> {
        Mr18W::new(self, 18)
    }
}
#[doc = "Interrupt mask register (EXTI_IMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
