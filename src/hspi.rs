#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_hspi_cfg: R8HspiCfg,
    r8_hspi_ctrl: R8HspiCtrl,
    r8_hspi_int_en: R8HspiIntEn,
    r8_hspi_aux: R8HspiAux,
    r32_hspi_tx_addr0: R32HspiTxAddr0,
    r32_hspi_tx_addr1: R32HspiTxAddr1,
    r32_hspi_rx_addr0: R32HspiRxAddr0,
    r32_hspi_rx_addr1: R32HspiRxAddr1,
    r16_hspi_dma_len0: R16HspiDmaLen0,
    r16_hspi_rx_len0: R16HspiRxLen0,
    r16_hspi_dma_len1: R16HspiDmaLen1,
    r16_hspi_rx_len1: R16HspiRxLen1,
    r16_hspi_burst_cfg: R16HspiBurstCfg,
    r8_hspi_burst_cnt: R8HspiBurstCnt,
    _reserved14: [u8; 0x01],
    r32_hspi_udf0: R32HspiUdf0,
    r32_hspi_udf1: R32HspiUdf1,
    r8_hspi_int_flag: R8HspiIntFlag,
    r8_hspi_rtx_status: R8HspiRtxStatus,
    r8_hspi_tx_sc: R8HspiTxSc,
    hspi_rx_sc: HspiRxSc,
}
impl RegisterBlock {
    #[doc = "0x00 - parallel if tx/rx cfg"]
    #[inline(always)]
    pub const fn r8_hspi_cfg(&self) -> &R8HspiCfg {
        &self.r8_hspi_cfg
    }
    #[doc = "0x01 - parallel if tx/rx control"]
    #[inline(always)]
    pub const fn r8_hspi_ctrl(&self) -> &R8HspiCtrl {
        &self.r8_hspi_ctrl
    }
    #[doc = "0x02 - parallel if interrupt enable register"]
    #[inline(always)]
    pub const fn r8_hspi_int_en(&self) -> &R8HspiIntEn {
        &self.r8_hspi_int_en
    }
    #[doc = "0x03 - parallel if aux"]
    #[inline(always)]
    pub const fn r8_hspi_aux(&self) -> &R8HspiAux {
        &self.r8_hspi_aux
    }
    #[doc = "0x04 - parallel if dma tx addr0"]
    #[inline(always)]
    pub const fn r32_hspi_tx_addr0(&self) -> &R32HspiTxAddr0 {
        &self.r32_hspi_tx_addr0
    }
    #[doc = "0x08 - parallel if dma tx addr1"]
    #[inline(always)]
    pub const fn r32_hspi_tx_addr1(&self) -> &R32HspiTxAddr1 {
        &self.r32_hspi_tx_addr1
    }
    #[doc = "0x0c - parallel if dma rx addr0"]
    #[inline(always)]
    pub const fn r32_hspi_rx_addr0(&self) -> &R32HspiRxAddr0 {
        &self.r32_hspi_rx_addr0
    }
    #[doc = "0x10 - parallel if dma rx addr1"]
    #[inline(always)]
    pub const fn r32_hspi_rx_addr1(&self) -> &R32HspiRxAddr1 {
        &self.r32_hspi_rx_addr1
    }
    #[doc = "0x14 - parallel if dma length0"]
    #[inline(always)]
    pub const fn r16_hspi_dma_len0(&self) -> &R16HspiDmaLen0 {
        &self.r16_hspi_dma_len0
    }
    #[doc = "0x16 - parallel if receive length0"]
    #[inline(always)]
    pub const fn r16_hspi_rx_len0(&self) -> &R16HspiRxLen0 {
        &self.r16_hspi_rx_len0
    }
    #[doc = "0x18 - parallel if dma length1"]
    #[inline(always)]
    pub const fn r16_hspi_dma_len1(&self) -> &R16HspiDmaLen1 {
        &self.r16_hspi_dma_len1
    }
    #[doc = "0x1a - parallel if receive length1"]
    #[inline(always)]
    pub const fn r16_hspi_rx_len1(&self) -> &R16HspiRxLen1 {
        &self.r16_hspi_rx_len1
    }
    #[doc = "0x1c - parallel if tx burst config register"]
    #[inline(always)]
    pub const fn r16_hspi_burst_cfg(&self) -> &R16HspiBurstCfg {
        &self.r16_hspi_burst_cfg
    }
    #[doc = "0x1e - parallel if tx burst count"]
    #[inline(always)]
    pub const fn r8_hspi_burst_cnt(&self) -> &R8HspiBurstCnt {
        &self.r8_hspi_burst_cnt
    }
    #[doc = "0x20 - parallel if user defined field 0 register"]
    #[inline(always)]
    pub const fn r32_hspi_udf0(&self) -> &R32HspiUdf0 {
        &self.r32_hspi_udf0
    }
    #[doc = "0x24 - parallel if user defined field 1 register"]
    #[inline(always)]
    pub const fn r32_hspi_udf1(&self) -> &R32HspiUdf1 {
        &self.r32_hspi_udf1
    }
    #[doc = "0x28 - parallel if interrupt flag"]
    #[inline(always)]
    pub const fn r8_hspi_int_flag(&self) -> &R8HspiIntFlag {
        &self.r8_hspi_int_flag
    }
    #[doc = "0x29 - parallel rtx status"]
    #[inline(always)]
    pub const fn r8_hspi_rtx_status(&self) -> &R8HspiRtxStatus {
        &self.r8_hspi_rtx_status
    }
    #[doc = "0x2a - parallel TX sequence ctrl"]
    #[inline(always)]
    pub const fn r8_hspi_tx_sc(&self) -> &R8HspiTxSc {
        &self.r8_hspi_tx_sc
    }
    #[doc = "0x2b - parallel RX sequence ctrl"]
    #[inline(always)]
    pub const fn hspi_rx_sc(&self) -> &HspiRxSc {
        &self.hspi_rx_sc
    }
}
#[doc = "R8_HSPI_CFG (rw) register accessor: parallel if tx/rx cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_cfg`] module"]
#[doc(alias = "R8_HSPI_CFG")]
pub type R8HspiCfg = crate::Reg<r8_hspi_cfg::R8HspiCfgSpec>;
#[doc = "parallel if tx/rx cfg"]
pub mod r8_hspi_cfg;
#[doc = "R8_HSPI_CTRL (rw) register accessor: parallel if tx/rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_ctrl`] module"]
#[doc(alias = "R8_HSPI_CTRL")]
pub type R8HspiCtrl = crate::Reg<r8_hspi_ctrl::R8HspiCtrlSpec>;
#[doc = "parallel if tx/rx control"]
pub mod r8_hspi_ctrl;
#[doc = "R8_HSPI_INT_EN (rw) register accessor: parallel if interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_int_en`] module"]
#[doc(alias = "R8_HSPI_INT_EN")]
pub type R8HspiIntEn = crate::Reg<r8_hspi_int_en::R8HspiIntEnSpec>;
#[doc = "parallel if interrupt enable register"]
pub mod r8_hspi_int_en;
#[doc = "R8_HSPI_AUX (rw) register accessor: parallel if aux\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_aux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_aux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_aux`] module"]
#[doc(alias = "R8_HSPI_AUX")]
pub type R8HspiAux = crate::Reg<r8_hspi_aux::R8HspiAuxSpec>;
#[doc = "parallel if aux"]
pub mod r8_hspi_aux;
#[doc = "R32_HSPI_TX_ADDR0 (rw) register accessor: parallel if dma tx addr0\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_tx_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_tx_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_hspi_tx_addr0`] module"]
#[doc(alias = "R32_HSPI_TX_ADDR0")]
pub type R32HspiTxAddr0 = crate::Reg<r32_hspi_tx_addr0::R32HspiTxAddr0Spec>;
#[doc = "parallel if dma tx addr0"]
pub mod r32_hspi_tx_addr0;
#[doc = "R32_HSPI_TX_ADDR1 (rw) register accessor: parallel if dma tx addr1\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_tx_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_tx_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_hspi_tx_addr1`] module"]
#[doc(alias = "R32_HSPI_TX_ADDR1")]
pub type R32HspiTxAddr1 = crate::Reg<r32_hspi_tx_addr1::R32HspiTxAddr1Spec>;
#[doc = "parallel if dma tx addr1"]
pub mod r32_hspi_tx_addr1;
#[doc = "R32_HSPI_RX_ADDR0 (rw) register accessor: parallel if dma rx addr0\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_rx_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_rx_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_hspi_rx_addr0`] module"]
#[doc(alias = "R32_HSPI_RX_ADDR0")]
pub type R32HspiRxAddr0 = crate::Reg<r32_hspi_rx_addr0::R32HspiRxAddr0Spec>;
#[doc = "parallel if dma rx addr0"]
pub mod r32_hspi_rx_addr0;
#[doc = "R32_HSPI_RX_ADDR1 (rw) register accessor: parallel if dma rx addr1\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_rx_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_rx_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_hspi_rx_addr1`] module"]
#[doc(alias = "R32_HSPI_RX_ADDR1")]
pub type R32HspiRxAddr1 = crate::Reg<r32_hspi_rx_addr1::R32HspiRxAddr1Spec>;
#[doc = "parallel if dma rx addr1"]
pub mod r32_hspi_rx_addr1;
#[doc = "R16_HSPI_DMA_LEN0 (rw) register accessor: parallel if dma length0\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_dma_len0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_dma_len0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_hspi_dma_len0`] module"]
#[doc(alias = "R16_HSPI_DMA_LEN0")]
pub type R16HspiDmaLen0 = crate::Reg<r16_hspi_dma_len0::R16HspiDmaLen0Spec>;
#[doc = "parallel if dma length0"]
pub mod r16_hspi_dma_len0;
#[doc = "R16_HSPI_RX_LEN0 (rw) register accessor: parallel if receive length0\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_rx_len0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_rx_len0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_hspi_rx_len0`] module"]
#[doc(alias = "R16_HSPI_RX_LEN0")]
pub type R16HspiRxLen0 = crate::Reg<r16_hspi_rx_len0::R16HspiRxLen0Spec>;
#[doc = "parallel if receive length0"]
pub mod r16_hspi_rx_len0;
#[doc = "R16_HSPI_DMA_LEN1 (rw) register accessor: parallel if dma length1\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_dma_len1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_dma_len1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_hspi_dma_len1`] module"]
#[doc(alias = "R16_HSPI_DMA_LEN1")]
pub type R16HspiDmaLen1 = crate::Reg<r16_hspi_dma_len1::R16HspiDmaLen1Spec>;
#[doc = "parallel if dma length1"]
pub mod r16_hspi_dma_len1;
#[doc = "R16_HSPI_RX_LEN1 (rw) register accessor: parallel if receive length1\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_rx_len1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_rx_len1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_hspi_rx_len1`] module"]
#[doc(alias = "R16_HSPI_RX_LEN1")]
pub type R16HspiRxLen1 = crate::Reg<r16_hspi_rx_len1::R16HspiRxLen1Spec>;
#[doc = "parallel if receive length1"]
pub mod r16_hspi_rx_len1;
#[doc = "R16_HSPI_BURST_CFG (rw) register accessor: parallel if tx burst config register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_burst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_burst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_hspi_burst_cfg`] module"]
#[doc(alias = "R16_HSPI_BURST_CFG")]
pub type R16HspiBurstCfg = crate::Reg<r16_hspi_burst_cfg::R16HspiBurstCfgSpec>;
#[doc = "parallel if tx burst config register"]
pub mod r16_hspi_burst_cfg;
#[doc = "R8_HSPI_BURST_CNT (rw) register accessor: parallel if tx burst count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_burst_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_burst_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_burst_cnt`] module"]
#[doc(alias = "R8_HSPI_BURST_CNT")]
pub type R8HspiBurstCnt = crate::Reg<r8_hspi_burst_cnt::R8HspiBurstCntSpec>;
#[doc = "parallel if tx burst count"]
pub mod r8_hspi_burst_cnt;
#[doc = "R32_HSPI_UDF0 (rw) register accessor: parallel if user defined field 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_udf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_udf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_hspi_udf0`] module"]
#[doc(alias = "R32_HSPI_UDF0")]
pub type R32HspiUdf0 = crate::Reg<r32_hspi_udf0::R32HspiUdf0Spec>;
#[doc = "parallel if user defined field 0 register"]
pub mod r32_hspi_udf0;
#[doc = "R32_HSPI_UDF1 (rw) register accessor: parallel if user defined field 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_udf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_udf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_hspi_udf1`] module"]
#[doc(alias = "R32_HSPI_UDF1")]
pub type R32HspiUdf1 = crate::Reg<r32_hspi_udf1::R32HspiUdf1Spec>;
#[doc = "parallel if user defined field 1 register"]
pub mod r32_hspi_udf1;
#[doc = "R8_HSPI_INT_FLAG (rw) register accessor: parallel if interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_int_flag`] module"]
#[doc(alias = "R8_HSPI_INT_FLAG")]
pub type R8HspiIntFlag = crate::Reg<r8_hspi_int_flag::R8HspiIntFlagSpec>;
#[doc = "parallel if interrupt flag"]
pub mod r8_hspi_int_flag;
#[doc = "R8_HSPI_RTX_STATUS (rw) register accessor: parallel rtx status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_rtx_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_rtx_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_rtx_status`] module"]
#[doc(alias = "R8_HSPI_RTX_STATUS")]
pub type R8HspiRtxStatus = crate::Reg<r8_hspi_rtx_status::R8HspiRtxStatusSpec>;
#[doc = "parallel rtx status"]
pub mod r8_hspi_rtx_status;
#[doc = "R8_HSPI_TX_SC (rw) register accessor: parallel TX sequence ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_tx_sc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_tx_sc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_hspi_tx_sc`] module"]
#[doc(alias = "R8_HSPI_TX_SC")]
pub type R8HspiTxSc = crate::Reg<r8_hspi_tx_sc::R8HspiTxScSpec>;
#[doc = "parallel TX sequence ctrl"]
pub mod r8_hspi_tx_sc;
#[doc = "HSPI_RX_SC (rw) register accessor: parallel RX sequence ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`hspi_rx_sc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_rx_sc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hspi_rx_sc`] module"]
#[doc(alias = "HSPI_RX_SC")]
pub type HspiRxSc = crate::Reg<hspi_rx_sc::HspiRxScSpec>;
#[doc = "parallel RX sequence ctrl"]
pub mod hspi_rx_sc;
