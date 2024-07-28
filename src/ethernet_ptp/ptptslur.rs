#[doc = "Register `PTPTSLUR` reader"]
pub type R = crate::R<PtptslurSpec>;
#[doc = "Register `PTPTSLUR` writer"]
pub type W = crate::W<PtptslurSpec>;
#[doc = "Field `TSUSS` reader - Time stamp update subseconds"]
pub type TsussR = crate::FieldReader<u32>;
#[doc = "Field `TSUSS` writer - Time stamp update subseconds"]
pub type TsussW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TSUPNS` reader - Time stamp update positive or negative sign"]
pub type TsupnsR = crate::BitReader;
#[doc = "Field `TSUPNS` writer - Time stamp update positive or negative sign"]
pub type TsupnsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    pub fn tsuss(&self) -> TsussR {
        TsussR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    pub fn tsupns(&self) -> TsupnsR {
        TsupnsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Time stamp update subseconds"]
    #[inline(always)]
    #[must_use]
    pub fn tsuss(&mut self) -> TsussW<PtptslurSpec> {
        TsussW::new(self, 0)
    }
    #[doc = "Bit 31 - Time stamp update positive or negative sign"]
    #[inline(always)]
    #[must_use]
    pub fn tsupns(&mut self) -> TsupnsW<PtptslurSpec> {
        TsupnsW::new(self, 31)
    }
}
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptslur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptslur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptslurSpec;
impl crate::RegisterSpec for PtptslurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslur::R`](R) reader structure"]
impl crate::Readable for PtptslurSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptslur::W`](W) writer structure"]
impl crate::Writable for PtptslurSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSLUR to value 0"]
impl crate::Resettable for PtptslurSpec {
    const RESET_VALUE: u32 = 0;
}
