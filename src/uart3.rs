#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_uart3_mcr: R8Uart3Mcr,
    r8_uart3_ier: R8Uart3Ier,
    r8_uart3_fcr: R8Uart3Fcr,
    r8_uart3_lcr: R8Uart3Lcr,
    r8_uart3_iir: R8Uart3Iir,
    r8_uart3_lsr: R8Uart3Lsr,
    _reserved6: [u8; 0x02],
    r8_uart3_rbr_r8_uart3_thr: R8Uart3RbrR8Uart3Thr,
    _reserved7: [u8; 0x01],
    r8_uart3_rfc: R8Uart3Rfc,
    r8_uart3_tfc: R8Uart3Tfc,
    r16_uart3_dl: R16Uart3Dl,
    r8_uart3_div: R8Uart3Div,
}
impl RegisterBlock {
    #[doc = "0x00 - UART3 modem control"]
    #[inline(always)]
    pub const fn r8_uart3_mcr(&self) -> &R8Uart3Mcr {
        &self.r8_uart3_mcr
    }
    #[doc = "0x01 - UART3 interrupt enable"]
    #[inline(always)]
    pub const fn r8_uart3_ier(&self) -> &R8Uart3Ier {
        &self.r8_uart3_ier
    }
    #[doc = "0x02 - UART3 FIFO control"]
    #[inline(always)]
    pub const fn r8_uart3_fcr(&self) -> &R8Uart3Fcr {
        &self.r8_uart3_fcr
    }
    #[doc = "0x03 - UART3 line control"]
    #[inline(always)]
    pub const fn r8_uart3_lcr(&self) -> &R8Uart3Lcr {
        &self.r8_uart3_lcr
    }
    #[doc = "0x04 - UART3 interrupt identification"]
    #[inline(always)]
    pub const fn r8_uart3_iir(&self) -> &R8Uart3Iir {
        &self.r8_uart3_iir
    }
    #[doc = "0x05 - UART3 line status"]
    #[inline(always)]
    pub const fn r8_uart3_lsr(&self) -> &R8Uart3Lsr {
        &self.r8_uart3_lsr
    }
    #[doc = "0x08 - UART3 receiver buffer, receiving byte / UART3 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub const fn r8_uart3_rbr_r8_uart3_thr(&self) -> &R8Uart3RbrR8Uart3Thr {
        &self.r8_uart3_rbr_r8_uart3_thr
    }
    #[doc = "0x0a - UART3 receiver FIFO count"]
    #[inline(always)]
    pub const fn r8_uart3_rfc(&self) -> &R8Uart3Rfc {
        &self.r8_uart3_rfc
    }
    #[doc = "0x0b - UART3 transmitter FIFO count"]
    #[inline(always)]
    pub const fn r8_uart3_tfc(&self) -> &R8Uart3Tfc {
        &self.r8_uart3_tfc
    }
    #[doc = "0x0c - UART3 divisor latch"]
    #[inline(always)]
    pub const fn r16_uart3_dl(&self) -> &R16Uart3Dl {
        &self.r16_uart3_dl
    }
    #[doc = "0x0e - UART3 pre-divisor latch byte"]
    #[inline(always)]
    pub const fn r8_uart3_div(&self) -> &R8Uart3Div {
        &self.r8_uart3_div
    }
}
#[doc = "R8_UART3_MCR (rw) register accessor: UART3 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_mcr`] module"]
#[doc(alias = "R8_UART3_MCR")]
pub type R8Uart3Mcr = crate::Reg<r8_uart3_mcr::R8Uart3McrSpec>;
#[doc = "UART3 modem control"]
pub mod r8_uart3_mcr;
#[doc = "R8_UART3_IER (rw) register accessor: UART3 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_ier`] module"]
#[doc(alias = "R8_UART3_IER")]
pub type R8Uart3Ier = crate::Reg<r8_uart3_ier::R8Uart3IerSpec>;
#[doc = "UART3 interrupt enable"]
pub mod r8_uart3_ier;
#[doc = "R8_UART3_FCR (rw) register accessor: UART3 FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_fcr`] module"]
#[doc(alias = "R8_UART3_FCR")]
pub type R8Uart3Fcr = crate::Reg<r8_uart3_fcr::R8Uart3FcrSpec>;
#[doc = "UART3 FIFO control"]
pub mod r8_uart3_fcr;
#[doc = "R8_UART3_LCR (rw) register accessor: UART3 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_lcr`] module"]
#[doc(alias = "R8_UART3_LCR")]
pub type R8Uart3Lcr = crate::Reg<r8_uart3_lcr::R8Uart3LcrSpec>;
#[doc = "UART3 line control"]
pub mod r8_uart3_lcr;
#[doc = "R8_UART3_IIR (r) register accessor: UART3 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_iir`] module"]
#[doc(alias = "R8_UART3_IIR")]
pub type R8Uart3Iir = crate::Reg<r8_uart3_iir::R8Uart3IirSpec>;
#[doc = "UART3 interrupt identification"]
pub mod r8_uart3_iir;
#[doc = "R8_UART3_LSR (r) register accessor: UART3 line status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_lsr`] module"]
#[doc(alias = "R8_UART3_LSR")]
pub type R8Uart3Lsr = crate::Reg<r8_uart3_lsr::R8Uart3LsrSpec>;
#[doc = "UART3 line status"]
pub mod r8_uart3_lsr;
#[doc = "R8_UART3_RBR_R8_UART3_THR (rw) register accessor: UART3 receiver buffer, receiving byte / UART3 transmitter holding, transmittal byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_rbr_r8_uart3_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_rbr_r8_uart3_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_rbr_r8_uart3_thr`] module"]
#[doc(alias = "R8_UART3_RBR_R8_UART3_THR")]
pub type R8Uart3RbrR8Uart3Thr = crate::Reg<r8_uart3_rbr_r8_uart3_thr::R8Uart3RbrR8Uart3ThrSpec>;
#[doc = "UART3 receiver buffer, receiving byte / UART3 transmitter holding, transmittal byte"]
pub mod r8_uart3_rbr_r8_uart3_thr;
#[doc = "R8_UART3_RFC (r) register accessor: UART3 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_rfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_rfc`] module"]
#[doc(alias = "R8_UART3_RFC")]
pub type R8Uart3Rfc = crate::Reg<r8_uart3_rfc::R8Uart3RfcSpec>;
#[doc = "UART3 receiver FIFO count"]
pub mod r8_uart3_rfc;
#[doc = "R8_UART3_TFC (r) register accessor: UART3 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_tfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_tfc`] module"]
#[doc(alias = "R8_UART3_TFC")]
pub type R8Uart3Tfc = crate::Reg<r8_uart3_tfc::R8Uart3TfcSpec>;
#[doc = "UART3 transmitter FIFO count"]
pub mod r8_uart3_tfc;
#[doc = "R16_UART3_DL (rw) register accessor: UART3 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart3_dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart3_dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uart3_dl`] module"]
#[doc(alias = "R16_UART3_DL")]
pub type R16Uart3Dl = crate::Reg<r16_uart3_dl::R16Uart3DlSpec>;
#[doc = "UART3 divisor latch"]
pub mod r16_uart3_dl;
#[doc = "R8_UART3_DIV (rw) register accessor: UART3 pre-divisor latch byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart3_div`] module"]
#[doc(alias = "R8_UART3_DIV")]
pub type R8Uart3Div = crate::Reg<r8_uart3_div::R8Uart3DivSpec>;
#[doc = "UART3 pre-divisor latch byte"]
pub mod r8_uart3_div;
