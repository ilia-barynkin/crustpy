#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<Smpr1Spec>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<Smpr1Spec>;
#[doc = "Field `SMP10` reader - Channel 10 sample time selection"]
pub type Smp10R = crate::FieldReader;
#[doc = "Field `SMP10` writer - Channel 10 sample time selection"]
pub type Smp10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP11` reader - Channel 11 sample time selection"]
pub type Smp11R = crate::FieldReader;
#[doc = "Field `SMP11` writer - Channel 11 sample time selection"]
pub type Smp11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP12` reader - Channel 12 sample time selection"]
pub type Smp12R = crate::FieldReader;
#[doc = "Field `SMP12` writer - Channel 12 sample time selection"]
pub type Smp12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP13` reader - Channel 13 sample time selection"]
pub type Smp13R = crate::FieldReader;
#[doc = "Field `SMP13` writer - Channel 13 sample time selection"]
pub type Smp13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP14` reader - Channel 14 sample time selection"]
pub type Smp14R = crate::FieldReader;
#[doc = "Field `SMP14` writer - Channel 14 sample time selection"]
pub type Smp14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP15` reader - Channel 15 sample time selection"]
pub type Smp15R = crate::FieldReader;
#[doc = "Field `SMP15` writer - Channel 15 sample time selection"]
pub type Smp15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP16` reader - Channel 16 sample time selection"]
pub type Smp16R = crate::FieldReader;
#[doc = "Field `SMP16` writer - Channel 16 sample time selection"]
pub type Smp16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP17` reader - Channel 17 sample time selection"]
pub type Smp17R = crate::FieldReader;
#[doc = "Field `SMP17` writer - Channel 17 sample time selection"]
pub type Smp17W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> Smp10R {
        Smp10R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> Smp11R {
        Smp11R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> Smp12R {
        Smp12R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> Smp13R {
        Smp13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> Smp14R {
        Smp14R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> Smp15R {
        Smp15R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> Smp16R {
        Smp16R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> Smp17R {
        Smp17R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> Smp10W<Smpr1Spec> {
        Smp10W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> Smp11W<Smpr1Spec> {
        Smp11W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> Smp12W<Smpr1Spec> {
        Smp12W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> Smp13W<Smpr1Spec> {
        Smp13W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> Smp14W<Smpr1Spec> {
        Smp14W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> Smp15W<Smpr1Spec> {
        Smp15W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> Smp16W<Smpr1Spec> {
        Smp16W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> Smp17W<Smpr1Spec> {
        Smp17W::new(self, 21)
    }
}
#[doc = "sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr1Spec;
impl crate::RegisterSpec for Smpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for Smpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for Smpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for Smpr1Spec {
    const RESET_VALUE: u32 = 0;
}
