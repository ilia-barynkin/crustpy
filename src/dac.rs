#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    swtrigr: Swtrigr,
    dhr12r1: Dhr12r1,
    dhr12l1: Dhr12l1,
    dhr8r1: Dhr8r1,
    dhr12r2: Dhr12r2,
    dhr12l2: Dhr12l2,
    dhr8r2: Dhr8r2,
    dhr12rd: Dhr12rd,
    dhr12ld: Dhr12ld,
    dhr8rd: Dhr8rd,
    dor1: Dor1,
    dor2: Dor2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register (DAC_CR)"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DAC software trigger register (DAC_SWTRIGR)"]
    #[inline(always)]
    pub const fn swtrigr(&self) -> &Swtrigr {
        &self.swtrigr
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)"]
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &Dhr12r1 {
        &self.dhr12r1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)"]
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &Dhr12l1 {
        &self.dhr12l1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)"]
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &Dhr8r1 {
        &self.dhr8r1
    }
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)"]
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &Dhr12r2 {
        &self.dhr12r2
    }
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)"]
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &Dhr12l2 {
        &self.dhr12l2
    }
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)"]
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &Dhr8r2 {
        &self.dhr8r2
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved"]
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &Dhr12rd {
        &self.dhr12rd
    }
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved"]
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &Dhr12ld {
        &self.dhr12ld
    }
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved"]
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &Dhr8rd {
        &self.dhr8rd
    }
    #[doc = "0x2c - DAC channel1 data output register (DAC_DOR1)"]
    #[inline(always)]
    pub const fn dor1(&self) -> &Dor1 {
        &self.dor1
    }
    #[doc = "0x30 - DAC channel2 data output register (DAC_DOR2)"]
    #[inline(always)]
    pub const fn dor2(&self) -> &Dor2 {
        &self.dor2
    }
}
#[doc = "CR (rw) register accessor: Control register (DAC_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register (DAC_CR)"]
pub mod cr;
#[doc = "SWTRIGR (w) register accessor: DAC software trigger register (DAC_SWTRIGR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrigr`]
module"]
#[doc(alias = "SWTRIGR")]
pub type Swtrigr = crate::Reg<swtrigr::SwtrigrSpec>;
#[doc = "DAC software trigger register (DAC_SWTRIGR)"]
pub mod swtrigr;
#[doc = "DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r1`]
module"]
#[doc(alias = "DHR12R1")]
pub type Dhr12r1 = crate::Reg<dhr12r1::Dhr12r1Spec>;
#[doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l1`]
module"]
#[doc(alias = "DHR12L1")]
pub type Dhr12l1 = crate::Reg<dhr12l1::Dhr12l1Spec>;
#[doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r1`]
module"]
#[doc(alias = "DHR8R1")]
pub type Dhr8r1 = crate::Reg<dhr8r1::Dhr8r1Spec>;
#[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)"]
pub mod dhr8r1;
#[doc = "DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r2`]
module"]
#[doc(alias = "DHR12R2")]
pub type Dhr12r2 = crate::Reg<dhr12r2::Dhr12r2Spec>;
#[doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)"]
pub mod dhr12r2;
#[doc = "DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l2`]
module"]
#[doc(alias = "DHR12L2")]
pub type Dhr12l2 = crate::Reg<dhr12l2::Dhr12l2Spec>;
#[doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)"]
pub mod dhr12l2;
#[doc = "DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r2`]
module"]
#[doc(alias = "DHR8R2")]
pub type Dhr8r2 = crate::Reg<dhr8r2::Dhr8r2Spec>;
#[doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)"]
pub mod dhr8r2;
#[doc = "DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12rd`]
module"]
#[doc(alias = "DHR12RD")]
pub type Dhr12rd = crate::Reg<dhr12rd::Dhr12rdSpec>;
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12ld`]
module"]
#[doc(alias = "DHR12LD")]
pub type Dhr12ld = crate::Reg<dhr12ld::Dhr12ldSpec>;
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8rd`]
module"]
#[doc(alias = "DHR8RD")]
pub type Dhr8rd = crate::Reg<dhr8rd::Dhr8rdSpec>;
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: DAC channel1 data output register (DAC_DOR1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`]
module"]
#[doc(alias = "DOR1")]
pub type Dor1 = crate::Reg<dor1::Dor1Spec>;
#[doc = "DAC channel1 data output register (DAC_DOR1)"]
pub mod dor1;
#[doc = "DOR2 (r) register accessor: DAC channel2 data output register (DAC_DOR2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor2`]
module"]
#[doc(alias = "DOR2")]
pub type Dor2 = crate::Reg<dor2::Dor2Spec>;
#[doc = "DAC channel2 data output register (DAC_DOR2)"]
pub mod dor2;
