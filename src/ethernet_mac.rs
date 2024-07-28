#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    maccr: Maccr,
    macffr: Macffr,
    machthr: Machthr,
    machtlr: Machtlr,
    macmiiar: Macmiiar,
    macmiidr: Macmiidr,
    macfcr: Macfcr,
    macvlantr: Macvlantr,
    _reserved8: [u8; 0x08],
    macrwuffr: Macrwuffr,
    macpmtcsr: Macpmtcsr,
    _reserved10: [u8; 0x08],
    macsr: Macsr,
    macimr: Macimr,
    maca0hr: Maca0hr,
    maca0lr: Maca0lr,
    maca1hr: Maca1hr,
    maca1lr: Maca1lr,
    maca2hr: Maca2hr,
    maca2lr: Maca2lr,
    maca3hr: Maca3hr,
    maca3lr: Maca3lr,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register (ETH_MACCR)"]
    #[inline(always)]
    pub const fn maccr(&self) -> &Maccr {
        &self.maccr
    }
    #[doc = "0x04 - Ethernet MAC frame filter register (ETH_MACCFFR)"]
    #[inline(always)]
    pub const fn macffr(&self) -> &Macffr {
        &self.macffr
    }
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    #[inline(always)]
    pub const fn machthr(&self) -> &Machthr {
        &self.machthr
    }
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    #[inline(always)]
    pub const fn machtlr(&self) -> &Machtlr {
        &self.machtlr
    }
    #[doc = "0x10 - Ethernet MAC MII address register (ETH_MACMIIAR)"]
    #[inline(always)]
    pub const fn macmiiar(&self) -> &Macmiiar {
        &self.macmiiar
    }
    #[doc = "0x14 - Ethernet MAC MII data register (ETH_MACMIIDR)"]
    #[inline(always)]
    pub const fn macmiidr(&self) -> &Macmiidr {
        &self.macmiidr
    }
    #[doc = "0x18 - Ethernet MAC flow control register (ETH_MACFCR)"]
    #[inline(always)]
    pub const fn macfcr(&self) -> &Macfcr {
        &self.macfcr
    }
    #[doc = "0x1c - Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
    #[inline(always)]
    pub const fn macvlantr(&self) -> &Macvlantr {
        &self.macvlantr
    }
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
    #[inline(always)]
    pub const fn macrwuffr(&self) -> &Macrwuffr {
        &self.macrwuffr
    }
    #[doc = "0x2c - Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
    #[inline(always)]
    pub const fn macpmtcsr(&self) -> &Macpmtcsr {
        &self.macpmtcsr
    }
    #[doc = "0x38 - Ethernet MAC interrupt status register (ETH_MACSR)"]
    #[inline(always)]
    pub const fn macsr(&self) -> &Macsr {
        &self.macsr
    }
    #[doc = "0x3c - Ethernet MAC interrupt mask register (ETH_MACIMR)"]
    #[inline(always)]
    pub const fn macimr(&self) -> &Macimr {
        &self.macimr
    }
    #[doc = "0x40 - Ethernet MAC address 0 high register (ETH_MACA0HR)"]
    #[inline(always)]
    pub const fn maca0hr(&self) -> &Maca0hr {
        &self.maca0hr
    }
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(&self) -> &Maca0lr {
        &self.maca0lr
    }
    #[doc = "0x48 - Ethernet MAC address 1 high register (ETH_MACA1HR)"]
    #[inline(always)]
    pub const fn maca1hr(&self) -> &Maca1hr {
        &self.maca1hr
    }
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    #[inline(always)]
    pub const fn maca1lr(&self) -> &Maca1lr {
        &self.maca1lr
    }
    #[doc = "0x50 - Ethernet MAC address 2 high register (ETH_MACA2HR)"]
    #[inline(always)]
    pub const fn maca2hr(&self) -> &Maca2hr {
        &self.maca2hr
    }
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    #[inline(always)]
    pub const fn maca2lr(&self) -> &Maca2lr {
        &self.maca2lr
    }
    #[doc = "0x58 - Ethernet MAC address 3 high register (ETH_MACA3HR)"]
    #[inline(always)]
    pub const fn maca3hr(&self) -> &Maca3hr {
        &self.maca3hr
    }
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    #[inline(always)]
    pub const fn maca3lr(&self) -> &Maca3lr {
        &self.maca3lr
    }
}
#[doc = "MACCR (rw) register accessor: Ethernet MAC configuration register (ETH_MACCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maccr`]
module"]
#[doc(alias = "MACCR")]
pub type Maccr = crate::Reg<maccr::MaccrSpec>;
#[doc = "Ethernet MAC configuration register (ETH_MACCR)"]
pub mod maccr;
#[doc = "MACFFR (rw) register accessor: Ethernet MAC frame filter register (ETH_MACCFFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macffr`]
module"]
#[doc(alias = "MACFFR")]
pub type Macffr = crate::Reg<macffr::MacffrSpec>;
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)"]
pub mod macffr;
#[doc = "MACHTHR (rw) register accessor: Ethernet MAC hash table high register\n\nYou can [`read`](crate::Reg::read) this register and get [`machthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machthr`]
module"]
#[doc(alias = "MACHTHR")]
pub type Machthr = crate::Reg<machthr::MachthrSpec>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "MACHTLR (rw) register accessor: Ethernet MAC hash table low register\n\nYou can [`read`](crate::Reg::read) this register and get [`machtlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machtlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machtlr`]
module"]
#[doc(alias = "MACHTLR")]
pub type Machtlr = crate::Reg<machtlr::MachtlrSpec>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "MACMIIAR (rw) register accessor: Ethernet MAC MII address register (ETH_MACMIIAR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiiar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiiar`]
module"]
#[doc(alias = "MACMIIAR")]
pub type Macmiiar = crate::Reg<macmiiar::MacmiiarSpec>;
#[doc = "Ethernet MAC MII address register (ETH_MACMIIAR)"]
pub mod macmiiar;
#[doc = "MACMIIDR (rw) register accessor: Ethernet MAC MII data register (ETH_MACMIIDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiidr`]
module"]
#[doc(alias = "MACMIIDR")]
pub type Macmiidr = crate::Reg<macmiidr::MacmiidrSpec>;
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)"]
pub mod macmiidr;
#[doc = "MACFCR (rw) register accessor: Ethernet MAC flow control register (ETH_MACFCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macfcr`]
module"]
#[doc(alias = "MACFCR")]
pub type Macfcr = crate::Reg<macfcr::MacfcrSpec>;
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)"]
pub mod macfcr;
#[doc = "MACVLANTR (rw) register accessor: Ethernet MAC VLAN tag register (ETH_MACVLANTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macvlantr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvlantr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvlantr`]
module"]
#[doc(alias = "MACVLANTR")]
pub type Macvlantr = crate::Reg<macvlantr::MacvlantrSpec>;
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
pub mod macvlantr;
#[doc = "MACRWUFFR (rw) register accessor: Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macrwuffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwuffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrwuffr`]
module"]
#[doc(alias = "MACRWUFFR")]
pub type Macrwuffr = crate::Reg<macrwuffr::MacrwuffrSpec>;
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
pub mod macrwuffr;
#[doc = "MACPMTCSR (rw) register accessor: Ethernet MAC PMT control and status register (ETH_MACPMTCSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macpmtcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpmtcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpmtcsr`]
module"]
#[doc(alias = "MACPMTCSR")]
pub type Macpmtcsr = crate::Reg<macpmtcsr::MacpmtcsrSpec>;
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
pub mod macpmtcsr;
#[doc = "MACSR (rw) register accessor: Ethernet MAC interrupt status register (ETH_MACSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macsr`]
module"]
#[doc(alias = "MACSR")]
pub type Macsr = crate::Reg<macsr::MacsrSpec>;
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)"]
pub mod macsr;
#[doc = "MACIMR (rw) register accessor: Ethernet MAC interrupt mask register (ETH_MACIMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macimr`]
module"]
#[doc(alias = "MACIMR")]
pub type Macimr = crate::Reg<macimr::MacimrSpec>;
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)"]
pub mod macimr;
#[doc = "MACA0HR (rw) register accessor: Ethernet MAC address 0 high register (ETH_MACA0HR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0hr`]
module"]
#[doc(alias = "MACA0HR")]
pub type Maca0hr = crate::Reg<maca0hr::Maca0hrSpec>;
#[doc = "Ethernet MAC address 0 high register (ETH_MACA0HR)"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0lr`]
module"]
#[doc(alias = "MACA0LR")]
pub type Maca0lr = crate::Reg<maca0lr::Maca0lrSpec>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR (rw) register accessor: Ethernet MAC address 1 high register (ETH_MACA1HR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1hr`]
module"]
#[doc(alias = "MACA1HR")]
pub type Maca1hr = crate::Reg<maca1hr::Maca1hrSpec>;
#[doc = "Ethernet MAC address 1 high register (ETH_MACA1HR)"]
pub mod maca1hr;
#[doc = "MACA1LR (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1lr`]
module"]
#[doc(alias = "MACA1LR")]
pub type Maca1lr = crate::Reg<maca1lr::Maca1lrSpec>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR (rw) register accessor: Ethernet MAC address 2 high register (ETH_MACA2HR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2hr`]
module"]
#[doc(alias = "MACA2HR")]
pub type Maca2hr = crate::Reg<maca2hr::Maca2hrSpec>;
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)"]
pub mod maca2hr;
#[doc = "MACA2LR (rw) register accessor: Ethernet MAC address 2 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2lr`]
module"]
#[doc(alias = "MACA2LR")]
pub type Maca2lr = crate::Reg<maca2lr::Maca2lrSpec>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3HR (rw) register accessor: Ethernet MAC address 3 high register (ETH_MACA3HR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maca3hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3hr`]
module"]
#[doc(alias = "MACA3HR")]
pub type Maca3hr = crate::Reg<maca3hr::Maca3hrSpec>;
#[doc = "Ethernet MAC address 3 high register (ETH_MACA3HR)"]
pub mod maca3hr;
#[doc = "MACA3LR (rw) register accessor: Ethernet MAC address 3 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca3lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3lr`]
module"]
#[doc(alias = "MACA3LR")]
pub type Maca3lr = crate::Reg<maca3lr::Maca3lrSpec>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
