#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `PE` reader - PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `NE` reader - NE"]
pub type NeR = crate::BitReader;
#[doc = "Field `ORE` reader - ORE"]
pub type OreR = crate::BitReader;
#[doc = "Field `IDLE` reader - IDLE"]
pub type IdleR = crate::BitReader;
#[doc = "Field `RXNE` reader - RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - RXNE"]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - TC"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `LBD` reader - LBD"]
pub type LbdR = crate::BitReader;
#[doc = "Field `LBD` writer - LBD"]
pub type LbdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NE"]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ORE"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LBD"]
    #[inline(always)]
    pub fn lbd(&self) -> LbdR {
        LbdR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RXNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RxneW<SrSpec> {
        RxneW::new(self, 5)
    }
    #[doc = "Bit 6 - TC"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<SrSpec> {
        TcW::new(self, 6)
    }
    #[doc = "Bit 8 - LBD"]
    #[inline(always)]
    #[must_use]
    pub fn lbd(&mut self) -> LbdW<SrSpec> {
        LbdW::new(self, 8)
    }
}
#[doc = "UART4_SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
