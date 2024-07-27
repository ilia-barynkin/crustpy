#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mmccr: Mmccr,
    mmcrir: Mmcrir,
    mmctir: Mmctir,
    mmcrimr: Mmcrimr,
    mmctimr: Mmctimr,
    _reserved5: [u8; 0x38],
    mmctgfsccr: Mmctgfsccr,
    mmctgfmsccr: Mmctgfmsccr,
    _reserved7: [u8; 0x14],
    mmctgfcr: Mmctgfcr,
    _reserved8: [u8; 0x28],
    mmcrfcecr: Mmcrfcecr,
    mmcrfaecr: Mmcrfaecr,
    _reserved10: [u8; 0x28],
    mmcrgufcr: Mmcrgufcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register (ETH_MMCCR)"]
    #[inline(always)]
    pub const fn mmccr(&self) -> &Mmccr {
        &self.mmccr
    }
    #[doc = "0x04 - Ethernet MMC receive interrupt register (ETH_MMCRIR)"]
    #[inline(always)]
    pub const fn mmcrir(&self) -> &Mmcrir {
        &self.mmcrir
    }
    #[doc = "0x08 - Ethernet MMC transmit interrupt register (ETH_MMCTIR)"]
    #[inline(always)]
    pub const fn mmctir(&self) -> &Mmctir {
        &self.mmctir
    }
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)"]
    #[inline(always)]
    pub const fn mmcrimr(&self) -> &Mmcrimr {
        &self.mmcrimr
    }
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)"]
    #[inline(always)]
    pub const fn mmctimr(&self) -> &Mmctimr {
        &self.mmctimr
    }
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub const fn mmctgfsccr(&self) -> &Mmctgfsccr {
        &self.mmctgfsccr
    }
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    #[inline(always)]
    pub const fn mmctgfmsccr(&self) -> &Mmctgfmsccr {
        &self.mmctgfmsccr
    }
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"]
    #[inline(always)]
    pub const fn mmctgfcr(&self) -> &Mmctgfcr {
        &self.mmctgfcr
    }
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    #[inline(always)]
    pub const fn mmcrfcecr(&self) -> &Mmcrfcecr {
        &self.mmcrfcecr
    }
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    #[inline(always)]
    pub const fn mmcrfaecr(&self) -> &Mmcrfaecr {
        &self.mmcrfaecr
    }
    #[doc = "0xc4 - MMC received good unicast frames counter register"]
    #[inline(always)]
    pub const fn mmcrgufcr(&self) -> &Mmcrgufcr {
        &self.mmcrgufcr
    }
}
#[doc = "MMCCR (rw) register accessor: Ethernet MMC control register (ETH_MMCCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmccr`]
module"]
#[doc(alias = "MMCCR")]
pub type Mmccr = crate::Reg<mmccr::MmccrSpec>;
#[doc = "Ethernet MMC control register (ETH_MMCCR)"]
pub mod mmccr;
#[doc = "MMCRIR (rw) register accessor: Ethernet MMC receive interrupt register (ETH_MMCRIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrir`]
module"]
#[doc(alias = "MMCRIR")]
pub type Mmcrir = crate::Reg<mmcrir::MmcrirSpec>;
#[doc = "Ethernet MMC receive interrupt register (ETH_MMCRIR)"]
pub mod mmcrir;
#[doc = "MMCTIR (rw) register accessor: Ethernet MMC transmit interrupt register (ETH_MMCTIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctir`]
module"]
#[doc(alias = "MMCTIR")]
pub type Mmctir = crate::Reg<mmctir::MmctirSpec>;
#[doc = "Ethernet MMC transmit interrupt register (ETH_MMCTIR)"]
pub mod mmctir;
#[doc = "MMCRIMR (rw) register accessor: Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrimr`]
module"]
#[doc(alias = "MMCRIMR")]
pub type Mmcrimr = crate::Reg<mmcrimr::MmcrimrSpec>;
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)"]
pub mod mmcrimr;
#[doc = "MMCTIMR (rw) register accessor: Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctimr`]
module"]
#[doc(alias = "MMCTIMR")]
pub type Mmctimr = crate::Reg<mmctimr::MmctimrSpec>;
#[doc = "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)"]
pub mod mmctimr;
#[doc = "MMCTGFSCCR (r) register accessor: Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfsccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctgfsccr`]
module"]
#[doc(alias = "MMCTGFSCCR")]
pub type Mmctgfsccr = crate::Reg<mmctgfsccr::MmctgfsccrSpec>;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctgfsccr;
#[doc = "MMCTGFMSCCR (r) register accessor: Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfmsccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctgfmsccr`]
module"]
#[doc(alias = "MMCTGFMSCCR")]
pub type Mmctgfmsccr = crate::Reg<mmctgfmsccr::MmctgfmsccrSpec>;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctgfmsccr;
#[doc = "MMCTGFCR (r) register accessor: Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctgfcr`]
module"]
#[doc(alias = "MMCTGFCR")]
pub type Mmctgfcr = crate::Reg<mmctgfcr::MmctgfcrSpec>;
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctgfcr;
#[doc = "MMCRFCECR (r) register accessor: Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrfcecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrfcecr`]
module"]
#[doc(alias = "MMCRFCECR")]
pub type Mmcrfcecr = crate::Reg<mmcrfcecr::MmcrfcecrSpec>;
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecr;
#[doc = "MMCRFAECR (r) register accessor: Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrfaecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrfaecr`]
module"]
#[doc(alias = "MMCRFAECR")]
pub type Mmcrfaecr = crate::Reg<mmcrfaecr::MmcrfaecrSpec>;
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecr;
#[doc = "MMCRGUFCR (r) register accessor: MMC received good unicast frames counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrgufcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrgufcr`]
module"]
#[doc(alias = "MMCRGUFCR")]
pub type Mmcrgufcr = crate::Reg<mmcrgufcr::MmcrgufcrSpec>;
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcr;
