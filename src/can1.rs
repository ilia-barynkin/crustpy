#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    can_mcr: CanMcr,
    can_msr: CanMsr,
    can_tsr: CanTsr,
    can_rf0r: CanRf0r,
    can_rf1r: CanRf1r,
    can_ier: CanIer,
    can_esr: CanEsr,
    can_btr: CanBtr,
    _reserved8: [u8; 0x0160],
    can_ti0r: CanTi0r,
    can_tdt0r: CanTdt0r,
    can_tdl0r: CanTdl0r,
    can_tdh0r: CanTdh0r,
    can_ti1r: CanTi1r,
    can_tdt1r: CanTdt1r,
    can_tdl1r: CanTdl1r,
    can_tdh1r: CanTdh1r,
    can_ti2r: CanTi2r,
    can_tdt2r: CanTdt2r,
    can_tdl2r: CanTdl2r,
    can_tdh2r: CanTdh2r,
    can_ri0r: CanRi0r,
    can_rdt0r: CanRdt0r,
    can_rdl0r: CanRdl0r,
    can_rdh0r: CanRdh0r,
    can_ri1r: CanRi1r,
    can_rdt1r: CanRdt1r,
    can_rdl1r: CanRdl1r,
    can_rdh1r: CanRdh1r,
    _reserved28: [u8; 0x30],
    can_fmr: CanFmr,
    can_fm1r: CanFm1r,
    _reserved30: [u8; 0x04],
    can_fs1r: CanFs1r,
    _reserved31: [u8; 0x04],
    can_ffa1r: CanFfa1r,
    _reserved32: [u8; 0x04],
    can_fa1r: CanFa1r,
    _reserved33: [u8; 0x20],
    f0r1: F0r1,
    f0r2: F0r2,
    f1r1: F1r1,
    f1r2: F1r2,
    f2r1: F2r1,
    f2r2: F2r2,
    f3r1: F3r1,
    f3r2: F3r2,
    f4r1: F4r1,
    f4r2: F4r2,
    f5r1: F5r1,
    f5r2: F5r2,
    f6r1: F6r1,
    f6r2: F6r2,
    f7r1: F7r1,
    f7r2: F7r2,
    f8r1: F8r1,
    f8r2: F8r2,
    f9r1: F9r1,
    f9r2: F9r2,
    f10r1: F10r1,
    f10r2: F10r2,
    f11r1: F11r1,
    f11r2: F11r2,
    f12r1: F12r1,
    f12r2: F12r2,
    f13r1: F13r1,
    f13r2: F13r2,
}
impl RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    #[inline(always)]
    pub const fn can_mcr(&self) -> &CanMcr {
        &self.can_mcr
    }
    #[doc = "0x04 - CAN_MSR"]
    #[inline(always)]
    pub const fn can_msr(&self) -> &CanMsr {
        &self.can_msr
    }
    #[doc = "0x08 - CAN_TSR"]
    #[inline(always)]
    pub const fn can_tsr(&self) -> &CanTsr {
        &self.can_tsr
    }
    #[doc = "0x0c - CAN_RF0R"]
    #[inline(always)]
    pub const fn can_rf0r(&self) -> &CanRf0r {
        &self.can_rf0r
    }
    #[doc = "0x10 - CAN_RF1R"]
    #[inline(always)]
    pub const fn can_rf1r(&self) -> &CanRf1r {
        &self.can_rf1r
    }
    #[doc = "0x14 - CAN_IER"]
    #[inline(always)]
    pub const fn can_ier(&self) -> &CanIer {
        &self.can_ier
    }
    #[doc = "0x18 - CAN_ESR"]
    #[inline(always)]
    pub const fn can_esr(&self) -> &CanEsr {
        &self.can_esr
    }
    #[doc = "0x1c - CAN_BTR"]
    #[inline(always)]
    pub const fn can_btr(&self) -> &CanBtr {
        &self.can_btr
    }
    #[doc = "0x180 - CAN_TI0R"]
    #[inline(always)]
    pub const fn can_ti0r(&self) -> &CanTi0r {
        &self.can_ti0r
    }
    #[doc = "0x184 - CAN_TDT0R"]
    #[inline(always)]
    pub const fn can_tdt0r(&self) -> &CanTdt0r {
        &self.can_tdt0r
    }
    #[doc = "0x188 - CAN_TDL0R"]
    #[inline(always)]
    pub const fn can_tdl0r(&self) -> &CanTdl0r {
        &self.can_tdl0r
    }
    #[doc = "0x18c - CAN_TDH0R"]
    #[inline(always)]
    pub const fn can_tdh0r(&self) -> &CanTdh0r {
        &self.can_tdh0r
    }
    #[doc = "0x190 - CAN_TI1R"]
    #[inline(always)]
    pub const fn can_ti1r(&self) -> &CanTi1r {
        &self.can_ti1r
    }
    #[doc = "0x194 - CAN_TDT1R"]
    #[inline(always)]
    pub const fn can_tdt1r(&self) -> &CanTdt1r {
        &self.can_tdt1r
    }
    #[doc = "0x198 - CAN_TDL1R"]
    #[inline(always)]
    pub const fn can_tdl1r(&self) -> &CanTdl1r {
        &self.can_tdl1r
    }
    #[doc = "0x19c - CAN_TDH1R"]
    #[inline(always)]
    pub const fn can_tdh1r(&self) -> &CanTdh1r {
        &self.can_tdh1r
    }
    #[doc = "0x1a0 - CAN_TI2R"]
    #[inline(always)]
    pub const fn can_ti2r(&self) -> &CanTi2r {
        &self.can_ti2r
    }
    #[doc = "0x1a4 - CAN_TDT2R"]
    #[inline(always)]
    pub const fn can_tdt2r(&self) -> &CanTdt2r {
        &self.can_tdt2r
    }
    #[doc = "0x1a8 - CAN_TDL2R"]
    #[inline(always)]
    pub const fn can_tdl2r(&self) -> &CanTdl2r {
        &self.can_tdl2r
    }
    #[doc = "0x1ac - CAN_TDH2R"]
    #[inline(always)]
    pub const fn can_tdh2r(&self) -> &CanTdh2r {
        &self.can_tdh2r
    }
    #[doc = "0x1b0 - CAN_RI0R"]
    #[inline(always)]
    pub const fn can_ri0r(&self) -> &CanRi0r {
        &self.can_ri0r
    }
    #[doc = "0x1b4 - CAN_RDT0R"]
    #[inline(always)]
    pub const fn can_rdt0r(&self) -> &CanRdt0r {
        &self.can_rdt0r
    }
    #[doc = "0x1b8 - CAN_RDL0R"]
    #[inline(always)]
    pub const fn can_rdl0r(&self) -> &CanRdl0r {
        &self.can_rdl0r
    }
    #[doc = "0x1bc - CAN_RDH0R"]
    #[inline(always)]
    pub const fn can_rdh0r(&self) -> &CanRdh0r {
        &self.can_rdh0r
    }
    #[doc = "0x1c0 - CAN_RI1R"]
    #[inline(always)]
    pub const fn can_ri1r(&self) -> &CanRi1r {
        &self.can_ri1r
    }
    #[doc = "0x1c4 - CAN_RDT1R"]
    #[inline(always)]
    pub const fn can_rdt1r(&self) -> &CanRdt1r {
        &self.can_rdt1r
    }
    #[doc = "0x1c8 - CAN_RDL1R"]
    #[inline(always)]
    pub const fn can_rdl1r(&self) -> &CanRdl1r {
        &self.can_rdl1r
    }
    #[doc = "0x1cc - CAN_RDH1R"]
    #[inline(always)]
    pub const fn can_rdh1r(&self) -> &CanRdh1r {
        &self.can_rdh1r
    }
    #[doc = "0x200 - CAN_FMR"]
    #[inline(always)]
    pub const fn can_fmr(&self) -> &CanFmr {
        &self.can_fmr
    }
    #[doc = "0x204 - CAN_FM1R"]
    #[inline(always)]
    pub const fn can_fm1r(&self) -> &CanFm1r {
        &self.can_fm1r
    }
    #[doc = "0x20c - CAN_FS1R"]
    #[inline(always)]
    pub const fn can_fs1r(&self) -> &CanFs1r {
        &self.can_fs1r
    }
    #[doc = "0x214 - CAN_FFA1R"]
    #[inline(always)]
    pub const fn can_ffa1r(&self) -> &CanFfa1r {
        &self.can_ffa1r
    }
    #[doc = "0x21c - CAN_FA1R"]
    #[inline(always)]
    pub const fn can_fa1r(&self) -> &CanFa1r {
        &self.can_fa1r
    }
    #[doc = "0x240 - Filter bank 0 register 1"]
    #[inline(always)]
    pub const fn f0r1(&self) -> &F0r1 {
        &self.f0r1
    }
    #[doc = "0x244 - Filter bank 0 register 2"]
    #[inline(always)]
    pub const fn f0r2(&self) -> &F0r2 {
        &self.f0r2
    }
    #[doc = "0x248 - Filter bank 1 register 1"]
    #[inline(always)]
    pub const fn f1r1(&self) -> &F1r1 {
        &self.f1r1
    }
    #[doc = "0x24c - Filter bank 1 register 2"]
    #[inline(always)]
    pub const fn f1r2(&self) -> &F1r2 {
        &self.f1r2
    }
    #[doc = "0x250 - Filter bank 2 register 1"]
    #[inline(always)]
    pub const fn f2r1(&self) -> &F2r1 {
        &self.f2r1
    }
    #[doc = "0x254 - Filter bank 2 register 2"]
    #[inline(always)]
    pub const fn f2r2(&self) -> &F2r2 {
        &self.f2r2
    }
    #[doc = "0x258 - Filter bank 3 register 1"]
    #[inline(always)]
    pub const fn f3r1(&self) -> &F3r1 {
        &self.f3r1
    }
    #[doc = "0x25c - Filter bank 3 register 2"]
    #[inline(always)]
    pub const fn f3r2(&self) -> &F3r2 {
        &self.f3r2
    }
    #[doc = "0x260 - Filter bank 4 register 1"]
    #[inline(always)]
    pub const fn f4r1(&self) -> &F4r1 {
        &self.f4r1
    }
    #[doc = "0x264 - Filter bank 4 register 2"]
    #[inline(always)]
    pub const fn f4r2(&self) -> &F4r2 {
        &self.f4r2
    }
    #[doc = "0x268 - Filter bank 5 register 1"]
    #[inline(always)]
    pub const fn f5r1(&self) -> &F5r1 {
        &self.f5r1
    }
    #[doc = "0x26c - Filter bank 5 register 2"]
    #[inline(always)]
    pub const fn f5r2(&self) -> &F5r2 {
        &self.f5r2
    }
    #[doc = "0x270 - Filter bank 6 register 1"]
    #[inline(always)]
    pub const fn f6r1(&self) -> &F6r1 {
        &self.f6r1
    }
    #[doc = "0x274 - Filter bank 6 register 2"]
    #[inline(always)]
    pub const fn f6r2(&self) -> &F6r2 {
        &self.f6r2
    }
    #[doc = "0x278 - Filter bank 7 register 1"]
    #[inline(always)]
    pub const fn f7r1(&self) -> &F7r1 {
        &self.f7r1
    }
    #[doc = "0x27c - Filter bank 7 register 2"]
    #[inline(always)]
    pub const fn f7r2(&self) -> &F7r2 {
        &self.f7r2
    }
    #[doc = "0x280 - Filter bank 8 register 1"]
    #[inline(always)]
    pub const fn f8r1(&self) -> &F8r1 {
        &self.f8r1
    }
    #[doc = "0x284 - Filter bank 8 register 2"]
    #[inline(always)]
    pub const fn f8r2(&self) -> &F8r2 {
        &self.f8r2
    }
    #[doc = "0x288 - Filter bank 9 register 1"]
    #[inline(always)]
    pub const fn f9r1(&self) -> &F9r1 {
        &self.f9r1
    }
    #[doc = "0x28c - Filter bank 9 register 2"]
    #[inline(always)]
    pub const fn f9r2(&self) -> &F9r2 {
        &self.f9r2
    }
    #[doc = "0x290 - Filter bank 10 register 1"]
    #[inline(always)]
    pub const fn f10r1(&self) -> &F10r1 {
        &self.f10r1
    }
    #[doc = "0x294 - Filter bank 10 register 2"]
    #[inline(always)]
    pub const fn f10r2(&self) -> &F10r2 {
        &self.f10r2
    }
    #[doc = "0x298 - Filter bank 11 register 1"]
    #[inline(always)]
    pub const fn f11r1(&self) -> &F11r1 {
        &self.f11r1
    }
    #[doc = "0x29c - Filter bank 11 register 2"]
    #[inline(always)]
    pub const fn f11r2(&self) -> &F11r2 {
        &self.f11r2
    }
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    #[inline(always)]
    pub const fn f12r1(&self) -> &F12r1 {
        &self.f12r1
    }
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    #[inline(always)]
    pub const fn f12r2(&self) -> &F12r2 {
        &self.f12r2
    }
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    #[inline(always)]
    pub const fn f13r1(&self) -> &F13r1 {
        &self.f13r1
    }
    #[doc = "0x2ac - Filter bank 13 register 2"]
    #[inline(always)]
    pub const fn f13r2(&self) -> &F13r2 {
        &self.f13r2
    }
}
#[doc = "CAN_MCR (rw) register accessor: CAN_MCR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_mcr`]
module"]
#[doc(alias = "CAN_MCR")]
pub type CanMcr = crate::Reg<can_mcr::CanMcrSpec>;
#[doc = "CAN_MCR"]
pub mod can_mcr;
#[doc = "CAN_MSR (rw) register accessor: CAN_MSR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_msr`]
module"]
#[doc(alias = "CAN_MSR")]
pub type CanMsr = crate::Reg<can_msr::CanMsrSpec>;
#[doc = "CAN_MSR"]
pub mod can_msr;
#[doc = "CAN_TSR (rw) register accessor: CAN_TSR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tsr`]
module"]
#[doc(alias = "CAN_TSR")]
pub type CanTsr = crate::Reg<can_tsr::CanTsrSpec>;
#[doc = "CAN_TSR"]
pub mod can_tsr;
#[doc = "CAN_RF0R (rw) register accessor: CAN_RF0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rf0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_rf0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rf0r`]
module"]
#[doc(alias = "CAN_RF0R")]
pub type CanRf0r = crate::Reg<can_rf0r::CanRf0rSpec>;
#[doc = "CAN_RF0R"]
pub mod can_rf0r;
#[doc = "CAN_RF1R (rw) register accessor: CAN_RF1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rf1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_rf1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rf1r`]
module"]
#[doc(alias = "CAN_RF1R")]
pub type CanRf1r = crate::Reg<can_rf1r::CanRf1rSpec>;
#[doc = "CAN_RF1R"]
pub mod can_rf1r;
#[doc = "CAN_IER (rw) register accessor: CAN_IER\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ier`]
module"]
#[doc(alias = "CAN_IER")]
pub type CanIer = crate::Reg<can_ier::CanIerSpec>;
#[doc = "CAN_IER"]
pub mod can_ier;
#[doc = "CAN_ESR (rw) register accessor: CAN_ESR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_esr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_esr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_esr`]
module"]
#[doc(alias = "CAN_ESR")]
pub type CanEsr = crate::Reg<can_esr::CanEsrSpec>;
#[doc = "CAN_ESR"]
pub mod can_esr;
#[doc = "CAN_BTR (rw) register accessor: CAN_BTR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_btr`]
module"]
#[doc(alias = "CAN_BTR")]
pub type CanBtr = crate::Reg<can_btr::CanBtrSpec>;
#[doc = "CAN_BTR"]
pub mod can_btr;
#[doc = "CAN_TI0R (rw) register accessor: CAN_TI0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ti0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ti0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ti0r`]
module"]
#[doc(alias = "CAN_TI0R")]
pub type CanTi0r = crate::Reg<can_ti0r::CanTi0rSpec>;
#[doc = "CAN_TI0R"]
pub mod can_ti0r;
#[doc = "CAN_TDT0R (rw) register accessor: CAN_TDT0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdt0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdt0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdt0r`]
module"]
#[doc(alias = "CAN_TDT0R")]
pub type CanTdt0r = crate::Reg<can_tdt0r::CanTdt0rSpec>;
#[doc = "CAN_TDT0R"]
pub mod can_tdt0r;
#[doc = "CAN_TDL0R (rw) register accessor: CAN_TDL0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdl0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdl0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdl0r`]
module"]
#[doc(alias = "CAN_TDL0R")]
pub type CanTdl0r = crate::Reg<can_tdl0r::CanTdl0rSpec>;
#[doc = "CAN_TDL0R"]
pub mod can_tdl0r;
#[doc = "CAN_TDH0R (rw) register accessor: CAN_TDH0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdh0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdh0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdh0r`]
module"]
#[doc(alias = "CAN_TDH0R")]
pub type CanTdh0r = crate::Reg<can_tdh0r::CanTdh0rSpec>;
#[doc = "CAN_TDH0R"]
pub mod can_tdh0r;
#[doc = "CAN_TI1R (rw) register accessor: CAN_TI1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ti1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ti1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ti1r`]
module"]
#[doc(alias = "CAN_TI1R")]
pub type CanTi1r = crate::Reg<can_ti1r::CanTi1rSpec>;
#[doc = "CAN_TI1R"]
pub mod can_ti1r;
#[doc = "CAN_TDT1R (rw) register accessor: CAN_TDT1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdt1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdt1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdt1r`]
module"]
#[doc(alias = "CAN_TDT1R")]
pub type CanTdt1r = crate::Reg<can_tdt1r::CanTdt1rSpec>;
#[doc = "CAN_TDT1R"]
pub mod can_tdt1r;
#[doc = "CAN_TDL1R (rw) register accessor: CAN_TDL1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdl1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdl1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdl1r`]
module"]
#[doc(alias = "CAN_TDL1R")]
pub type CanTdl1r = crate::Reg<can_tdl1r::CanTdl1rSpec>;
#[doc = "CAN_TDL1R"]
pub mod can_tdl1r;
#[doc = "CAN_TDH1R (rw) register accessor: CAN_TDH1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdh1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdh1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdh1r`]
module"]
#[doc(alias = "CAN_TDH1R")]
pub type CanTdh1r = crate::Reg<can_tdh1r::CanTdh1rSpec>;
#[doc = "CAN_TDH1R"]
pub mod can_tdh1r;
#[doc = "CAN_TI2R (rw) register accessor: CAN_TI2R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ti2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ti2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ti2r`]
module"]
#[doc(alias = "CAN_TI2R")]
pub type CanTi2r = crate::Reg<can_ti2r::CanTi2rSpec>;
#[doc = "CAN_TI2R"]
pub mod can_ti2r;
#[doc = "CAN_TDT2R (rw) register accessor: CAN_TDT2R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdt2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdt2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdt2r`]
module"]
#[doc(alias = "CAN_TDT2R")]
pub type CanTdt2r = crate::Reg<can_tdt2r::CanTdt2rSpec>;
#[doc = "CAN_TDT2R"]
pub mod can_tdt2r;
#[doc = "CAN_TDL2R (rw) register accessor: CAN_TDL2R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdl2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdl2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdl2r`]
module"]
#[doc(alias = "CAN_TDL2R")]
pub type CanTdl2r = crate::Reg<can_tdl2r::CanTdl2rSpec>;
#[doc = "CAN_TDL2R"]
pub mod can_tdl2r;
#[doc = "CAN_TDH2R (rw) register accessor: CAN_TDH2R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tdh2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_tdh2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdh2r`]
module"]
#[doc(alias = "CAN_TDH2R")]
pub type CanTdh2r = crate::Reg<can_tdh2r::CanTdh2rSpec>;
#[doc = "CAN_TDH2R"]
pub mod can_tdh2r;
#[doc = "CAN_RI0R (r) register accessor: CAN_RI0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ri0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ri0r`]
module"]
#[doc(alias = "CAN_RI0R")]
pub type CanRi0r = crate::Reg<can_ri0r::CanRi0rSpec>;
#[doc = "CAN_RI0R"]
pub mod can_ri0r;
#[doc = "CAN_RDT0R (r) register accessor: CAN_RDT0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdt0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdt0r`]
module"]
#[doc(alias = "CAN_RDT0R")]
pub type CanRdt0r = crate::Reg<can_rdt0r::CanRdt0rSpec>;
#[doc = "CAN_RDT0R"]
pub mod can_rdt0r;
#[doc = "CAN_RDL0R (r) register accessor: CAN_RDL0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdl0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdl0r`]
module"]
#[doc(alias = "CAN_RDL0R")]
pub type CanRdl0r = crate::Reg<can_rdl0r::CanRdl0rSpec>;
#[doc = "CAN_RDL0R"]
pub mod can_rdl0r;
#[doc = "CAN_RDH0R (r) register accessor: CAN_RDH0R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdh0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdh0r`]
module"]
#[doc(alias = "CAN_RDH0R")]
pub type CanRdh0r = crate::Reg<can_rdh0r::CanRdh0rSpec>;
#[doc = "CAN_RDH0R"]
pub mod can_rdh0r;
#[doc = "CAN_RI1R (r) register accessor: CAN_RI1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ri1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ri1r`]
module"]
#[doc(alias = "CAN_RI1R")]
pub type CanRi1r = crate::Reg<can_ri1r::CanRi1rSpec>;
#[doc = "CAN_RI1R"]
pub mod can_ri1r;
#[doc = "CAN_RDT1R (r) register accessor: CAN_RDT1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdt1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdt1r`]
module"]
#[doc(alias = "CAN_RDT1R")]
pub type CanRdt1r = crate::Reg<can_rdt1r::CanRdt1rSpec>;
#[doc = "CAN_RDT1R"]
pub mod can_rdt1r;
#[doc = "CAN_RDL1R (r) register accessor: CAN_RDL1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdl1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdl1r`]
module"]
#[doc(alias = "CAN_RDL1R")]
pub type CanRdl1r = crate::Reg<can_rdl1r::CanRdl1rSpec>;
#[doc = "CAN_RDL1R"]
pub mod can_rdl1r;
#[doc = "CAN_RDH1R (r) register accessor: CAN_RDH1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rdh1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdh1r`]
module"]
#[doc(alias = "CAN_RDH1R")]
pub type CanRdh1r = crate::Reg<can_rdh1r::CanRdh1rSpec>;
#[doc = "CAN_RDH1R"]
pub mod can_rdh1r;
#[doc = "CAN_FMR (rw) register accessor: CAN_FMR\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fmr`]
module"]
#[doc(alias = "CAN_FMR")]
pub type CanFmr = crate::Reg<can_fmr::CanFmrSpec>;
#[doc = "CAN_FMR"]
pub mod can_fmr;
#[doc = "CAN_FM1R (rw) register accessor: CAN_FM1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fm1r`]
module"]
#[doc(alias = "CAN_FM1R")]
pub type CanFm1r = crate::Reg<can_fm1r::CanFm1rSpec>;
#[doc = "CAN_FM1R"]
pub mod can_fm1r;
#[doc = "CAN_FS1R (rw) register accessor: CAN_FS1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fs1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fs1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fs1r`]
module"]
#[doc(alias = "CAN_FS1R")]
pub type CanFs1r = crate::Reg<can_fs1r::CanFs1rSpec>;
#[doc = "CAN_FS1R"]
pub mod can_fs1r;
#[doc = "CAN_FFA1R (rw) register accessor: CAN_FFA1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ffa1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ffa1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ffa1r`]
module"]
#[doc(alias = "CAN_FFA1R")]
pub type CanFfa1r = crate::Reg<can_ffa1r::CanFfa1rSpec>;
#[doc = "CAN_FFA1R"]
pub mod can_ffa1r;
#[doc = "CAN_FA1R (rw) register accessor: CAN_FA1R\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fa1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fa1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fa1r`]
module"]
#[doc(alias = "CAN_FA1R")]
pub type CanFa1r = crate::Reg<can_fa1r::CanFa1rSpec>;
#[doc = "CAN_FA1R"]
pub mod can_fa1r;
#[doc = "F0R1 (rw) register accessor: Filter bank 0 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f0r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f0r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0r1`]
module"]
#[doc(alias = "F0R1")]
pub type F0r1 = crate::Reg<f0r1::F0r1Spec>;
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "F0R2 (rw) register accessor: Filter bank 0 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f0r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f0r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0r2`]
module"]
#[doc(alias = "F0R2")]
pub type F0r2 = crate::Reg<f0r2::F0r2Spec>;
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "F1R1 (rw) register accessor: Filter bank 1 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1r1`]
module"]
#[doc(alias = "F1R1")]
pub type F1r1 = crate::Reg<f1r1::F1r1Spec>;
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "F1R2 (rw) register accessor: Filter bank 1 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1r2`]
module"]
#[doc(alias = "F1R2")]
pub type F1r2 = crate::Reg<f1r2::F1r2Spec>;
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "F2R1 (rw) register accessor: Filter bank 2 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2r1`]
module"]
#[doc(alias = "F2R1")]
pub type F2r1 = crate::Reg<f2r1::F2r1Spec>;
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "F2R2 (rw) register accessor: Filter bank 2 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2r2`]
module"]
#[doc(alias = "F2R2")]
pub type F2r2 = crate::Reg<f2r2::F2r2Spec>;
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "F3R1 (rw) register accessor: Filter bank 3 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f3r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f3r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3r1`]
module"]
#[doc(alias = "F3R1")]
pub type F3r1 = crate::Reg<f3r1::F3r1Spec>;
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "F3R2 (rw) register accessor: Filter bank 3 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f3r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f3r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3r2`]
module"]
#[doc(alias = "F3R2")]
pub type F3r2 = crate::Reg<f3r2::F3r2Spec>;
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "F4R1 (rw) register accessor: Filter bank 4 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f4r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f4r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4r1`]
module"]
#[doc(alias = "F4R1")]
pub type F4r1 = crate::Reg<f4r1::F4r1Spec>;
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "F4R2 (rw) register accessor: Filter bank 4 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f4r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f4r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4r2`]
module"]
#[doc(alias = "F4R2")]
pub type F4r2 = crate::Reg<f4r2::F4r2Spec>;
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "F5R1 (rw) register accessor: Filter bank 5 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f5r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f5r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5r1`]
module"]
#[doc(alias = "F5R1")]
pub type F5r1 = crate::Reg<f5r1::F5r1Spec>;
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "F5R2 (rw) register accessor: Filter bank 5 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f5r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f5r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5r2`]
module"]
#[doc(alias = "F5R2")]
pub type F5r2 = crate::Reg<f5r2::F5r2Spec>;
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "F6R1 (rw) register accessor: Filter bank 6 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f6r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f6r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6r1`]
module"]
#[doc(alias = "F6R1")]
pub type F6r1 = crate::Reg<f6r1::F6r1Spec>;
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "F6R2 (rw) register accessor: Filter bank 6 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f6r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f6r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6r2`]
module"]
#[doc(alias = "F6R2")]
pub type F6r2 = crate::Reg<f6r2::F6r2Spec>;
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "F7R1 (rw) register accessor: Filter bank 7 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f7r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f7r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7r1`]
module"]
#[doc(alias = "F7R1")]
pub type F7r1 = crate::Reg<f7r1::F7r1Spec>;
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "F7R2 (rw) register accessor: Filter bank 7 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f7r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f7r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7r2`]
module"]
#[doc(alias = "F7R2")]
pub type F7r2 = crate::Reg<f7r2::F7r2Spec>;
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "F8R1 (rw) register accessor: Filter bank 8 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8r1`]
module"]
#[doc(alias = "F8R1")]
pub type F8r1 = crate::Reg<f8r1::F8r1Spec>;
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "F8R2 (rw) register accessor: Filter bank 8 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8r2`]
module"]
#[doc(alias = "F8R2")]
pub type F8r2 = crate::Reg<f8r2::F8r2Spec>;
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "F9R1 (rw) register accessor: Filter bank 9 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f9r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f9r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9r1`]
module"]
#[doc(alias = "F9R1")]
pub type F9r1 = crate::Reg<f9r1::F9r1Spec>;
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "F9R2 (rw) register accessor: Filter bank 9 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f9r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f9r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9r2`]
module"]
#[doc(alias = "F9R2")]
pub type F9r2 = crate::Reg<f9r2::F9r2Spec>;
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "F10R1 (rw) register accessor: Filter bank 10 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f10r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f10r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10r1`]
module"]
#[doc(alias = "F10R1")]
pub type F10r1 = crate::Reg<f10r1::F10r1Spec>;
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "F10R2 (rw) register accessor: Filter bank 10 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f10r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f10r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10r2`]
module"]
#[doc(alias = "F10R2")]
pub type F10r2 = crate::Reg<f10r2::F10r2Spec>;
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "F11R1 (rw) register accessor: Filter bank 11 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f11r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f11r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11r1`]
module"]
#[doc(alias = "F11R1")]
pub type F11r1 = crate::Reg<f11r1::F11r1Spec>;
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "F11R2 (rw) register accessor: Filter bank 11 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f11r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f11r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11r2`]
module"]
#[doc(alias = "F11R2")]
pub type F11r2 = crate::Reg<f11r2::F11r2Spec>;
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "F12R1 (rw) register accessor: Filter bank 4 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12r1`]
module"]
#[doc(alias = "F12R1")]
pub type F12r1 = crate::Reg<f12r1::F12r1Spec>;
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "F12R2 (rw) register accessor: Filter bank 12 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12r2`]
module"]
#[doc(alias = "F12R2")]
pub type F12r2 = crate::Reg<f12r2::F12r2Spec>;
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "F13R1 (rw) register accessor: Filter bank 13 register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`f13r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f13r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13r1`]
module"]
#[doc(alias = "F13R1")]
pub type F13r1 = crate::Reg<f13r1::F13r1Spec>;
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "F13R2 (rw) register accessor: Filter bank 13 register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`f13r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f13r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13r2`]
module"]
#[doc(alias = "F13R2")]
pub type F13r2 = crate::Reg<f13r2::F13r2Spec>;
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
