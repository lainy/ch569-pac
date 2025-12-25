#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r32_emmc_argument: R32EmmcArgument,
    r16_emmc_cmd_set: R16EmmcCmdSet,
    _reserved2: [u8; 0x02],
    r32_emmc_response0: R32EmmcResponse0,
    r32_emmc_response1: R32EmmcResponse1,
    r32_emmc_response2: R32EmmcResponse2,
    _reserved_5_r32_emmc: [u8; 0x04],
    r8_emmc_control: R8EmmcControl,
    _reserved7: [u8; 0x03],
    r8_emmc_timeout: R8EmmcTimeout,
    _reserved8: [u8; 0x03],
    r32_emmc_status: R32EmmcStatus,
    r16_emmc_int_fg: R16EmmcIntFg,
    _reserved10: [u8; 0x02],
    r16_emmc_int_en: R16EmmcIntEn,
    _reserved11: [u8; 0x02],
    r32_emmc_dma_beg1: R32EmmcDmaBeg1,
    r32_emmc_block_cfg: R32EmmcBlockCfg,
    r32_emmc_tran_mode: R32EmmcTranMode,
    r16_emmc_clk_div: R16EmmcClkDiv,
    _reserved15: [u8; 0x02],
    r32_emmc_dma_beg2: R32EmmcDmaBeg2,
}
impl RegisterBlock {
    #[doc = "0x00 - SD 32bits command argument register"]
    #[inline(always)]
    pub const fn r32_emmc_argument(&self) -> &R32EmmcArgument {
        &self.r32_emmc_argument
    }
    #[doc = "0x04 - SD 16bits cmd setting register"]
    #[inline(always)]
    pub const fn r16_emmc_cmd_set(&self) -> &R16EmmcCmdSet {
        &self.r16_emmc_cmd_set
    }
    #[doc = "0x08 - SD 128bits response register, \\[31:0\\] 32bits"]
    #[inline(always)]
    pub const fn r32_emmc_response0(&self) -> &R32EmmcResponse0 {
        &self.r32_emmc_response0
    }
    #[doc = "0x0c - SD 128bits response register, \\[63:32\\] 32bits"]
    #[inline(always)]
    pub const fn r32_emmc_response1(&self) -> &R32EmmcResponse1 {
        &self.r32_emmc_response1
    }
    #[doc = "0x10 - SD 128bits response register, \\[95:64\\] 32bits"]
    #[inline(always)]
    pub const fn r32_emmc_response2(&self) -> &R32EmmcResponse2 {
        &self.r32_emmc_response2
    }
    #[doc = "0x14 - Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\] 32bits"]
    #[inline(always)]
    pub const fn r32_emmc_write_cont(&self) -> &R32EmmcWriteCont {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - SD 128bits response register, \\[127:96\\] 32bits"]
    #[inline(always)]
    pub const fn r32_emmc_response3(&self) -> &R32EmmcResponse3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - SD 8bits control register"]
    #[inline(always)]
    pub const fn r8_emmc_control(&self) -> &R8EmmcControl {
        &self.r8_emmc_control
    }
    #[doc = "0x1c - SD 8bits data timeout value"]
    #[inline(always)]
    pub const fn r8_emmc_timeout(&self) -> &R8EmmcTimeout {
        &self.r8_emmc_timeout
    }
    #[doc = "0x20 - SD status"]
    #[inline(always)]
    pub const fn r32_emmc_status(&self) -> &R32EmmcStatus {
        &self.r32_emmc_status
    }
    #[doc = "0x24 - SD 16bits interrupt flag register"]
    #[inline(always)]
    pub const fn r16_emmc_int_fg(&self) -> &R16EmmcIntFg {
        &self.r16_emmc_int_fg
    }
    #[doc = "0x28 - SD 16bits interrupt enable register"]
    #[inline(always)]
    pub const fn r16_emmc_int_en(&self) -> &R16EmmcIntEn {
        &self.r16_emmc_int_en
    }
    #[doc = "0x2c - SD 16bits DMA start address register when to operate"]
    #[inline(always)]
    pub const fn r32_emmc_dma_beg1(&self) -> &R32EmmcDmaBeg1 {
        &self.r32_emmc_dma_beg1
    }
    #[doc = "0x30 - SD 32bits data counter, \\[15:0\\] number of blocks this time will tran/recv, \\[27:16\\] block sise(byte number) of every block in this time tran/recv"]
    #[inline(always)]
    pub const fn r32_emmc_block_cfg(&self) -> &R32EmmcBlockCfg {
        &self.r32_emmc_block_cfg
    }
    #[doc = "0x34 - SD TRANSFER MODE register"]
    #[inline(always)]
    pub const fn r32_emmc_tran_mode(&self) -> &R32EmmcTranMode {
        &self.r32_emmc_tran_mode
    }
    #[doc = "0x38 - SD clock divider register"]
    #[inline(always)]
    pub const fn r16_emmc_clk_div(&self) -> &R16EmmcClkDiv {
        &self.r16_emmc_clk_div
    }
    #[doc = "0x3c - SD 16bits DMA start address register when to operate"]
    #[inline(always)]
    pub const fn r32_emmc_dma_beg2(&self) -> &R32EmmcDmaBeg2 {
        &self.r32_emmc_dma_beg2
    }
}
#[doc = "R16_EMMC_CLK_DIV (rw) register accessor: SD clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_emmc_clk_div`] module"]
#[doc(alias = "R16_EMMC_CLK_DIV")]
pub type R16EmmcClkDiv = crate::Reg<r16_emmc_clk_div::R16EmmcClkDivSpec>;
#[doc = "SD clock divider register"]
pub mod r16_emmc_clk_div;
#[doc = "R32_EMMC_ARGUMENT (rw) register accessor: SD 32bits command argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_argument::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_argument::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_argument`] module"]
#[doc(alias = "R32_EMMC_ARGUMENT")]
pub type R32EmmcArgument = crate::Reg<r32_emmc_argument::R32EmmcArgumentSpec>;
#[doc = "SD 32bits command argument register"]
pub mod r32_emmc_argument;
#[doc = "R16_EMMC_CMD_SET (rw) register accessor: SD 16bits cmd setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_cmd_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_cmd_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_emmc_cmd_set`] module"]
#[doc(alias = "R16_EMMC_CMD_SET")]
pub type R16EmmcCmdSet = crate::Reg<r16_emmc_cmd_set::R16EmmcCmdSetSpec>;
#[doc = "SD 16bits cmd setting register"]
pub mod r16_emmc_cmd_set;
#[doc = "R32_EMMC_RESPONSE0 (r) register accessor: SD 128bits response register, \\[31:0\\] 32bits\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_response0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_response0`] module"]
#[doc(alias = "R32_EMMC_RESPONSE0")]
pub type R32EmmcResponse0 = crate::Reg<r32_emmc_response0::R32EmmcResponse0Spec>;
#[doc = "SD 128bits response register, \\[31:0\\] 32bits"]
pub mod r32_emmc_response0;
#[doc = "R32_EMMC_RESPONSE1 (r) register accessor: SD 128bits response register, \\[63:32\\] 32bits\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_response1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_response1`] module"]
#[doc(alias = "R32_EMMC_RESPONSE1")]
pub type R32EmmcResponse1 = crate::Reg<r32_emmc_response1::R32EmmcResponse1Spec>;
#[doc = "SD 128bits response register, \\[63:32\\] 32bits"]
pub mod r32_emmc_response1;
#[doc = "R32_EMMC_RESPONSE2 (r) register accessor: SD 128bits response register, \\[95:64\\] 32bits\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_response2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_response2`] module"]
#[doc(alias = "R32_EMMC_RESPONSE2")]
pub type R32EmmcResponse2 = crate::Reg<r32_emmc_response2::R32EmmcResponse2Spec>;
#[doc = "SD 128bits response register, \\[95:64\\] 32bits"]
pub mod r32_emmc_response2;
#[doc = "R32_EMMC_RESPONSE3 (r) register accessor: SD 128bits response register, \\[127:96\\] 32bits\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_response3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_response3`] module"]
#[doc(alias = "R32_EMMC_RESPONSE3")]
pub type R32EmmcResponse3 = crate::Reg<r32_emmc_response3::R32EmmcResponse3Spec>;
#[doc = "SD 128bits response register, \\[127:96\\] 32bits"]
pub mod r32_emmc_response3;
#[doc = "R32_EMMC_WRITE_CONT (w) register accessor: Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\] 32bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_write_cont::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_write_cont`] module"]
#[doc(alias = "R32_EMMC_WRITE_CONT")]
pub type R32EmmcWriteCont = crate::Reg<r32_emmc_write_cont::R32EmmcWriteContSpec>;
#[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\] 32bits"]
pub mod r32_emmc_write_cont;
#[doc = "R8_EMMC_CONTROL (rw) register accessor: SD 8bits control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_emmc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_emmc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_emmc_control`] module"]
#[doc(alias = "R8_EMMC_CONTROL")]
pub type R8EmmcControl = crate::Reg<r8_emmc_control::R8EmmcControlSpec>;
#[doc = "SD 8bits control register"]
pub mod r8_emmc_control;
#[doc = "R8_EMMC_TIMEOUT (rw) register accessor: SD 8bits data timeout value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_emmc_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_emmc_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_emmc_timeout`] module"]
#[doc(alias = "R8_EMMC_TIMEOUT")]
pub type R8EmmcTimeout = crate::Reg<r8_emmc_timeout::R8EmmcTimeoutSpec>;
#[doc = "SD 8bits data timeout value"]
pub mod r8_emmc_timeout;
#[doc = "R32_EMMC_STATUS (r) register accessor: SD status\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_status`] module"]
#[doc(alias = "R32_EMMC_STATUS")]
pub type R32EmmcStatus = crate::Reg<r32_emmc_status::R32EmmcStatusSpec>;
#[doc = "SD status"]
pub mod r32_emmc_status;
#[doc = "R16_EMMC_INT_FG (rw) register accessor: SD 16bits interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_int_fg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_int_fg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_emmc_int_fg`] module"]
#[doc(alias = "R16_EMMC_INT_FG")]
pub type R16EmmcIntFg = crate::Reg<r16_emmc_int_fg::R16EmmcIntFgSpec>;
#[doc = "SD 16bits interrupt flag register"]
pub mod r16_emmc_int_fg;
#[doc = "R16_EMMC_INT_EN (rw) register accessor: SD 16bits interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_emmc_int_en`] module"]
#[doc(alias = "R16_EMMC_INT_EN")]
pub type R16EmmcIntEn = crate::Reg<r16_emmc_int_en::R16EmmcIntEnSpec>;
#[doc = "SD 16bits interrupt enable register"]
pub mod r16_emmc_int_en;
#[doc = "R32_EMMC_DMA_BEG1 (rw) register accessor: SD 16bits DMA start address register when to operate\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_dma_beg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_dma_beg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_dma_beg1`] module"]
#[doc(alias = "R32_EMMC_DMA_BEG1")]
pub type R32EmmcDmaBeg1 = crate::Reg<r32_emmc_dma_beg1::R32EmmcDmaBeg1Spec>;
#[doc = "SD 16bits DMA start address register when to operate"]
pub mod r32_emmc_dma_beg1;
#[doc = "R32_EMMC_BLOCK_CFG (rw) register accessor: SD 32bits data counter, \\[15:0\\] number of blocks this time will tran/recv, \\[27:16\\] block sise(byte number) of every block in this time tran/recv\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_block_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_block_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_block_cfg`] module"]
#[doc(alias = "R32_EMMC_BLOCK_CFG")]
pub type R32EmmcBlockCfg = crate::Reg<r32_emmc_block_cfg::R32EmmcBlockCfgSpec>;
#[doc = "SD 32bits data counter, \\[15:0\\] number of blocks this time will tran/recv, \\[27:16\\] block sise(byte number) of every block in this time tran/recv"]
pub mod r32_emmc_block_cfg;
#[doc = "R32_EMMC_TRAN_MODE (rw) register accessor: SD TRANSFER MODE register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_tran_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_tran_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_tran_mode`] module"]
#[doc(alias = "R32_EMMC_TRAN_MODE")]
pub type R32EmmcTranMode = crate::Reg<r32_emmc_tran_mode::R32EmmcTranModeSpec>;
#[doc = "SD TRANSFER MODE register"]
pub mod r32_emmc_tran_mode;
#[doc = "R32_EMMC_DMA_BEG2 (rw) register accessor: SD 16bits DMA start address register when to operate\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_dma_beg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_dma_beg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_emmc_dma_beg2`] module"]
#[doc(alias = "R32_EMMC_DMA_BEG2")]
pub type R32EmmcDmaBeg2 = crate::Reg<r32_emmc_dma_beg2::R32EmmcDmaBeg2Spec>;
#[doc = "SD 16bits DMA start address register when to operate"]
pub mod r32_emmc_dma_beg2;
