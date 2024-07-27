#[doc = "Register `DR35` reader"]
pub type R = crate::R<Dr35Spec>;
#[doc = "Register `DR35` writer"]
pub type W = crate::W<Dr35Spec>;
#[doc = "Field `D35` reader - Backup data"]
pub type D35R = crate::FieldReader<u16>;
#[doc = "Field `D35` writer - Backup data"]
pub type D35W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d35(&self) -> D35R {
        D35R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d35(&mut self) -> D35W<Dr35Spec> {
        D35W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr35Spec;
impl crate::RegisterSpec for Dr35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr35::R`](R) reader structure"]
impl crate::Readable for Dr35Spec {}
#[doc = "`write(|w| ..)` method takes [`dr35::W`](W) writer structure"]
impl crate::Writable for Dr35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR35 to value 0"]
impl crate::Resettable for Dr35Spec {
    const RESET_VALUE: u32 = 0;
}
