#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    load_: Load_,
    val: Val,
    calib: Calib,
}
impl RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - SysTick reload value register"]
    #[inline(always)]
    pub const fn load_(&self) -> &Load_ {
        &self.load_
    }
    #[doc = "0x08 - SysTick current value register"]
    #[inline(always)]
    pub const fn val(&self) -> &Val {
        &self.val
    }
    #[doc = "0x0c - SysTick calibration value register"]
    #[inline(always)]
    pub const fn calib(&self) -> &Calib {
        &self.calib
    }
}
#[doc = "CTRL (rw) register accessor: SysTick control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SysTick control and status register"]
pub mod ctrl;
#[doc = "LOAD_ (rw) register accessor: SysTick reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`load_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load_`]
module"]
#[doc(alias = "LOAD_")]
pub type Load_ = crate::Reg<load_::Load_Spec>;
#[doc = "SysTick reload value register"]
pub mod load_;
#[doc = "VAL (rw) register accessor: SysTick current value register\n\nYou can [`read`](crate::Reg::read) this register and get [`val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@val`]
module"]
#[doc(alias = "VAL")]
pub type Val = crate::Reg<val::ValSpec>;
#[doc = "SysTick current value register"]
pub mod val;
#[doc = "CALIB (rw) register accessor: SysTick calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
#[doc(alias = "CALIB")]
pub type Calib = crate::Reg<calib::CalibSpec>;
#[doc = "SysTick calibration value register"]
pub mod calib;
