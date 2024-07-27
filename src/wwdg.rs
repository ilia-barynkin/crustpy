#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfr: Cfr,
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    #[inline(always)]
    pub const fn cfr(&self) -> &Cfr {
        &self.cfr
    }
    #[doc = "0x08 - Status register (WWDG_SR)"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "CR (rw) register accessor: Control register (WWDG_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register (WWDG_CR)"]
pub mod cr;
#[doc = "CFR (rw) register accessor: Configuration register (WWDG_CFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`]
module"]
#[doc(alias = "CFR")]
pub type Cfr = crate::Reg<cfr::CfrSpec>;
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfr;
#[doc = "SR (rw) register accessor: Status register (WWDG_SR)\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register (WWDG_SR)"]
pub mod sr;
