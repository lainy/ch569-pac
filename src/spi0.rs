#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_spi0_ctrl_mod: R8Spi0CtrlMod,
    r8_spi0_ctrl_cfg: R8Spi0CtrlCfg,
    r8_spi0_inter_en: R8Spi0InterEn,
    r8_spi0_clock_div_r8_spi0_slave_pre: R8Spi0ClockDivR8Spi0SlavePre,
    r8_spi0_buffer: R8Spi0Buffer,
    r8_spi0_run_flag: R8Spi0RunFlag,
    r8_spi0_int_flag: R8Spi0IntFlag,
    r8_spi0_fifo_count: R8Spi0FifoCount,
    _reserved8: [u8; 0x04],
    r16_spi0_total_cnt: R16Spi0TotalCnt,
    _reserved9: [u8; 0x02],
    r8_spi0_fifo: R8Spi0Fifo,
    _reserved10: [u8; 0x02],
    r8_spi0_fifo_count1: R8Spi0FifoCount1,
    r32_spi0_dma_now: R32Spi0DmaNow,
    r32_spi0_dma_beg: R32Spi0DmaBeg,
    r32_spi0_dma_end: R32Spi0DmaEnd,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI0 mode control"]
    #[inline(always)]
    pub const fn r8_spi0_ctrl_mod(&self) -> &R8Spi0CtrlMod {
        &self.r8_spi0_ctrl_mod
    }
    #[doc = "0x01 - SPI0 configuration control"]
    #[inline(always)]
    pub const fn r8_spi0_ctrl_cfg(&self) -> &R8Spi0CtrlCfg {
        &self.r8_spi0_ctrl_cfg
    }
    #[doc = "0x02 - SPI0 interrupt enable"]
    #[inline(always)]
    pub const fn r8_spi0_inter_en(&self) -> &R8Spi0InterEn {
        &self.r8_spi0_inter_en
    }
    #[doc = "0x03 - SPI0 master clock divisor / SPI0 slave preset value"]
    #[inline(always)]
    pub const fn r8_spi0_clock_div_r8_spi0_slave_pre(&self) -> &R8Spi0ClockDivR8Spi0SlavePre {
        &self.r8_spi0_clock_div_r8_spi0_slave_pre
    }
    #[doc = "0x04 - SPI0 data buffer"]
    #[inline(always)]
    pub const fn r8_spi0_buffer(&self) -> &R8Spi0Buffer {
        &self.r8_spi0_buffer
    }
    #[doc = "0x05 - SPI0 work flag"]
    #[inline(always)]
    pub const fn r8_spi0_run_flag(&self) -> &R8Spi0RunFlag {
        &self.r8_spi0_run_flag
    }
    #[doc = "0x06 - SPI0 interrupt flag"]
    #[inline(always)]
    pub const fn r8_spi0_int_flag(&self) -> &R8Spi0IntFlag {
        &self.r8_spi0_int_flag
    }
    #[doc = "0x07 - SPI0 FIFO count status"]
    #[inline(always)]
    pub const fn r8_spi0_fifo_count(&self) -> &R8Spi0FifoCount {
        &self.r8_spi0_fifo_count
    }
    #[doc = "0x0c - SPI0 total byte count, only low 12 bit"]
    #[inline(always)]
    pub const fn r16_spi0_total_cnt(&self) -> &R16Spi0TotalCnt {
        &self.r16_spi0_total_cnt
    }
    #[doc = "0x10 - SPI0 FIFO register"]
    #[inline(always)]
    pub const fn r8_spi0_fifo(&self) -> &R8Spi0Fifo {
        &self.r8_spi0_fifo
    }
    #[doc = "0x13 - SPI0 FIFO count status"]
    #[inline(always)]
    pub const fn r8_spi0_fifo_count1(&self) -> &R8Spi0FifoCount1 {
        &self.r8_spi0_fifo_count1
    }
    #[doc = "0x14 - SPI0 DMA current address"]
    #[inline(always)]
    pub const fn r32_spi0_dma_now(&self) -> &R32Spi0DmaNow {
        &self.r32_spi0_dma_now
    }
    #[doc = "0x18 - SPI0 DMA begin address"]
    #[inline(always)]
    pub const fn r32_spi0_dma_beg(&self) -> &R32Spi0DmaBeg {
        &self.r32_spi0_dma_beg
    }
    #[doc = "0x1c - SPI0 DMA end address"]
    #[inline(always)]
    pub const fn r32_spi0_dma_end(&self) -> &R32Spi0DmaEnd {
        &self.r32_spi0_dma_end
    }
}
#[doc = "R8_SPI0_CTRL_MOD (rw) register accessor: SPI0 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_ctrl_mod`] module"]
#[doc(alias = "R8_SPI0_CTRL_MOD")]
pub type R8Spi0CtrlMod = crate::Reg<r8_spi0_ctrl_mod::R8Spi0CtrlModSpec>;
#[doc = "SPI0 mode control"]
pub mod r8_spi0_ctrl_mod;
#[doc = "R8_SPI0_CTRL_CFG (rw) register accessor: SPI0 configuration control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_ctrl_cfg`] module"]
#[doc(alias = "R8_SPI0_CTRL_CFG")]
pub type R8Spi0CtrlCfg = crate::Reg<r8_spi0_ctrl_cfg::R8Spi0CtrlCfgSpec>;
#[doc = "SPI0 configuration control"]
pub mod r8_spi0_ctrl_cfg;
#[doc = "R8_SPI0_INTER_EN (rw) register accessor: SPI0 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_inter_en`] module"]
#[doc(alias = "R8_SPI0_INTER_EN")]
pub type R8Spi0InterEn = crate::Reg<r8_spi0_inter_en::R8Spi0InterEnSpec>;
#[doc = "SPI0 interrupt enable"]
pub mod r8_spi0_inter_en;
#[doc = "R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE (rw) register accessor: SPI0 master clock divisor / SPI0 slave preset value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_clock_div_r8_spi0_slave_pre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_clock_div_r8_spi0_slave_pre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_clock_div_r8_spi0_slave_pre`] module"]
#[doc(alias = "R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE")]
pub type R8Spi0ClockDivR8Spi0SlavePre =
    crate::Reg<r8_spi0_clock_div_r8_spi0_slave_pre::R8Spi0ClockDivR8Spi0SlavePreSpec>;
#[doc = "SPI0 master clock divisor / SPI0 slave preset value"]
pub mod r8_spi0_clock_div_r8_spi0_slave_pre;
#[doc = "R8_SPI0_BUFFER (rw) register accessor: SPI0 data buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_buffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_buffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_buffer`] module"]
#[doc(alias = "R8_SPI0_BUFFER")]
pub type R8Spi0Buffer = crate::Reg<r8_spi0_buffer::R8Spi0BufferSpec>;
#[doc = "SPI0 data buffer"]
pub mod r8_spi0_buffer;
#[doc = "R8_SPI0_RUN_FLAG (r) register accessor: SPI0 work flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_run_flag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_run_flag`] module"]
#[doc(alias = "R8_SPI0_RUN_FLAG")]
pub type R8Spi0RunFlag = crate::Reg<r8_spi0_run_flag::R8Spi0RunFlagSpec>;
#[doc = "SPI0 work flag"]
pub mod r8_spi0_run_flag;
#[doc = "R8_SPI0_INT_FLAG (rw) register accessor: SPI0 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_int_flag`] module"]
#[doc(alias = "R8_SPI0_INT_FLAG")]
pub type R8Spi0IntFlag = crate::Reg<r8_spi0_int_flag::R8Spi0IntFlagSpec>;
#[doc = "SPI0 interrupt flag"]
pub mod r8_spi0_int_flag;
#[doc = "R8_SPI0_FIFO_COUNT (rw) register accessor: SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_fifo_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_fifo_count`] module"]
#[doc(alias = "R8_SPI0_FIFO_COUNT")]
pub type R8Spi0FifoCount = crate::Reg<r8_spi0_fifo_count::R8Spi0FifoCountSpec>;
#[doc = "SPI0 FIFO count status"]
pub mod r8_spi0_fifo_count;
#[doc = "R16_SPI0_TOTAL_CNT (rw) register accessor: SPI0 total byte count, only low 12 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_total_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_total_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_spi0_total_cnt`] module"]
#[doc(alias = "R16_SPI0_TOTAL_CNT")]
pub type R16Spi0TotalCnt = crate::Reg<r16_spi0_total_cnt::R16Spi0TotalCntSpec>;
#[doc = "SPI0 total byte count, only low 12 bit"]
pub mod r16_spi0_total_cnt;
#[doc = "R8_SPI0_FIFO (rw) register accessor: SPI0 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_fifo`] module"]
#[doc(alias = "R8_SPI0_FIFO")]
pub type R8Spi0Fifo = crate::Reg<r8_spi0_fifo::R8Spi0FifoSpec>;
#[doc = "SPI0 FIFO register"]
pub mod r8_spi0_fifo;
#[doc = "R8_SPI0_FIFO_COUNT1 (rw) register accessor: SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_fifo_count1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi0_fifo_count1`] module"]
#[doc(alias = "R8_SPI0_FIFO_COUNT1")]
pub type R8Spi0FifoCount1 = crate::Reg<r8_spi0_fifo_count1::R8Spi0FifoCount1Spec>;
#[doc = "SPI0 FIFO count status"]
pub mod r8_spi0_fifo_count1;
#[doc = "R32_SPI0_DMA_NOW (rw) register accessor: SPI0 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi0_dma_now::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi0_dma_now::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_spi0_dma_now`] module"]
#[doc(alias = "R32_SPI0_DMA_NOW")]
pub type R32Spi0DmaNow = crate::Reg<r32_spi0_dma_now::R32Spi0DmaNowSpec>;
#[doc = "SPI0 DMA current address"]
pub mod r32_spi0_dma_now;
#[doc = "R32_SPI0_DMA_BEG (rw) register accessor: SPI0 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi0_dma_beg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi0_dma_beg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_spi0_dma_beg`] module"]
#[doc(alias = "R32_SPI0_DMA_BEG")]
pub type R32Spi0DmaBeg = crate::Reg<r32_spi0_dma_beg::R32Spi0DmaBegSpec>;
#[doc = "SPI0 DMA begin address"]
pub mod r32_spi0_dma_beg;
#[doc = "R32_SPI0_DMA_END (rw) register accessor: SPI0 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi0_dma_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi0_dma_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_spi0_dma_end`] module"]
#[doc(alias = "R32_SPI0_DMA_END")]
pub type R32Spi0DmaEnd = crate::Reg<r32_spi0_dma_end::R32Spi0DmaEndSpec>;
#[doc = "SPI0 DMA end address"]
pub mod r32_spi0_dma_end;
