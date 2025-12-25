#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_spi1_ctrl_mod: R8Spi1CtrlMod,
    r8_spi1_ctrl_cfg: R8Spi1CtrlCfg,
    r8_spi1_inter_en: R8Spi1InterEn,
    r8_spi1_clock_div_r8_spi1_slave_pre: R8Spi1ClockDivR8Spi1SlavePre,
    r8_spi1_buffer: R8Spi1Buffer,
    r8_spi1_run_flag: R8Spi1RunFlag,
    r8_spi1_int_flag: R8Spi1IntFlag,
    r8_spi1_fifo_count: R8Spi1FifoCount,
    _reserved8: [u8; 0x04],
    r16_spi1_total_cnt: R16Spi1TotalCnt,
    _reserved9: [u8; 0x02],
    r8_spi1_fifo: R8Spi1Fifo,
    _reserved10: [u8; 0x02],
    r8_spi1_fifo_count1: R8Spi1FifoCount1,
    r32_spi1_dma_now: R32Spi1DmaNow,
    r32_spi1_dma_beg: R32Spi1DmaBeg,
    r32_spi1_dma_end: R32Spi1DmaEnd,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI1 mode control"]
    #[inline(always)]
    pub const fn r8_spi1_ctrl_mod(&self) -> &R8Spi1CtrlMod {
        &self.r8_spi1_ctrl_mod
    }
    #[doc = "0x01 - SPI1 configuration control"]
    #[inline(always)]
    pub const fn r8_spi1_ctrl_cfg(&self) -> &R8Spi1CtrlCfg {
        &self.r8_spi1_ctrl_cfg
    }
    #[doc = "0x02 - SPI1 interrupt enable"]
    #[inline(always)]
    pub const fn r8_spi1_inter_en(&self) -> &R8Spi1InterEn {
        &self.r8_spi1_inter_en
    }
    #[doc = "0x03 - SPI1 master clock divisor / SPI0 slave preset value"]
    #[inline(always)]
    pub const fn r8_spi1_clock_div_r8_spi1_slave_pre(&self) -> &R8Spi1ClockDivR8Spi1SlavePre {
        &self.r8_spi1_clock_div_r8_spi1_slave_pre
    }
    #[doc = "0x04 - SPI1 data buffer"]
    #[inline(always)]
    pub const fn r8_spi1_buffer(&self) -> &R8Spi1Buffer {
        &self.r8_spi1_buffer
    }
    #[doc = "0x05 - SPI1 work flag"]
    #[inline(always)]
    pub const fn r8_spi1_run_flag(&self) -> &R8Spi1RunFlag {
        &self.r8_spi1_run_flag
    }
    #[doc = "0x06 - SPI1 interrupt flag"]
    #[inline(always)]
    pub const fn r8_spi1_int_flag(&self) -> &R8Spi1IntFlag {
        &self.r8_spi1_int_flag
    }
    #[doc = "0x07 - SPI1 FIFO count status"]
    #[inline(always)]
    pub const fn r8_spi1_fifo_count(&self) -> &R8Spi1FifoCount {
        &self.r8_spi1_fifo_count
    }
    #[doc = "0x0c - SPI1 total byte count, only low 12 bit"]
    #[inline(always)]
    pub const fn r16_spi1_total_cnt(&self) -> &R16Spi1TotalCnt {
        &self.r16_spi1_total_cnt
    }
    #[doc = "0x10 - SPI1 FIFO register"]
    #[inline(always)]
    pub const fn r8_spi1_fifo(&self) -> &R8Spi1Fifo {
        &self.r8_spi1_fifo
    }
    #[doc = "0x13 - SPI0 FIFO count status"]
    #[inline(always)]
    pub const fn r8_spi1_fifo_count1(&self) -> &R8Spi1FifoCount1 {
        &self.r8_spi1_fifo_count1
    }
    #[doc = "0x14 - SPI1 DMA current address"]
    #[inline(always)]
    pub const fn r32_spi1_dma_now(&self) -> &R32Spi1DmaNow {
        &self.r32_spi1_dma_now
    }
    #[doc = "0x18 - SPI1 DMA begin address"]
    #[inline(always)]
    pub const fn r32_spi1_dma_beg(&self) -> &R32Spi1DmaBeg {
        &self.r32_spi1_dma_beg
    }
    #[doc = "0x1c - SPI1 DMA end address"]
    #[inline(always)]
    pub const fn r32_spi1_dma_end(&self) -> &R32Spi1DmaEnd {
        &self.r32_spi1_dma_end
    }
}
#[doc = "R8_SPI1_CTRL_MOD (rw) register accessor: SPI1 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_ctrl_mod`] module"]
#[doc(alias = "R8_SPI1_CTRL_MOD")]
pub type R8Spi1CtrlMod = crate::Reg<r8_spi1_ctrl_mod::R8Spi1CtrlModSpec>;
#[doc = "SPI1 mode control"]
pub mod r8_spi1_ctrl_mod;
#[doc = "R8_SPI1_CTRL_CFG (rw) register accessor: SPI1 configuration control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_ctrl_cfg`] module"]
#[doc(alias = "R8_SPI1_CTRL_CFG")]
pub type R8Spi1CtrlCfg = crate::Reg<r8_spi1_ctrl_cfg::R8Spi1CtrlCfgSpec>;
#[doc = "SPI1 configuration control"]
pub mod r8_spi1_ctrl_cfg;
#[doc = "R8_SPI1_INTER_EN (rw) register accessor: SPI1 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_inter_en`] module"]
#[doc(alias = "R8_SPI1_INTER_EN")]
pub type R8Spi1InterEn = crate::Reg<r8_spi1_inter_en::R8Spi1InterEnSpec>;
#[doc = "SPI1 interrupt enable"]
pub mod r8_spi1_inter_en;
#[doc = "R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE (rw) register accessor: SPI1 master clock divisor / SPI0 slave preset value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_clock_div_r8_spi1_slave_pre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_clock_div_r8_spi1_slave_pre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_clock_div_r8_spi1_slave_pre`] module"]
#[doc(alias = "R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE")]
pub type R8Spi1ClockDivR8Spi1SlavePre =
    crate::Reg<r8_spi1_clock_div_r8_spi1_slave_pre::R8Spi1ClockDivR8Spi1SlavePreSpec>;
#[doc = "SPI1 master clock divisor / SPI0 slave preset value"]
pub mod r8_spi1_clock_div_r8_spi1_slave_pre;
#[doc = "R8_SPI1_BUFFER (rw) register accessor: SPI1 data buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_buffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_buffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_buffer`] module"]
#[doc(alias = "R8_SPI1_BUFFER")]
pub type R8Spi1Buffer = crate::Reg<r8_spi1_buffer::R8Spi1BufferSpec>;
#[doc = "SPI1 data buffer"]
pub mod r8_spi1_buffer;
#[doc = "R8_SPI1_RUN_FLAG (rw) register accessor: SPI1 work flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_run_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_run_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_run_flag`] module"]
#[doc(alias = "R8_SPI1_RUN_FLAG")]
pub type R8Spi1RunFlag = crate::Reg<r8_spi1_run_flag::R8Spi1RunFlagSpec>;
#[doc = "SPI1 work flag"]
pub mod r8_spi1_run_flag;
#[doc = "R8_SPI1_INT_FLAG (rw) register accessor: SPI1 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_int_flag`] module"]
#[doc(alias = "R8_SPI1_INT_FLAG")]
pub type R8Spi1IntFlag = crate::Reg<r8_spi1_int_flag::R8Spi1IntFlagSpec>;
#[doc = "SPI1 interrupt flag"]
pub mod r8_spi1_int_flag;
#[doc = "R8_SPI1_FIFO_COUNT (rw) register accessor: SPI1 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_fifo_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_fifo_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_fifo_count`] module"]
#[doc(alias = "R8_SPI1_FIFO_COUNT")]
pub type R8Spi1FifoCount = crate::Reg<r8_spi1_fifo_count::R8Spi1FifoCountSpec>;
#[doc = "SPI1 FIFO count status"]
pub mod r8_spi1_fifo_count;
#[doc = "R16_SPI1_TOTAL_CNT (rw) register accessor: SPI1 total byte count, only low 12 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi1_total_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi1_total_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_spi1_total_cnt`] module"]
#[doc(alias = "R16_SPI1_TOTAL_CNT")]
pub type R16Spi1TotalCnt = crate::Reg<r16_spi1_total_cnt::R16Spi1TotalCntSpec>;
#[doc = "SPI1 total byte count, only low 12 bit"]
pub mod r16_spi1_total_cnt;
#[doc = "R8_SPI1_FIFO (rw) register accessor: SPI1 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_fifo`] module"]
#[doc(alias = "R8_SPI1_FIFO")]
pub type R8Spi1Fifo = crate::Reg<r8_spi1_fifo::R8Spi1FifoSpec>;
#[doc = "SPI1 FIFO register"]
pub mod r8_spi1_fifo;
#[doc = "R8_SPI1_FIFO_COUNT1 (rw) register accessor: SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_fifo_count1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_fifo_count1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_spi1_fifo_count1`] module"]
#[doc(alias = "R8_SPI1_FIFO_COUNT1")]
pub type R8Spi1FifoCount1 = crate::Reg<r8_spi1_fifo_count1::R8Spi1FifoCount1Spec>;
#[doc = "SPI0 FIFO count status"]
pub mod r8_spi1_fifo_count1;
#[doc = "R32_SPI1_DMA_NOW (rw) register accessor: SPI1 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi1_dma_now::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi1_dma_now::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_spi1_dma_now`] module"]
#[doc(alias = "R32_SPI1_DMA_NOW")]
pub type R32Spi1DmaNow = crate::Reg<r32_spi1_dma_now::R32Spi1DmaNowSpec>;
#[doc = "SPI1 DMA current address"]
pub mod r32_spi1_dma_now;
#[doc = "R32_SPI1_DMA_BEG (rw) register accessor: SPI1 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi1_dma_beg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi1_dma_beg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_spi1_dma_beg`] module"]
#[doc(alias = "R32_SPI1_DMA_BEG")]
pub type R32Spi1DmaBeg = crate::Reg<r32_spi1_dma_beg::R32Spi1DmaBegSpec>;
#[doc = "SPI1 DMA begin address"]
pub mod r32_spi1_dma_beg;
#[doc = "R32_SPI1_DMA_END (rw) register accessor: SPI1 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi1_dma_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi1_dma_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_spi1_dma_end`] module"]
#[doc(alias = "R32_SPI1_DMA_END")]
pub type R32Spi1DmaEnd = crate::Reg<r32_spi1_dma_end::R32Spi1DmaEndSpec>;
#[doc = "SPI1 DMA end address"]
pub mod r32_spi1_dma_end;
