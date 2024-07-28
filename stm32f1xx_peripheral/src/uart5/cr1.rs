#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SBK` reader - SBK"]
pub type SbkR = crate::BitReader;
#[doc = "Field `SBK` writer - SBK"]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - RWU"]
pub type RwuR = crate::BitReader;
#[doc = "Field `RWU` writer - RWU"]
pub type RwuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - RE"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - RE"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - TE"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - TE"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLEIE"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLEIE"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - TXEIE"]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - TXEIE"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PEIE"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PEIE"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - PS"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - PS"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - PCE"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - PCE"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - WAKE"]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - WAKE"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - M"]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - M"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UE` reader - UE"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - UE"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SBK"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RWU"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RE"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TE"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLEIE"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXEIE"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PS"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCE"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAKE"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - M"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UE"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBK"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SbkW<Cr1Spec> {
        SbkW::new(self, 0)
    }
    #[doc = "Bit 1 - RWU"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RwuW<Cr1Spec> {
        RwuW::new(self, 1)
    }
    #[doc = "Bit 2 - RE"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - TE"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLEIE"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IdleieW<Cr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RxneieW<Cr1Spec> {
        RxneieW::new(self, 5)
    }
    #[doc = "Bit 6 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - TXEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TxeieW<Cr1Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 8 - PEIE"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<Cr1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - PS"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - PCE"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - WAKE"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WakeW<Cr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - M"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<Cr1Spec> {
        MW::new(self, 12)
    }
    #[doc = "Bit 13 - UE"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UeW<Cr1Spec> {
        UeW::new(self, 13)
    }
}
#[doc = "UART4_CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
