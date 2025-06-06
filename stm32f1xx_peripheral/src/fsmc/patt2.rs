#[doc = "Register `PATT2` reader"]
pub type R = crate::R<Patt2Spec>;
#[doc = "Register `PATT2` writer"]
pub type W = crate::W<Patt2Spec>;
#[doc = "Field `ATTSETx` reader - Attribute memory x setup time"]
pub type AttsetxR = crate::FieldReader;
#[doc = "Field `ATTSETx` writer - Attribute memory x setup time"]
pub type AttsetxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAITx` reader - Attribute memory x wait time"]
pub type AttwaitxR = crate::FieldReader;
#[doc = "Field `ATTWAITx` writer - Attribute memory x wait time"]
pub type AttwaitxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHOLDx` reader - Attribute memory x hold time"]
pub type AttholdxR = crate::FieldReader;
#[doc = "Field `ATTHOLDx` writer - Attribute memory x hold time"]
pub type AttholdxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZx` reader - Attribute memory x databus HiZ time"]
pub type AtthizxR = crate::FieldReader;
#[doc = "Field `ATTHIZx` writer - Attribute memory x databus HiZ time"]
pub type AtthizxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    pub fn attsetx(&self) -> AttsetxR {
        AttsetxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    pub fn attwaitx(&self) -> AttwaitxR {
        AttwaitxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    pub fn attholdx(&self) -> AttholdxR {
        AttholdxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    pub fn atthizx(&self) -> AtthizxR {
        AtthizxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    #[must_use]
    pub fn attsetx(&mut self) -> AttsetxW<Patt2Spec> {
        AttsetxW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    #[must_use]
    pub fn attwaitx(&mut self) -> AttwaitxW<Patt2Spec> {
        AttwaitxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    #[must_use]
    pub fn attholdx(&mut self) -> AttholdxW<Patt2Spec> {
        AttholdxW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn atthizx(&mut self) -> AtthizxW<Patt2Spec> {
        AtthizxW::new(self, 24)
    }
}
#[doc = "Attribute memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`patt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Patt2Spec;
impl crate::RegisterSpec for Patt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patt2::R`](R) reader structure"]
impl crate::Readable for Patt2Spec {}
#[doc = "`write(|w| ..)` method takes [`patt2::W`](W) writer structure"]
impl crate::Writable for Patt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PATT2 to value 0xfcfc_fcfc"]
impl crate::Resettable for Patt2Spec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
