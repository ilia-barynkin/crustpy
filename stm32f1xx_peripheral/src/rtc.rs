#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crh: Crh,
    crl: Crl,
    prlh: Prlh,
    prll: Prll,
    divh: Divh,
    divl: Divl,
    cnth: Cnth,
    cntl: Cntl,
    alrh: Alrh,
    alrl: Alrl,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    #[inline(always)]
    pub const fn crh(&self) -> &Crh {
        &self.crh
    }
    #[doc = "0x04 - RTC Control Register Low"]
    #[inline(always)]
    pub const fn crl(&self) -> &Crl {
        &self.crl
    }
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub const fn prlh(&self) -> &Prlh {
        &self.prlh
    }
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    #[inline(always)]
    pub const fn prll(&self) -> &Prll {
        &self.prll
    }
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    #[inline(always)]
    pub const fn divh(&self) -> &Divh {
        &self.divh
    }
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    pub const fn divl(&self) -> &Divl {
        &self.divl
    }
    #[doc = "0x18 - RTC Counter Register High"]
    #[inline(always)]
    pub const fn cnth(&self) -> &Cnth {
        &self.cnth
    }
    #[doc = "0x1c - RTC Counter Register Low"]
    #[inline(always)]
    pub const fn cntl(&self) -> &Cntl {
        &self.cntl
    }
    #[doc = "0x20 - RTC Alarm Register High"]
    #[inline(always)]
    pub const fn alrh(&self) -> &Alrh {
        &self.alrh
    }
    #[doc = "0x24 - RTC Alarm Register Low"]
    #[inline(always)]
    pub const fn alrl(&self) -> &Alrl {
        &self.alrl
    }
}
#[doc = "CRH (rw) register accessor: RTC Control Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`]
module"]
#[doc(alias = "CRH")]
pub type Crh = crate::Reg<crh::CrhSpec>;
#[doc = "RTC Control Register High"]
pub mod crh;
#[doc = "CRL (rw) register accessor: RTC Control Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`]
module"]
#[doc(alias = "CRL")]
pub type Crl = crate::Reg<crl::CrlSpec>;
#[doc = "RTC Control Register Low"]
pub mod crl;
#[doc = "PRLH (w) register accessor: RTC Prescaler Load Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prlh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prlh`]
module"]
#[doc(alias = "PRLH")]
pub type Prlh = crate::Reg<prlh::PrlhSpec>;
#[doc = "RTC Prescaler Load Register High"]
pub mod prlh;
#[doc = "PRLL (w) register accessor: RTC Prescaler Load Register Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prll::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prll`]
module"]
#[doc(alias = "PRLL")]
pub type Prll = crate::Reg<prll::PrllSpec>;
#[doc = "RTC Prescaler Load Register Low"]
pub mod prll;
#[doc = "DIVH (r) register accessor: RTC Prescaler Divider Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`divh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`]
module"]
#[doc(alias = "DIVH")]
pub type Divh = crate::Reg<divh::DivhSpec>;
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;
#[doc = "DIVL (r) register accessor: RTC Prescaler Divider Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`divl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`]
module"]
#[doc(alias = "DIVL")]
pub type Divl = crate::Reg<divl::DivlSpec>;
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: RTC Counter Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
#[doc(alias = "CNTH")]
pub type Cnth = crate::Reg<cnth::CnthSpec>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC Counter Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
#[doc(alias = "CNTL")]
pub type Cntl = crate::Reg<cntl::CntlSpec>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "ALRH (w) register accessor: RTC Alarm Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrh`]
module"]
#[doc(alias = "ALRH")]
pub type Alrh = crate::Reg<alrh::AlrhSpec>;
#[doc = "RTC Alarm Register High"]
pub mod alrh;
#[doc = "ALRL (w) register accessor: RTC Alarm Register Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrl`]
module"]
#[doc(alias = "ALRL")]
pub type Alrl = crate::Reg<alrl::AlrlSpec>;
#[doc = "RTC Alarm Register Low"]
pub mod alrl;
