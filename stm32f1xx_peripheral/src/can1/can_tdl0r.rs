#[doc = "Register `CAN_TDL0R` reader"]
pub type R = crate::R<CanTdl0rSpec>;
#[doc = "Register `CAN_TDL0R` writer"]
pub type W = crate::W<CanTdl0rSpec>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - DATA0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - DATA2"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - DATA3"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<CanTdl0rSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<CanTdl0rSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> Data2W<CanTdl0rSpec> {
        Data2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> Data3W<CanTdl0rSpec> {
        Data3W::new(self, 24)
    }
}
#[doc = "CAN_TDL0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdl0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdl0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTdl0rSpec;
impl crate::RegisterSpec for CanTdl0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tdl0r::R`](R) reader structure"]
impl crate::Readable for CanTdl0rSpec {}
#[doc = "`write(|w| ..)` method takes [`can_tdl0r::W`](W) writer structure"]
impl crate::Writable for CanTdl0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_TDL0R to value 0"]
impl crate::Resettable for CanTdl0rSpec {
    const RESET_VALUE: u32 = 0;
}
