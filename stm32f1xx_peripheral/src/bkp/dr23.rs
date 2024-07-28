#[doc = "Register `DR23` reader"]
pub type R = crate::R<Dr23Spec>;
#[doc = "Register `DR23` writer"]
pub type W = crate::W<Dr23Spec>;
#[doc = "Field `D23` reader - Backup data"]
pub type D23R = crate::FieldReader<u16>;
#[doc = "Field `D23` writer - Backup data"]
pub type D23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d23(&self) -> D23R {
        D23R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d23(&mut self) -> D23W<Dr23Spec> {
        D23W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr23Spec;
impl crate::RegisterSpec for Dr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr23::R`](R) reader structure"]
impl crate::Readable for Dr23Spec {}
#[doc = "`write(|w| ..)` method takes [`dr23::W`](W) writer structure"]
impl crate::Writable for Dr23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR23 to value 0"]
impl crate::Resettable for Dr23Spec {
    const RESET_VALUE: u32 = 0;
}
