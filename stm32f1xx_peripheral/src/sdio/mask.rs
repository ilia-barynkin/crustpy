#[doc = "Register `MASK` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `MASK` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `CCRCFAILIE` reader - CCRCFAILIE"]
pub type CcrcfailieR = crate::BitReader;
#[doc = "Field `CCRCFAILIE` writer - CCRCFAILIE"]
pub type CcrcfailieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILIE` reader - DCRCFAILIE"]
pub type DcrcfailieR = crate::BitReader;
#[doc = "Field `DCRCFAILIE` writer - DCRCFAILIE"]
pub type DcrcfailieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTIE` reader - CTIMEOUTIE"]
pub type CtimeoutieR = crate::BitReader;
#[doc = "Field `CTIMEOUTIE` writer - CTIMEOUTIE"]
pub type CtimeoutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTIE` reader - DTIMEOUTIE"]
pub type DtimeoutieR = crate::BitReader;
#[doc = "Field `DTIMEOUTIE` writer - DTIMEOUTIE"]
pub type DtimeoutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRIE` reader - TXUNDERRIE"]
pub type TxunderrieR = crate::BitReader;
#[doc = "Field `TXUNDERRIE` writer - TXUNDERRIE"]
pub type TxunderrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRIE` reader - RXOVERRIE"]
pub type RxoverrieR = crate::BitReader;
#[doc = "Field `RXOVERRIE` writer - RXOVERRIE"]
pub type RxoverrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDIE` reader - CMDRENDIE"]
pub type CmdrendieR = crate::BitReader;
#[doc = "Field `CMDRENDIE` writer - CMDRENDIE"]
pub type CmdrendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTIE` reader - CMDSENTIE"]
pub type CmdsentieR = crate::BitReader;
#[doc = "Field `CMDSENTIE` writer - CMDSENTIE"]
pub type CmdsentieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDIE` reader - DATAENDIE"]
pub type DataendieR = crate::BitReader;
#[doc = "Field `DATAENDIE` writer - DATAENDIE"]
pub type DataendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITERRIE` reader - STBITERRIE"]
pub type StbiterrieR = crate::BitReader;
#[doc = "Field `STBITERRIE` writer - STBITERRIE"]
pub type StbiterrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBACKENDIE` reader - DBACKENDIE"]
pub type DbackendieR = crate::BitReader;
#[doc = "Field `DBACKENDIE` writer - DBACKENDIE"]
pub type DbackendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDACTIE` reader - CMDACTIE"]
pub type CmdactieR = crate::BitReader;
#[doc = "Field `CMDACTIE` writer - CMDACTIE"]
pub type CmdactieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXACTIE` reader - TXACTIE"]
pub type TxactieR = crate::BitReader;
#[doc = "Field `TXACTIE` writer - TXACTIE"]
pub type TxactieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACTIE` reader - RXACTIE"]
pub type RxactieR = crate::BitReader;
#[doc = "Field `RXACTIE` writer - RXACTIE"]
pub type RxactieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOHEIE` reader - TXFIFOHEIE"]
pub type TxfifoheieR = crate::BitReader;
#[doc = "Field `TXFIFOHEIE` writer - TXFIFOHEIE"]
pub type TxfifoheieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOHFIE` reader - RXFIFOHFIE"]
pub type RxfifohfieR = crate::BitReader;
#[doc = "Field `RXFIFOHFIE` writer - RXFIFOHFIE"]
pub type RxfifohfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOFIE` reader - TXFIFOFIE"]
pub type TxfifofieR = crate::BitReader;
#[doc = "Field `TXFIFOFIE` writer - TXFIFOFIE"]
pub type TxfifofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFIE` reader - RXFIFOFIE"]
pub type RxfifofieR = crate::BitReader;
#[doc = "Field `RXFIFOFIE` writer - RXFIFOFIE"]
pub type RxfifofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOEIE` reader - TXFIFOEIE"]
pub type TxfifoeieR = crate::BitReader;
#[doc = "Field `TXFIFOEIE` writer - TXFIFOEIE"]
pub type TxfifoeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOEIE` reader - RXFIFOEIE"]
pub type RxfifoeieR = crate::BitReader;
#[doc = "Field `RXFIFOEIE` writer - RXFIFOEIE"]
pub type RxfifoeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDAVLIE` reader - TXDAVLIE"]
pub type TxdavlieR = crate::BitReader;
#[doc = "Field `TXDAVLIE` writer - TXDAVLIE"]
pub type TxdavlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDAVLIE` reader - RXDAVLIE"]
pub type RxdavlieR = crate::BitReader;
#[doc = "Field `RXDAVLIE` writer - RXDAVLIE"]
pub type RxdavlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITIE` reader - SDIOITIE"]
pub type SdioitieR = crate::BitReader;
#[doc = "Field `SDIOITIE` writer - SDIOITIE"]
pub type SdioitieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATENDIE` reader - CEATENDIE"]
pub type CeatendieR = crate::BitReader;
#[doc = "Field `CEATENDIE` writer - CEATENDIE"]
pub type CeatendieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CcrcfailieR {
        CcrcfailieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DcrcfailieR {
        DcrcfailieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CtimeoutieR {
        CtimeoutieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DtimeoutieR {
        DtimeoutieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TxunderrieR {
        TxunderrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RxoverrieR {
        RxoverrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CmdrendieR {
        CmdrendieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CmdsentieR {
        CmdsentieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dataendie(&self) -> DataendieR {
        DataendieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn stbiterrie(&self) -> StbiterrieR {
        StbiterrieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dbackendie(&self) -> DbackendieR {
        DbackendieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn cmdactie(&self) -> CmdactieR {
        CmdactieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn txactie(&self) -> TxactieR {
        TxactieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn rxactie(&self) -> RxactieR {
        RxactieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TxfifoheieR {
        TxfifoheieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RxfifohfieR {
        RxfifohfieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TxfifofieR {
        TxfifofieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RxfifofieR {
        RxfifofieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TxfifoeieR {
        TxfifoeieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RxfifoeieR {
        RxfifoeieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TxdavlieR {
        TxdavlieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RxdavlieR {
        RxdavlieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SdioitieR {
        SdioitieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATENDIE"]
    #[inline(always)]
    pub fn ceatendie(&self) -> CeatendieR {
        CeatendieR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailie(&mut self) -> CcrcfailieW<MaskSpec> {
        CcrcfailieW::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailie(&mut self) -> DcrcfailieW<MaskSpec> {
        DcrcfailieW::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutie(&mut self) -> CtimeoutieW<MaskSpec> {
        CtimeoutieW::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutie(&mut self) -> DtimeoutieW<MaskSpec> {
        DtimeoutieW::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrie(&mut self) -> TxunderrieW<MaskSpec> {
        TxunderrieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrie(&mut self) -> RxoverrieW<MaskSpec> {
        RxoverrieW::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendie(&mut self) -> CmdrendieW<MaskSpec> {
        CmdrendieW::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentie(&mut self) -> CmdsentieW<MaskSpec> {
        CmdsentieW::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dataendie(&mut self) -> DataendieW<MaskSpec> {
        DataendieW::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn stbiterrie(&mut self) -> StbiterrieW<MaskSpec> {
        StbiterrieW::new(self, 9)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dbackendie(&mut self) -> DbackendieW<MaskSpec> {
        DbackendieW::new(self, 10)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdactie(&mut self) -> CmdactieW<MaskSpec> {
        CmdactieW::new(self, 11)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn txactie(&mut self) -> TxactieW<MaskSpec> {
        TxactieW::new(self, 12)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxactie(&mut self) -> RxactieW<MaskSpec> {
        RxactieW::new(self, 13)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoheie(&mut self) -> TxfifoheieW<MaskSpec> {
        TxfifoheieW::new(self, 14)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifohfie(&mut self) -> RxfifohfieW<MaskSpec> {
        RxfifohfieW::new(self, 15)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfifofie(&mut self) -> TxfifofieW<MaskSpec> {
        TxfifofieW::new(self, 16)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofie(&mut self) -> RxfifofieW<MaskSpec> {
        RxfifofieW::new(self, 17)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoeie(&mut self) -> TxfifoeieW<MaskSpec> {
        TxfifoeieW::new(self, 18)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoeie(&mut self) -> RxfifoeieW<MaskSpec> {
        RxfifoeieW::new(self, 19)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdavlie(&mut self) -> TxdavlieW<MaskSpec> {
        TxdavlieW::new(self, 20)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdavlie(&mut self) -> RxdavlieW<MaskSpec> {
        RxdavlieW::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SdioitieW<MaskSpec> {
        SdioitieW::new(self, 22)
    }
    #[doc = "Bit 23 - CEATENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn ceatendie(&mut self) -> CeatendieW<MaskSpec> {
        CeatendieW::new(self, 23)
    }
}
#[doc = "SDIO mask register (SDIO_MASK)\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
