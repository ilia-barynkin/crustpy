#[doc = "Register `DMAIER` reader"]
pub type R = crate::R<DmaierSpec>;
#[doc = "Register `DMAIER` writer"]
pub type W = crate::W<DmaierSpec>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPSIE` reader - Transmit process stopped interrupt enable"]
pub type TpsieR = crate::BitReader;
#[doc = "Field `TPSIE` writer - Transmit process stopped interrupt enable"]
pub type TpsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUIE` reader - Transmit buffer unavailable interrupt enable"]
pub type TbuieR = crate::BitReader;
#[doc = "Field `TBUIE` writer - Transmit buffer unavailable interrupt enable"]
pub type TbuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJTIE` reader - Transmit jabber timeout interrupt enable"]
pub type TjtieR = crate::BitReader;
#[doc = "Field `TJTIE` writer - Transmit jabber timeout interrupt enable"]
pub type TjtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIE` reader - Overflow interrupt enable"]
pub type RoieR = crate::BitReader;
#[doc = "Field `ROIE` writer - Overflow interrupt enable"]
pub type RoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUIE` reader - Underflow interrupt enable"]
pub type TuieR = crate::BitReader;
#[doc = "Field `TUIE` writer - Underflow interrupt enable"]
pub type TuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUIE` reader - Receive buffer unavailable interrupt enable"]
pub type RbuieR = crate::BitReader;
#[doc = "Field `RBUIE` writer - Receive buffer unavailable interrupt enable"]
pub type RbuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPSIE` reader - Receive process stopped interrupt enable"]
pub type RpsieR = crate::BitReader;
#[doc = "Field `RPSIE` writer - Receive process stopped interrupt enable"]
pub type RpsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTIE` reader - receive watchdog timeout interrupt enable"]
pub type RwtieR = crate::BitReader;
#[doc = "Field `RWTIE` writer - receive watchdog timeout interrupt enable"]
pub type RwtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETIE` reader - Early transmit interrupt enable"]
pub type EtieR = crate::BitReader;
#[doc = "Field `ETIE` writer - Early transmit interrupt enable"]
pub type EtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEIE` reader - Fatal bus error interrupt enable"]
pub type FbeieR = crate::BitReader;
#[doc = "Field `FBEIE` writer - Fatal bus error interrupt enable"]
pub type FbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - Early receive interrupt enable"]
pub type ErieR = crate::BitReader;
#[doc = "Field `ERIE` writer - Early receive interrupt enable"]
pub type ErieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AISE` reader - Abnormal interrupt summary enable"]
pub type AiseR = crate::BitReader;
#[doc = "Field `AISE` writer - Abnormal interrupt summary enable"]
pub type AiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISE` reader - Normal interrupt summary enable"]
pub type NiseR = crate::BitReader;
#[doc = "Field `NISE` writer - Normal interrupt summary enable"]
pub type NiseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    pub fn tpsie(&self) -> TpsieR {
        TpsieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn tbuie(&self) -> TbuieR {
        TbuieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    pub fn tjtie(&self) -> TjtieR {
        TjtieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn roie(&self) -> RoieR {
        RoieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TuieR {
        TuieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn rbuie(&self) -> RbuieR {
        RbuieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    pub fn rpsie(&self) -> RpsieR {
        RpsieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    pub fn rwtie(&self) -> RwtieR {
        RwtieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn etie(&self) -> EtieR {
        EtieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    pub fn fbeie(&self) -> FbeieR {
        FbeieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn erie(&self) -> ErieR {
        ErieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    pub fn aise(&self) -> AiseR {
        AiseR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    pub fn nise(&self) -> NiseR {
        NiseR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<DmaierSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpsie(&mut self) -> TpsieW<DmaierSpec> {
        TpsieW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbuie(&mut self) -> TbuieW<DmaierSpec> {
        TbuieW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tjtie(&mut self) -> TjtieW<DmaierSpec> {
        TjtieW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> RoieW<DmaierSpec> {
        RoieW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuie(&mut self) -> TuieW<DmaierSpec> {
        TuieW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<DmaierSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbuie(&mut self) -> RbuieW<DmaierSpec> {
        RbuieW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpsie(&mut self) -> RpsieW<DmaierSpec> {
        RpsieW::new(self, 8)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwtie(&mut self) -> RwtieW<DmaierSpec> {
        RwtieW::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> EtieW<DmaierSpec> {
        EtieW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbeie(&mut self) -> FbeieW<DmaierSpec> {
        FbeieW::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ErieW<DmaierSpec> {
        ErieW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn aise(&mut self) -> AiseW<DmaierSpec> {
        AiseW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn nise(&mut self) -> NiseW<DmaierSpec> {
        NiseW::new(self, 16)
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaierSpec;
impl crate::RegisterSpec for DmaierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaier::R`](R) reader structure"]
impl crate::Readable for DmaierSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaier::W`](W) writer structure"]
impl crate::Writable for DmaierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAIER to value 0"]
impl crate::Resettable for DmaierSpec {
    const RESET_VALUE: u32 = 0;
}
