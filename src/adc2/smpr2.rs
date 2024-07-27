#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<Smpr2Spec>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<Smpr2Spec>;
#[doc = "Field `SMP0` reader - Channel 0 sample time selection"]
pub type Smp0R = crate::FieldReader;
#[doc = "Field `SMP0` writer - Channel 0 sample time selection"]
pub type Smp0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP1` reader - Channel 1 sample time selection"]
pub type Smp1R = crate::FieldReader;
#[doc = "Field `SMP1` writer - Channel 1 sample time selection"]
pub type Smp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP2` reader - Channel 2 sample time selection"]
pub type Smp2R = crate::FieldReader;
#[doc = "Field `SMP2` writer - Channel 2 sample time selection"]
pub type Smp2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP3` reader - Channel 3 sample time selection"]
pub type Smp3R = crate::FieldReader;
#[doc = "Field `SMP3` writer - Channel 3 sample time selection"]
pub type Smp3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP4` reader - Channel 4 sample time selection"]
pub type Smp4R = crate::FieldReader;
#[doc = "Field `SMP4` writer - Channel 4 sample time selection"]
pub type Smp4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP5` reader - Channel 5 sample time selection"]
pub type Smp5R = crate::FieldReader;
#[doc = "Field `SMP5` writer - Channel 5 sample time selection"]
pub type Smp5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP6` reader - Channel 6 sample time selection"]
pub type Smp6R = crate::FieldReader;
#[doc = "Field `SMP6` writer - Channel 6 sample time selection"]
pub type Smp6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP7` reader - Channel 7 sample time selection"]
pub type Smp7R = crate::FieldReader;
#[doc = "Field `SMP7` writer - Channel 7 sample time selection"]
pub type Smp7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP8` reader - Channel 8 sample time selection"]
pub type Smp8R = crate::FieldReader;
#[doc = "Field `SMP8` writer - Channel 8 sample time selection"]
pub type Smp8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP9` reader - Channel 9 sample time selection"]
pub type Smp9R = crate::FieldReader;
#[doc = "Field `SMP9` writer - Channel 9 sample time selection"]
pub type Smp9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> Smp0R {
        Smp0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> Smp1R {
        Smp1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> Smp2R {
        Smp2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> Smp3R {
        Smp3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> Smp4R {
        Smp4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> Smp5R {
        Smp5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> Smp6R {
        Smp6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> Smp7R {
        Smp7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> Smp8R {
        Smp8R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> Smp9R {
        Smp9R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> Smp0W<Smpr2Spec> {
        Smp0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> Smp1W<Smpr2Spec> {
        Smp1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> Smp2W<Smpr2Spec> {
        Smp2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> Smp3W<Smpr2Spec> {
        Smp3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> Smp4W<Smpr2Spec> {
        Smp4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> Smp5W<Smpr2Spec> {
        Smp5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> Smp6W<Smpr2Spec> {
        Smp6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> Smp7W<Smpr2Spec> {
        Smp7W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> Smp8W<Smpr2Spec> {
        Smp8W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> Smp9W<Smpr2Spec> {
        Smp9W::new(self, 27)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr2Spec;
impl crate::RegisterSpec for Smpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for Smpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for Smpr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for Smpr2Spec {
    const RESET_VALUE: u32 = 0;
}
