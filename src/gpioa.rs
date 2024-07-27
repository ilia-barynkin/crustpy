#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crl: Crl,
    crh: Crh,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    brr: Brr,
    lckr: Lckr,
}
impl RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn crl(&self) -> &Crl {
        &self.crl
    }
    #[doc = "0x04 - Port configuration register high (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn crh(&self) -> &Crh {
        &self.crh
    }
    #[doc = "0x08 - Port input data register (GPIOn_IDR)"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x0c - Port output data register (GPIOn_ODR)"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x14 - Port bit reset register (GPIOn_BRR)"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x18 - Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &Lckr {
        &self.lckr
    }
}
#[doc = "CRL (rw) register accessor: Port configuration register low (GPIOn_CRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`]
module"]
#[doc(alias = "CRL")]
pub type Crl = crate::Reg<crl::CrlSpec>;
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub mod crl;
#[doc = "CRH (rw) register accessor: Port configuration register high (GPIOn_CRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`]
module"]
#[doc(alias = "CRH")]
pub type Crh = crate::Reg<crh::CrhSpec>;
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub mod crh;
#[doc = "IDR (r) register accessor: Port input data register (GPIOn_IDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Port input data register (GPIOn_IDR)"]
pub mod idr;
#[doc = "ODR (rw) register accessor: Port output data register (GPIOn_ODR)\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "Port output data register (GPIOn_ODR)"]
pub mod odr;
#[doc = "BSRR (w) register accessor: Port bit set/reset register (GPIOn_BSRR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`]
module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub mod bsrr;
#[doc = "BRR (w) register accessor: Port bit reset register (GPIOn_BRR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub mod brr;
#[doc = "LCKR (rw) register accessor: Port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`]
module"]
#[doc(alias = "LCKR")]
pub type Lckr = crate::Reg<lckr::LckrSpec>;
#[doc = "Port configuration lock register"]
pub mod lckr;
