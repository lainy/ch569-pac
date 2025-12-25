#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_tmr2_ctrl_mod: R8Tmr2CtrlMod,
    r8_tmr2_ctrl_dma: R8Tmr2CtrlDma,
    r8_tmr2_inter_en: R8Tmr2InterEn,
    _reserved3: [u8; 0x03],
    r8_tmr2_int_flag: R8Tmr2IntFlag,
    r8_tmr2_fifo_count: R8Tmr2FifoCount,
    r32_tmr2_count: R32Tmr2Count,
    r32_tmr2_cnt_end: R32Tmr2CntEnd,
    r32_tmr2_fifo: R32Tmr2Fifo,
    r32_tmr2_dma_now: R32Tmr2DmaNow,
    r32_tmr2_dma_beg: R32Tmr2DmaBeg,
    r32_tmr2_dma_end: R32Tmr2DmaEnd,
}
impl RegisterBlock {
    #[doc = "0x00 - TMR2 mode control"]
    #[inline(always)]
    pub const fn r8_tmr2_ctrl_mod(&self) -> &R8Tmr2CtrlMod {
        &self.r8_tmr2_ctrl_mod
    }
    #[doc = "0x01 - TMR2 DMA control"]
    #[inline(always)]
    pub const fn r8_tmr2_ctrl_dma(&self) -> &R8Tmr2CtrlDma {
        &self.r8_tmr2_ctrl_dma
    }
    #[doc = "0x02 - TMR2 interrupt enable"]
    #[inline(always)]
    pub const fn r8_tmr2_inter_en(&self) -> &R8Tmr2InterEn {
        &self.r8_tmr2_inter_en
    }
    #[doc = "0x06 - TMR2 interrupt flag"]
    #[inline(always)]
    pub const fn r8_tmr2_int_flag(&self) -> &R8Tmr2IntFlag {
        &self.r8_tmr2_int_flag
    }
    #[doc = "0x07 - TMR2 FIFO count status"]
    #[inline(always)]
    pub const fn r8_tmr2_fifo_count(&self) -> &R8Tmr2FifoCount {
        &self.r8_tmr2_fifo_count
    }
    #[doc = "0x08 - TMR2 current count"]
    #[inline(always)]
    pub const fn r32_tmr2_count(&self) -> &R32Tmr2Count {
        &self.r32_tmr2_count
    }
    #[doc = "0x0c - TMR2 end count value, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr2_cnt_end(&self) -> &R32Tmr2CntEnd {
        &self.r32_tmr2_cnt_end
    }
    #[doc = "0x10 - TMR2 end count value, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr2_fifo(&self) -> &R32Tmr2Fifo {
        &self.r32_tmr2_fifo
    }
    #[doc = "0x14 - TMR2 DMA current address"]
    #[inline(always)]
    pub const fn r32_tmr2_dma_now(&self) -> &R32Tmr2DmaNow {
        &self.r32_tmr2_dma_now
    }
    #[doc = "0x18 - TMR2 DMA begin address"]
    #[inline(always)]
    pub const fn r32_tmr2_dma_beg(&self) -> &R32Tmr2DmaBeg {
        &self.r32_tmr2_dma_beg
    }
    #[doc = "0x1c - TMR2 DMA end address"]
    #[inline(always)]
    pub const fn r32_tmr2_dma_end(&self) -> &R32Tmr2DmaEnd {
        &self.r32_tmr2_dma_end
    }
}
#[doc = "R8_TMR2_CTRL_MOD (rw) register accessor: TMR2 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr2_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr2_ctrl_mod`] module"]
#[doc(alias = "R8_TMR2_CTRL_MOD")]
pub type R8Tmr2CtrlMod = crate::Reg<r8_tmr2_ctrl_mod::R8Tmr2CtrlModSpec>;
#[doc = "TMR2 mode control"]
pub mod r8_tmr2_ctrl_mod;
#[doc = "R8_TMR2_INTER_EN (rw) register accessor: TMR2 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr2_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr2_inter_en`] module"]
#[doc(alias = "R8_TMR2_INTER_EN")]
pub type R8Tmr2InterEn = crate::Reg<r8_tmr2_inter_en::R8Tmr2InterEnSpec>;
#[doc = "TMR2 interrupt enable"]
pub mod r8_tmr2_inter_en;
#[doc = "R8_TMR2_INT_FLAG (rw) register accessor: TMR2 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr2_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr2_int_flag`] module"]
#[doc(alias = "R8_TMR2_INT_FLAG")]
pub type R8Tmr2IntFlag = crate::Reg<r8_tmr2_int_flag::R8Tmr2IntFlagSpec>;
#[doc = "TMR2 interrupt flag"]
pub mod r8_tmr2_int_flag;
#[doc = "R8_TMR2_FIFO_COUNT (r) register accessor: TMR2 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_fifo_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr2_fifo_count`] module"]
#[doc(alias = "R8_TMR2_FIFO_COUNT")]
pub type R8Tmr2FifoCount = crate::Reg<r8_tmr2_fifo_count::R8Tmr2FifoCountSpec>;
#[doc = "TMR2 FIFO count status"]
pub mod r8_tmr2_fifo_count;
#[doc = "R32_TMR2_COUNT (r) register accessor: TMR2 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr2_count`] module"]
#[doc(alias = "R32_TMR2_COUNT")]
pub type R32Tmr2Count = crate::Reg<r32_tmr2_count::R32Tmr2CountSpec>;
#[doc = "TMR2 current count"]
pub mod r32_tmr2_count;
#[doc = "R32_TMR2_CNT_END (rw) register accessor: TMR2 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_cnt_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_cnt_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr2_cnt_end`] module"]
#[doc(alias = "R32_TMR2_CNT_END")]
pub type R32Tmr2CntEnd = crate::Reg<r32_tmr2_cnt_end::R32Tmr2CntEndSpec>;
#[doc = "TMR2 end count value, only low 26 bit"]
pub mod r32_tmr2_cnt_end;
#[doc = "R32_TMR2_FIFO (rw) register accessor: TMR2 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr2_fifo`] module"]
#[doc(alias = "R32_TMR2_FIFO")]
pub type R32Tmr2Fifo = crate::Reg<r32_tmr2_fifo::R32Tmr2FifoSpec>;
#[doc = "TMR2 end count value, only low 26 bit"]
pub mod r32_tmr2_fifo;
#[doc = "R8_TMR2_CTRL_DMA (rw) register accessor: TMR2 DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_ctrl_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr2_ctrl_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr2_ctrl_dma`] module"]
#[doc(alias = "R8_TMR2_CTRL_DMA")]
pub type R8Tmr2CtrlDma = crate::Reg<r8_tmr2_ctrl_dma::R8Tmr2CtrlDmaSpec>;
#[doc = "TMR2 DMA control"]
pub mod r8_tmr2_ctrl_dma;
#[doc = "R32_TMR2_DMA_NOW (rw) register accessor: TMR2 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_dma_now::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_dma_now::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr2_dma_now`] module"]
#[doc(alias = "R32_TMR2_DMA_NOW")]
pub type R32Tmr2DmaNow = crate::Reg<r32_tmr2_dma_now::R32Tmr2DmaNowSpec>;
#[doc = "TMR2 DMA current address"]
pub mod r32_tmr2_dma_now;
#[doc = "R32_TMR2_DMA_BEG (rw) register accessor: TMR2 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_dma_beg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_dma_beg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr2_dma_beg`] module"]
#[doc(alias = "R32_TMR2_DMA_BEG")]
pub type R32Tmr2DmaBeg = crate::Reg<r32_tmr2_dma_beg::R32Tmr2DmaBegSpec>;
#[doc = "TMR2 DMA begin address"]
pub mod r32_tmr2_dma_beg;
#[doc = "R32_TMR2_DMA_END (rw) register accessor: TMR2 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_dma_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_dma_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr2_dma_end`] module"]
#[doc(alias = "R32_TMR2_DMA_END")]
pub type R32Tmr2DmaEnd = crate::Reg<r32_tmr2_dma_end::R32Tmr2DmaEndSpec>;
#[doc = "TMR2 DMA end address"]
pub mod r32_tmr2_dma_end;
