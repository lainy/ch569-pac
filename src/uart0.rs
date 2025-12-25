#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_uart0_mcr: R8Uart0Mcr,
    r8_uart0_ier: R8Uart0Ier,
    r8_uart0_fcr: R8Uart0Fcr,
    r8_uart0_lcr: R8Uart0Lcr,
    r8_uart0_iir: R8Uart0Iir,
    r8_uart0_lsr: R8Uart0Lsr,
    r8_uart0_msr: R8Uart0Msr,
    _reserved7: [u8; 0x01],
    r8_uart0_rbr_r8_uart0_thr: R8Uart0RbrR8Uart0Thr,
    _reserved8: [u8; 0x01],
    r8_uart0_rfc: R8Uart0Rfc,
    r8_uart0_tfc: R8Uart0Tfc,
    r16_uart0_dl: R16Uart0Dl,
    r8_uart0_div: R8Uart0Div,
    r8_uart0_adr: R8Uart0Adr,
}
impl RegisterBlock {
    #[doc = "0x00 - UART0 modem control"]
    #[inline(always)]
    pub const fn r8_uart0_mcr(&self) -> &R8Uart0Mcr {
        &self.r8_uart0_mcr
    }
    #[doc = "0x01 - UART0 interrupt enable"]
    #[inline(always)]
    pub const fn r8_uart0_ier(&self) -> &R8Uart0Ier {
        &self.r8_uart0_ier
    }
    #[doc = "0x02 - UART0 FIFO control"]
    #[inline(always)]
    pub const fn r8_uart0_fcr(&self) -> &R8Uart0Fcr {
        &self.r8_uart0_fcr
    }
    #[doc = "0x03 - UART0 line control"]
    #[inline(always)]
    pub const fn r8_uart0_lcr(&self) -> &R8Uart0Lcr {
        &self.r8_uart0_lcr
    }
    #[doc = "0x04 - UART0 interrupt identification"]
    #[inline(always)]
    pub const fn r8_uart0_iir(&self) -> &R8Uart0Iir {
        &self.r8_uart0_iir
    }
    #[doc = "0x05 - UART0 line status"]
    #[inline(always)]
    pub const fn r8_uart0_lsr(&self) -> &R8Uart0Lsr {
        &self.r8_uart0_lsr
    }
    #[doc = "0x06 - UART0 modem status"]
    #[inline(always)]
    pub const fn r8_uart0_msr(&self) -> &R8Uart0Msr {
        &self.r8_uart0_msr
    }
    #[doc = "0x08 - UART0 receiver buffer, receiving byte / UART0 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub const fn r8_uart0_rbr_r8_uart0_thr(&self) -> &R8Uart0RbrR8Uart0Thr {
        &self.r8_uart0_rbr_r8_uart0_thr
    }
    #[doc = "0x0a - UART0 receiver FIFO count"]
    #[inline(always)]
    pub const fn r8_uart0_rfc(&self) -> &R8Uart0Rfc {
        &self.r8_uart0_rfc
    }
    #[doc = "0x0b - UART0 transmitter FIFO count"]
    #[inline(always)]
    pub const fn r8_uart0_tfc(&self) -> &R8Uart0Tfc {
        &self.r8_uart0_tfc
    }
    #[doc = "0x0c - UART0 divisor latch"]
    #[inline(always)]
    pub const fn r16_uart0_dl(&self) -> &R16Uart0Dl {
        &self.r16_uart0_dl
    }
    #[doc = "0x0e - UART0 pre-divisor latch byte"]
    #[inline(always)]
    pub const fn r8_uart0_div(&self) -> &R8Uart0Div {
        &self.r8_uart0_div
    }
    #[doc = "0x0f - UART0 slave address"]
    #[inline(always)]
    pub const fn r8_uart0_adr(&self) -> &R8Uart0Adr {
        &self.r8_uart0_adr
    }
}
#[doc = "R8_UART0_MCR (rw) register accessor: UART0 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_mcr`] module"]
#[doc(alias = "R8_UART0_MCR")]
pub type R8Uart0Mcr = crate::Reg<r8_uart0_mcr::R8Uart0McrSpec>;
#[doc = "UART0 modem control"]
pub mod r8_uart0_mcr;
#[doc = "R8_UART0_IER (rw) register accessor: UART0 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_ier`] module"]
#[doc(alias = "R8_UART0_IER")]
pub type R8Uart0Ier = crate::Reg<r8_uart0_ier::R8Uart0IerSpec>;
#[doc = "UART0 interrupt enable"]
pub mod r8_uart0_ier;
#[doc = "R8_UART0_FCR (rw) register accessor: UART0 FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_fcr`] module"]
#[doc(alias = "R8_UART0_FCR")]
pub type R8Uart0Fcr = crate::Reg<r8_uart0_fcr::R8Uart0FcrSpec>;
#[doc = "UART0 FIFO control"]
pub mod r8_uart0_fcr;
#[doc = "R8_UART0_LCR (rw) register accessor: UART0 line control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_lcr`] module"]
#[doc(alias = "R8_UART0_LCR")]
pub type R8Uart0Lcr = crate::Reg<r8_uart0_lcr::R8Uart0LcrSpec>;
#[doc = "UART0 line control"]
pub mod r8_uart0_lcr;
#[doc = "R8_UART0_IIR (r) register accessor: UART0 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_iir`] module"]
#[doc(alias = "R8_UART0_IIR")]
pub type R8Uart0Iir = crate::Reg<r8_uart0_iir::R8Uart0IirSpec>;
#[doc = "UART0 interrupt identification"]
pub mod r8_uart0_iir;
#[doc = "R8_UART0_LSR (r) register accessor: UART0 line status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_lsr`] module"]
#[doc(alias = "R8_UART0_LSR")]
pub type R8Uart0Lsr = crate::Reg<r8_uart0_lsr::R8Uart0LsrSpec>;
#[doc = "UART0 line status"]
pub mod r8_uart0_lsr;
#[doc = "R8_UART0_MSR (r) register accessor: UART0 modem status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_msr`] module"]
#[doc(alias = "R8_UART0_MSR")]
pub type R8Uart0Msr = crate::Reg<r8_uart0_msr::R8Uart0MsrSpec>;
#[doc = "UART0 modem status"]
pub mod r8_uart0_msr;
#[doc = "R8_UART0_RBR_R8_UART0_THR (rw) register accessor: UART0 receiver buffer, receiving byte / UART0 transmitter holding, transmittal byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_rbr_r8_uart0_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_rbr_r8_uart0_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_rbr_r8_uart0_thr`] module"]
#[doc(alias = "R8_UART0_RBR_R8_UART0_THR")]
pub type R8Uart0RbrR8Uart0Thr = crate::Reg<r8_uart0_rbr_r8_uart0_thr::R8Uart0RbrR8Uart0ThrSpec>;
#[doc = "UART0 receiver buffer, receiving byte / UART0 transmitter holding, transmittal byte"]
pub mod r8_uart0_rbr_r8_uart0_thr;
#[doc = "R8_UART0_RFC (r) register accessor: UART0 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_rfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_rfc`] module"]
#[doc(alias = "R8_UART0_RFC")]
pub type R8Uart0Rfc = crate::Reg<r8_uart0_rfc::R8Uart0RfcSpec>;
#[doc = "UART0 receiver FIFO count"]
pub mod r8_uart0_rfc;
#[doc = "R8_UART0_TFC (r) register accessor: UART0 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_tfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_tfc`] module"]
#[doc(alias = "R8_UART0_TFC")]
pub type R8Uart0Tfc = crate::Reg<r8_uart0_tfc::R8Uart0TfcSpec>;
#[doc = "UART0 transmitter FIFO count"]
pub mod r8_uart0_tfc;
#[doc = "R16_UART0_DL (rw) register accessor: UART0 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart0_dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart0_dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uart0_dl`] module"]
#[doc(alias = "R16_UART0_DL")]
pub type R16Uart0Dl = crate::Reg<r16_uart0_dl::R16Uart0DlSpec>;
#[doc = "UART0 divisor latch"]
pub mod r16_uart0_dl;
#[doc = "R8_UART0_DIV (rw) register accessor: UART0 pre-divisor latch byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_div`] module"]
#[doc(alias = "R8_UART0_DIV")]
pub type R8Uart0Div = crate::Reg<r8_uart0_div::R8Uart0DivSpec>;
#[doc = "UART0 pre-divisor latch byte"]
pub mod r8_uart0_div;
#[doc = "R8_UART0_ADR (rw) register accessor: UART0 slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uart0_adr`] module"]
#[doc(alias = "R8_UART0_ADR")]
pub type R8Uart0Adr = crate::Reg<r8_uart0_adr::R8Uart0AdrSpec>;
#[doc = "UART0 slave address"]
pub mod r8_uart0_adr;
