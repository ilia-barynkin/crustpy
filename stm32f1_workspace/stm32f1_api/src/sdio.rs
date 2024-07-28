#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: Power,
    clkcr: Clkcr,
    arg: Arg,
    cmd: Cmd,
    respcmd: Respcmd,
    respi1: Respi1,
    resp2: Resp2,
    resp3: Resp3,
    resp4: Resp4,
    dtimer: Dtimer,
    dlen: Dlen,
    dctrl: Dctrl,
    dcount: Dcount,
    sta: Sta,
    icr: Icr,
    mask: Mask,
    _reserved16: [u8; 0x08],
    fifocnt: Fifocnt,
    _reserved17: [u8; 0x34],
    fifo: Fifo,
}
impl RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCR)"]
    #[inline(always)]
    pub const fn clkcr(&self) -> &Clkcr {
        &self.clkcr
    }
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    #[inline(always)]
    pub const fn arg(&self) -> &Arg {
        &self.arg
    }
    #[doc = "0x0c - SDIO command register (SDIO_CMD)"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - SDIO command register"]
    #[inline(always)]
    pub const fn respcmd(&self) -> &Respcmd {
        &self.respcmd
    }
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    #[inline(always)]
    pub const fn respi1(&self) -> &Respi1 {
        &self.respi1
    }
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    #[inline(always)]
    pub const fn resp4(&self) -> &Resp4 {
        &self.resp4
    }
    #[doc = "0x24 - Bits 31:0 = DATATIME: Data timeout period"]
    #[inline(always)]
    pub const fn dtimer(&self) -> &Dtimer {
        &self.dtimer
    }
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    #[inline(always)]
    pub const fn dlen(&self) -> &Dlen {
        &self.dlen
    }
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    #[inline(always)]
    pub const fn dctrl(&self) -> &Dctrl {
        &self.dctrl
    }
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    #[inline(always)]
    pub const fn dcount(&self) -> &Dcount {
        &self.dcount
    }
    #[doc = "0x34 - SDIO status register (SDIO_STA)"]
    #[inline(always)]
    pub const fn sta(&self) -> &Sta {
        &self.sta
    }
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_ICR)"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x3c - SDIO mask register (SDIO_MASK)"]
    #[inline(always)]
    pub const fn mask(&self) -> &Mask {
        &self.mask
    }
    #[doc = "0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub const fn fifocnt(&self) -> &Fifocnt {
        &self.fifocnt
    }
    #[doc = "0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data"]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
}
#[doc = "POWER (rw) register accessor: Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: SDI clock control register (SDIO_CLKCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
#[doc(alias = "CLKCR")]
pub type Clkcr = crate::Reg<clkcr::ClkcrSpec>;
#[doc = "SDI clock control register (SDIO_CLKCR)"]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: Bits 31:0 = : Command argument\n\nYou can [`read`](crate::Reg::read) this register and get [`arg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg`]
module"]
#[doc(alias = "ARG")]
pub type Arg = crate::Reg<arg::ArgSpec>;
#[doc = "Bits 31:0 = : Command argument"]
pub mod arg;
#[doc = "CMD (rw) register accessor: SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "SDIO command register (SDIO_CMD)"]
pub mod cmd;
#[doc = "RESPCMD (r) register accessor: SDIO command register\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmd`]
module"]
#[doc(alias = "RESPCMD")]
pub type Respcmd = crate::Reg<respcmd::RespcmdSpec>;
#[doc = "SDIO command register"]
pub mod respcmd;
#[doc = "RESPI1 (r) register accessor: Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::Reg::read) this register and get [`respi1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respi1`]
module"]
#[doc(alias = "RESPI1")]
pub type Respi1 = crate::Reg<respi1::Respi1Spec>;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod respi1;
#[doc = "RESP2 (r) register accessor: Bits 31:0 = CARDSTATUS2\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`]
module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod resp3;
#[doc = "RESP4 (r) register accessor: Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4`]
module"]
#[doc(alias = "RESP4")]
pub type Resp4 = crate::Reg<resp4::Resp4Spec>;
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod resp4;
#[doc = "DTIMER (rw) register accessor: Bits 31:0 = DATATIME: Data timeout period\n\nYou can [`read`](crate::Reg::read) this register and get [`dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtimer`]
module"]
#[doc(alias = "DTIMER")]
pub type Dtimer = crate::Reg<dtimer::DtimerSpec>;
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub mod dtimer;
#[doc = "DLEN (rw) register accessor: Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::Reg::read) this register and get [`dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
#[doc(alias = "DLEN")]
pub type Dlen = crate::Reg<dlen::DlenSpec>;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dlen;
#[doc = "DCTRL (rw) register accessor: SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`]
module"]
#[doc(alias = "DCTRL")]
pub type Dctrl = crate::Reg<dctrl::DctrlSpec>;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dctrl;
#[doc = "DCOUNT (r) register accessor: Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::Reg::read) this register and get [`dcount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcount`]
module"]
#[doc(alias = "DCOUNT")]
pub type Dcount = crate::Reg<dcount::DcountSpec>;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dcount;
#[doc = "STA (r) register accessor: SDIO status register (SDIO_STA)\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
#[doc(alias = "STA")]
pub type Sta = crate::Reg<sta::StaSpec>;
#[doc = "SDIO status register (SDIO_STA)"]
pub mod sta;
#[doc = "ICR (rw) register accessor: SDIO interrupt clear register (SDIO_ICR)\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "SDIO interrupt clear register (SDIO_ICR)"]
pub mod icr;
#[doc = "MASK (rw) register accessor: SDIO mask register (SDIO_MASK)\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "SDIO mask register (SDIO_MASK)"]
pub mod mask;
#[doc = "FIFOCNT (r) register accessor: Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocnt`]
module"]
#[doc(alias = "FIFOCNT")]
pub type Fifocnt = crate::Reg<fifocnt::FifocntSpec>;
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod fifocnt;
#[doc = "FIFO (rw) register accessor: bits 31:0 = FIFOData: Receive and transmit FIFO data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub mod fifo;
