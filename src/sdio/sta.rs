#[doc = "Register `STA` reader"]
pub type R = crate::R<StaSpec>;
#[doc = "Field `CCRCFAIL` reader - CCRCFAIL"]
pub type CcrcfailR = crate::BitReader;
#[doc = "Field `DCRCFAIL` reader - DCRCFAIL"]
pub type DcrcfailR = crate::BitReader;
#[doc = "Field `CTIMEOUT` reader - CTIMEOUT"]
pub type CtimeoutR = crate::BitReader;
#[doc = "Field `DTIMEOUT` reader - DTIMEOUT"]
pub type DtimeoutR = crate::BitReader;
#[doc = "Field `TXUNDERR` reader - TXUNDERR"]
pub type TxunderrR = crate::BitReader;
#[doc = "Field `RXOVERR` reader - RXOVERR"]
pub type RxoverrR = crate::BitReader;
#[doc = "Field `CMDREND` reader - CMDREND"]
pub type CmdrendR = crate::BitReader;
#[doc = "Field `CMDSENT` reader - CMDSENT"]
pub type CmdsentR = crate::BitReader;
#[doc = "Field `DATAEND` reader - DATAEND"]
pub type DataendR = crate::BitReader;
#[doc = "Field `STBITERR` reader - STBITERR"]
pub type StbiterrR = crate::BitReader;
#[doc = "Field `DBCKEND` reader - DBCKEND"]
pub type DbckendR = crate::BitReader;
#[doc = "Field `CMDACT` reader - CMDACT"]
pub type CmdactR = crate::BitReader;
#[doc = "Field `TXACT` reader - TXACT"]
pub type TxactR = crate::BitReader;
#[doc = "Field `RXACT` reader - RXACT"]
pub type RxactR = crate::BitReader;
#[doc = "Field `TXFIFOHE` reader - TXFIFOHE"]
pub type TxfifoheR = crate::BitReader;
#[doc = "Field `RXFIFOHF` reader - RXFIFOHF"]
pub type RxfifohfR = crate::BitReader;
#[doc = "Field `TXFIFOF` reader - TXFIFOF"]
pub type TxfifofR = crate::BitReader;
#[doc = "Field `RXFIFOF` reader - RXFIFOF"]
pub type RxfifofR = crate::BitReader;
#[doc = "Field `TXFIFOE` reader - TXFIFOE"]
pub type TxfifoeR = crate::BitReader;
#[doc = "Field `RXFIFOE` reader - RXFIFOE"]
pub type RxfifoeR = crate::BitReader;
#[doc = "Field `TXDAVL` reader - TXDAVL"]
pub type TxdavlR = crate::BitReader;
#[doc = "Field `RXDAVL` reader - RXDAVL"]
pub type RxdavlR = crate::BitReader;
#[doc = "Field `SDIOIT` reader - SDIOIT"]
pub type SdioitR = crate::BitReader;
#[doc = "Field `CEATAEND` reader - CEATAEND"]
pub type CeataendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CCRCFAIL"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CcrcfailR {
        CcrcfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DcrcfailR {
        DcrcfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CtimeoutR {
        CtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DtimeoutR {
        DtimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR"]
    #[inline(always)]
    pub fn txunderr(&self) -> TxunderrR {
        TxunderrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RxoverrR {
        RxoverrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CmdrendR {
        CmdrendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CmdsentR {
        CmdsentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND"]
    #[inline(always)]
    pub fn dataend(&self) -> DataendR {
        DataendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERR"]
    #[inline(always)]
    pub fn stbiterr(&self) -> StbiterrR {
        StbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND"]
    #[inline(always)]
    pub fn dbckend(&self) -> DbckendR {
        DbckendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACT"]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACT"]
    #[inline(always)]
    pub fn txact(&self) -> TxactR {
        TxactR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACT"]
    #[inline(always)]
    pub fn rxact(&self) -> RxactR {
        RxactR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHE"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TxfifoheR {
        TxfifoheR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHF"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RxfifohfR {
        RxfifohfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOF"]
    #[inline(always)]
    pub fn txfifof(&self) -> TxfifofR {
        TxfifofR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOF"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RxfifofR {
        RxfifofR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOE"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TxfifoeR {
        TxfifoeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOE"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RxfifoeR {
        RxfifoeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVL"]
    #[inline(always)]
    pub fn txdavl(&self) -> TxdavlR {
        TxdavlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVL"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RxdavlR {
        RxdavlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT"]
    #[inline(always)]
    pub fn sdioit(&self) -> SdioitR {
        SdioitR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATAEND"]
    #[inline(always)]
    pub fn ceataend(&self) -> CeataendR {
        CeataendR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "SDIO status register (SDIO_STA)\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaSpec;
impl crate::RegisterSpec for StaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for StaSpec {}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for StaSpec {
    const RESET_VALUE: u32 = 0;
}
