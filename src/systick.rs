#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stk_ctlr: StkCtlr,
    stk_cntl: StkCntl,
    stk_cnth: StkCnth,
    stk_cmplr: StkCmplr,
    stk_cmphr: StkCmphr,
    stk_cntfg: StkCntfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Systick counter control register"]
    #[inline(always)]
    pub const fn stk_ctlr(&self) -> &StkCtlr {
        &self.stk_ctlr
    }
    #[doc = "0x04 - Systick counter low register"]
    #[inline(always)]
    pub const fn stk_cntl(&self) -> &StkCntl {
        &self.stk_cntl
    }
    #[doc = "0x08 - Systick counter high register"]
    #[inline(always)]
    pub const fn stk_cnth(&self) -> &StkCnth {
        &self.stk_cnth
    }
    #[doc = "0x0c - Systick compare low register"]
    #[inline(always)]
    pub const fn stk_cmplr(&self) -> &StkCmplr {
        &self.stk_cmplr
    }
    #[doc = "0x10 - Systick compare high register"]
    #[inline(always)]
    pub const fn stk_cmphr(&self) -> &StkCmphr {
        &self.stk_cmphr
    }
    #[doc = "0x14 - Systick counter flag"]
    #[inline(always)]
    pub const fn stk_cntfg(&self) -> &StkCntfg {
        &self.stk_cntfg
    }
}
#[doc = "STK_CTLR (rw) register accessor: Systick counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_ctlr`] module"]
#[doc(alias = "STK_CTLR")]
pub type StkCtlr = crate::Reg<stk_ctlr::StkCtlrSpec>;
#[doc = "Systick counter control register"]
pub mod stk_ctlr;
#[doc = "STK_CNTL (rw) register accessor: Systick counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cntl`] module"]
#[doc(alias = "STK_CNTL")]
pub type StkCntl = crate::Reg<stk_cntl::StkCntlSpec>;
#[doc = "Systick counter low register"]
pub mod stk_cntl;
#[doc = "STK_CNTH (rw) register accessor: Systick counter high register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cnth`] module"]
#[doc(alias = "STK_CNTH")]
pub type StkCnth = crate::Reg<stk_cnth::StkCnthSpec>;
#[doc = "Systick counter high register"]
pub mod stk_cnth;
#[doc = "STK_CMPLR (rw) register accessor: Systick compare low register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cmplr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cmplr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cmplr`] module"]
#[doc(alias = "STK_CMPLR")]
pub type StkCmplr = crate::Reg<stk_cmplr::StkCmplrSpec>;
#[doc = "Systick compare low register"]
pub mod stk_cmplr;
#[doc = "STK_CMPHR (rw) register accessor: Systick compare high register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cmphr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cmphr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cmphr`] module"]
#[doc(alias = "STK_CMPHR")]
pub type StkCmphr = crate::Reg<stk_cmphr::StkCmphrSpec>;
#[doc = "Systick compare high register"]
pub mod stk_cmphr;
#[doc = "STK_CNTFG (rw) register accessor: Systick counter flag\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cntfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cntfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stk_cntfg`] module"]
#[doc(alias = "STK_CNTFG")]
pub type StkCntfg = crate::Reg<stk_cntfg::StkCntfgSpec>;
#[doc = "Systick counter flag"]
pub mod stk_cntfg;
