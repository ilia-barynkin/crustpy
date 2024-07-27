#[doc = "Register `EP0R` reader"]
pub type R = crate::R<Ep0rSpec>;
#[doc = "Register `EP0R` writer"]
pub type W = crate::W<Ep0rSpec>;
#[doc = "Field `EA` reader - Endpoint address"]
pub type EaR = crate::FieldReader;
#[doc = "Field `EA` writer - Endpoint address"]
pub type EaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STAT_TX` reader - Status bits, for transmission transfers"]
pub type StatTxR = crate::FieldReader;
#[doc = "Field `STAT_TX` writer - Status bits, for transmission transfers"]
pub type StatTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOG_TX` reader - Data Toggle, for transmission transfers"]
pub type DtogTxR = crate::BitReader;
#[doc = "Field `DTOG_TX` writer - Data Toggle, for transmission transfers"]
pub type DtogTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TX` reader - Correct Transfer for transmission"]
pub type CtrTxR = crate::BitReader;
#[doc = "Field `CTR_TX` writer - Correct Transfer for transmission"]
pub type CtrTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_KIND` reader - Endpoint kind"]
pub type EpKindR = crate::BitReader;
#[doc = "Field `EP_KIND` writer - Endpoint kind"]
pub type EpKindW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_TYPE` reader - Endpoint type"]
pub type EpTypeR = crate::FieldReader;
#[doc = "Field `EP_TYPE` writer - Endpoint type"]
pub type EpTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAT_RX` reader - Status bits, for reception transfers"]
pub type StatRxR = crate::FieldReader;
#[doc = "Field `STAT_RX` writer - Status bits, for reception transfers"]
pub type StatRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOG_RX` reader - Data Toggle, for reception transfers"]
pub type DtogRxR = crate::BitReader;
#[doc = "Field `DTOG_RX` writer - Data Toggle, for reception transfers"]
pub type DtogRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_RX` reader - Correct transfer for reception"]
pub type CtrRxR = crate::BitReader;
#[doc = "Field `CTR_RX` writer - Correct transfer for reception"]
pub type CtrRxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&self) -> StatTxR {
        StatTxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DtogTxR {
        DtogTxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CtrTxR {
        CtrTxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EpKindR {
        EpKindR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&self) -> EpTypeR {
        EpTypeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&self) -> StatRxR {
        StatRxR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DtogRxR {
        DtogRxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CtrRxR {
        CtrRxR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EaW<Ep0rSpec> {
        EaW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> StatTxW<Ep0rSpec> {
        StatTxW::new(self, 4)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DtogTxW<Ep0rSpec> {
        DtogTxW::new(self, 6)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CtrTxW<Ep0rSpec> {
        CtrTxW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EpKindW<Ep0rSpec> {
        EpKindW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EpTypeW<Ep0rSpec> {
        EpTypeW::new(self, 9)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SetupW<Ep0rSpec> {
        SetupW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> StatRxW<Ep0rSpec> {
        StatRxW::new(self, 12)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DtogRxW<Ep0rSpec> {
        DtogRxW::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CtrRxW<Ep0rSpec> {
        CtrRxW::new(self, 15)
    }
}
#[doc = "endpoint 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0rSpec;
impl crate::RegisterSpec for Ep0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0r::R`](R) reader structure"]
impl crate::Readable for Ep0rSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0r::W`](W) writer structure"]
impl crate::Writable for Ep0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0R to value 0"]
impl crate::Resettable for Ep0rSpec {
    const RESET_VALUE: u32 = 0;
}
