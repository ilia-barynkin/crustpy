#[doc = "Register `RESPI1` reader"]
pub type R = crate::R<Respi1Spec>;
#[doc = "Field `CARDSTATUS1` reader - CARDSTATUS1"]
pub type Cardstatus1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> Cardstatus1R {
        Cardstatus1R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::Reg::read) this register and get [`respi1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Respi1Spec;
impl crate::RegisterSpec for Respi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respi1::R`](R) reader structure"]
impl crate::Readable for Respi1Spec {}
#[doc = "`reset()` method sets RESPI1 to value 0"]
impl crate::Resettable for Respi1Spec {
    const RESET_VALUE: u32 = 0;
}
