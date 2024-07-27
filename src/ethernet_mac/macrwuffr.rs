#[doc = "Register `MACRWUFFR` reader"]
pub type R = crate::R<MacrwuffrSpec>;
#[doc = "Register `MACRWUFFR` writer"]
pub type W = crate::W<MacrwuffrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macrwuffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwuffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacrwuffrSpec;
impl crate::RegisterSpec for MacrwuffrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwuffr::R`](R) reader structure"]
impl crate::Readable for MacrwuffrSpec {}
#[doc = "`write(|w| ..)` method takes [`macrwuffr::W`](W) writer structure"]
impl crate::Writable for MacrwuffrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRWUFFR to value 0"]
impl crate::Resettable for MacrwuffrSpec {
    const RESET_VALUE: u32 = 0;
}
