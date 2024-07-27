#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CCRCFAILC` reader - CCRCFAILC"]
pub type CcrcfailcR = crate::BitReader;
#[doc = "Field `CCRCFAILC` writer - CCRCFAILC"]
pub type CcrcfailcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILC` reader - DCRCFAILC"]
pub type DcrcfailcR = crate::BitReader;
#[doc = "Field `DCRCFAILC` writer - DCRCFAILC"]
pub type DcrcfailcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUTC"]
pub type CtimeoutcR = crate::BitReader;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUTC"]
pub type CtimeoutcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUTC"]
pub type DtimeoutcR = crate::BitReader;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUTC"]
pub type DtimeoutcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRC` reader - TXUNDERRC"]
pub type TxunderrcR = crate::BitReader;
#[doc = "Field `TXUNDERRC` writer - TXUNDERRC"]
pub type TxunderrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRC` reader - RXOVERRC"]
pub type RxoverrcR = crate::BitReader;
#[doc = "Field `RXOVERRC` writer - RXOVERRC"]
pub type RxoverrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDC` reader - CMDRENDC"]
pub type CmdrendcR = crate::BitReader;
#[doc = "Field `CMDRENDC` writer - CMDRENDC"]
pub type CmdrendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTC` reader - CMDSENTC"]
pub type CmdsentcR = crate::BitReader;
#[doc = "Field `CMDSENTC` writer - CMDSENTC"]
pub type CmdsentcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDC` reader - DATAENDC"]
pub type DataendcR = crate::BitReader;
#[doc = "Field `DATAENDC` writer - DATAENDC"]
pub type DataendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITERRC` reader - STBITERRC"]
pub type StbiterrcR = crate::BitReader;
#[doc = "Field `STBITERRC` writer - STBITERRC"]
pub type StbiterrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDC` reader - DBCKENDC"]
pub type DbckendcR = crate::BitReader;
#[doc = "Field `DBCKENDC` writer - DBCKENDC"]
pub type DbckendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITC` reader - SDIOITC"]
pub type SdioitcR = crate::BitReader;
#[doc = "Field `SDIOITC` writer - SDIOITC"]
pub type SdioitcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATAENDC` reader - CEATAENDC"]
pub type CeataendcR = crate::BitReader;
#[doc = "Field `CEATAENDC` writer - CEATAENDC"]
pub type CeataendcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CcrcfailcR {
        CcrcfailcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DcrcfailcR {
        DcrcfailcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CtimeoutcR {
        CtimeoutcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DtimeoutcR {
        DtimeoutcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TxunderrcR {
        TxunderrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RxoverrcR {
        RxoverrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CmdrendcR {
        CmdrendcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CmdsentcR {
        CmdsentcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dataendc(&self) -> DataendcR {
        DataendcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    pub fn stbiterrc(&self) -> StbiterrcR {
        StbiterrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DbckendcR {
        DbckendcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SdioitcR {
        SdioitcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATAENDC"]
    #[inline(always)]
    pub fn ceataendc(&self) -> CeataendcR {
        CeataendcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CcrcfailcW<IcrSpec> {
        CcrcfailcW::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DcrcfailcW<IcrSpec> {
        DcrcfailcW::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CtimeoutcW<IcrSpec> {
        CtimeoutcW::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DtimeoutcW<IcrSpec> {
        DtimeoutcW::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TxunderrcW<IcrSpec> {
        TxunderrcW::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RxoverrcW<IcrSpec> {
        RxoverrcW::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CmdrendcW<IcrSpec> {
        CmdrendcW::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CmdsentcW<IcrSpec> {
        CmdsentcW::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DataendcW<IcrSpec> {
        DataendcW::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    #[must_use]
    pub fn stbiterrc(&mut self) -> StbiterrcW<IcrSpec> {
        StbiterrcW::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DbckendcW<IcrSpec> {
        DbckendcW::new(self, 10)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SdioitcW<IcrSpec> {
        SdioitcW::new(self, 22)
    }
    #[doc = "Bit 23 - CEATAENDC"]
    #[inline(always)]
    #[must_use]
    pub fn ceataendc(&mut self) -> CeataendcW<IcrSpec> {
        CeataendcW::new(self, 23)
    }
}
#[doc = "SDIO interrupt clear register (SDIO_ICR)\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
