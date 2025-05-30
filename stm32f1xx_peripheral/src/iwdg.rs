#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    kr: Kr,
    pr: Pr,
    rlr: Rlr,
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_KR)"]
    #[inline(always)]
    pub const fn kr(&self) -> &Kr {
        &self.kr
    }
    #[doc = "0x04 - Prescaler register (IWDG_PR)"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x08 - Reload register (IWDG_RLR)"]
    #[inline(always)]
    pub const fn rlr(&self) -> &Rlr {
        &self.rlr
    }
    #[doc = "0x0c - Status register (IWDG_SR)"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "KR (w) register accessor: Key register (IWDG_KR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`]
module"]
#[doc(alias = "KR")]
pub type Kr = crate::Reg<kr::KrSpec>;
#[doc = "Key register (IWDG_KR)"]
pub mod kr;
#[doc = "PR (rw) register accessor: Prescaler register (IWDG_PR)\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Prescaler register (IWDG_PR)"]
pub mod pr;
#[doc = "RLR (rw) register accessor: Reload register (IWDG_RLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
#[doc(alias = "RLR")]
pub type Rlr = crate::Reg<rlr::RlrSpec>;
#[doc = "Reload register (IWDG_RLR)"]
pub mod rlr;
#[doc = "SR (r) register accessor: Status register (IWDG_SR)\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register (IWDG_SR)"]
pub mod sr;
