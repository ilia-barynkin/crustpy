#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADD` reader - Address of the USART node"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBDL` reader - lin break detection length"]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - lin break detection length"]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<Cr2Spec> {
        AddW::new(self, 0)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LbdlW<Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LbdieW<Cr2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LinenW<Cr2Spec> {
        LinenW::new(self, 14)
    }
}
#[doc = "UART4_CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
