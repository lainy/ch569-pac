#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_uart1_mcr: R8Uart1Mcr,
    r8_uart1_ier: R8Uart1Ier,
    r8_uart1_fcr: R8Uart1Fcr,
    r8_uart1_lcr: R8Uart1Lcr,
    r8_uart1_iir: R8Uart1Iir,
    r8_uart1_lsr: R8Uart1Lsr,
    _reserved6: [u8; 0x02],
    r8_uart1_rbr_r8_uart1_thr: R8Uart1RbrR8Uart1Thr,
    _reserved7: [u8; 0x01],
    r8_uart1_rfc: R8Uart1Rfc,
    r8_uart1_tfc: R8Uart1Tfc,
    r16_uart1_dl: R16Uart1Dl,
    r8_uart1_div: R8Uart1Div,
}
impl RegisterBlock {
    #[doc = "0x00 - UART1 modem control"]
    #[inline(always)]
    pub const fn r8_uart1_mcr(&self) -> &R8Uart1Mcr {
        &self.r8_uart1_mcr
    }
    #[doc = "0x01 - UART1 interrupt enable"]
    #[inline(always)]
    pub const fn r8_uart1_ier(&self) -> &R8Uart1Ier {
        &self.r8_uart1_ier
    }
    #[doc = "0x02 - UART1 FIFO control"]
    #[inline(always)]
    pub const fn r8_uart1_fcr(&self) -> &R8Uart1Fcr {
        &self.r8_uart1_fcr
    }
    #[doc = "0x03 - UART1 line control"]
    #[inline(always)]
    pub const fn r8_uart1_lcr(&self) -> &R8Uart1Lcr {
        &self.r8_uart1_lcr
    }
    #[doc = "0x04 - UART1 interrupt identification"]
    #[inline(always)]
    pub const fn r8_uart1_iir(&self) -> &R8Uart1Iir {
        &self.r8_uart1_iir
    }
    #[doc = "0x05 - UART1 line status"]
    #[inline(always)]
    pub const fn r8_uart1_lsr(&self) -> &R8Uart1Lsr {
        &self.r8_uart1_lsr
    }
    #[doc = "0x08 - UART1 receiver buffer, receiving byte / UART1 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub const fn r8_uart1_rbr_r8_uart1_thr(&self) -> &R8Uart1RbrR8Uart1Thr {
        &self.r8_uart1_rbr_r8_uart1_thr
    }
    #[doc = "0x0a - UART1 receiver FIFO count"]
    #[inline(always)]
    pub const fn r8_uart1_rfc(&self) -> &R8Uart1Rfc {
        &self.r8_uart1_rfc
    }
    #[doc = "0x0b - UART1 transmitter FIFO count"]
    #[inline(always)]
    pub const fn r8_uart1_tfc(&self) -> &R8Uart1Tfc {
        &self.r8_uart1_tfc
    }
    #[doc = "0x0c - UART1 divisor latch"]
    #[inline(always)]
    pub const fn r16_uart1_dl(&self) -> &R16Uart1Dl {
        &self.r16_uart1_dl
    }
    #[doc = "0x0e - UART1 pre-divisor latch byte"]
    #[inline(always)]
    pub const fn r8_uart1_div(&self) -> &R8Uart1Div {
        &self.r8_uart1_div
    }
}
#[doc = "R8_UART1_MCR (rw) register accessor: UART1 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_mcr`] module"]
#[doc(alias = "R8_UART1_MCR")]
pub type R8Uart1Mcr = crate::Reg<r8_uart1_mcr::R8Uart1McrSpec>;
#[doc = "UART1 modem control"]
pub mod r8_uart1_mcr;
#[doc = "R8_UART1_IER (rw) register accessor: UART1 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_ier`] module"]
#[doc(alias = "R8_UART1_IER")]
pub type R8Uart1Ier = crate::Reg<r8_uart1_ier::R8Uart1IerSpec>;
#[doc = "UART1 interrupt enable"]
pub mod r8_uart1_ier;
#[doc = "R8_UART1_FCR (rw) register accessor: UART1 FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_fcr`] module"]
#[doc(alias = "R8_UART1_FCR")]
pub type R8Uart1Fcr = crate::Reg<r8_uart1_fcr::R8Uart1FcrSpec>;
#[doc = "UART1 FIFO control"]
pub mod r8_uart1_fcr;
#[doc = "R8_UART1_LCR (rw) register accessor: UART1 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_lcr`] module"]
#[doc(alias = "R8_UART1_LCR")]
pub type R8Uart1Lcr = crate::Reg<r8_uart1_lcr::R8Uart1LcrSpec>;
#[doc = "UART1 line control"]
pub mod r8_uart1_lcr;
#[doc = "R8_UART1_IIR (r) register accessor: UART1 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_iir`] module"]
#[doc(alias = "R8_UART1_IIR")]
pub type R8Uart1Iir = crate::Reg<r8_uart1_iir::R8Uart1IirSpec>;
#[doc = "UART1 interrupt identification"]
pub mod r8_uart1_iir;
#[doc = "R8_UART1_LSR (r) register accessor: UART1 line status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_lsr`] module"]
#[doc(alias = "R8_UART1_LSR")]
pub type R8Uart1Lsr = crate::Reg<r8_uart1_lsr::R8Uart1LsrSpec>;
#[doc = "UART1 line status"]
pub mod r8_uart1_lsr;
#[doc = "R8_UART1_RBR_R8_UART1_THR (rw) register accessor: UART1 receiver buffer, receiving byte / UART1 transmitter holding, transmittal byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_rbr_r8_uart1_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_rbr_r8_uart1_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_rbr_r8_uart1_thr`] module"]
#[doc(alias = "R8_UART1_RBR_R8_UART1_THR")]
pub type R8Uart1RbrR8Uart1Thr = crate::Reg<r8_uart1_rbr_r8_uart1_thr::R8Uart1RbrR8Uart1ThrSpec>;
#[doc = "UART1 receiver buffer, receiving byte / UART1 transmitter holding, transmittal byte"]
pub mod r8_uart1_rbr_r8_uart1_thr;
#[doc = "R8_UART1_RFC (r) register accessor: UART1 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_rfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_rfc`] module"]
#[doc(alias = "R8_UART1_RFC")]
pub type R8Uart1Rfc = crate::Reg<r8_uart1_rfc::R8Uart1RfcSpec>;
#[doc = "UART1 receiver FIFO count"]
pub mod r8_uart1_rfc;
#[doc = "R8_UART1_TFC (r) register accessor: UART1 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_tfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_tfc`] module"]
#[doc(alias = "R8_UART1_TFC")]
pub type R8Uart1Tfc = crate::Reg<r8_uart1_tfc::R8Uart1TfcSpec>;
#[doc = "UART1 transmitter FIFO count"]
pub mod r8_uart1_tfc;
#[doc = "R16_UART1_DL (rw) register accessor: UART1 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart1_dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart1_dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uart1_dl`] module"]
#[doc(alias = "R16_UART1_DL")]
pub type R16Uart1Dl = crate::Reg<r16_uart1_dl::R16Uart1DlSpec>;
#[doc = "UART1 divisor latch"]
pub mod r16_uart1_dl;
#[doc = "R8_UART1_DIV (rw) register accessor: UART1 pre-divisor latch byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart1_div`] module"]
#[doc(alias = "R8_UART1_DIV")]
pub type R8Uart1Div = crate::Reg<r8_uart1_div::R8Uart1DivSpec>;
#[doc = "UART1 pre-divisor latch byte"]
pub mod r8_uart1_div;
