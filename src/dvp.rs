#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_dvp_cr0: R8DvpCr0,
    r8_dvp_cr1: R8DvpCr1,
    r8_dvp_int_en: R8DvpIntEn,
    _reserved3: [u8; 0x01],
    r16_dvp_row_num: R16DvpRowNum,
    r16_dvp_col_num: R16DvpColNum,
    r32_dvp_dma_buf0: R32DvpDmaBuf0,
    r32_dvp_dma_buf1: R32DvpDmaBuf1,
    _reserved_7_r8_dvp: [u8; 0x04],
    r16_dvp_row_cnt: R16DvpRowCnt,
    r16_dvp_col_cnt: R16DvpColCnt,
}
impl RegisterBlock {
    #[doc = "0x00 - DVP control register0"]
    #[inline(always)]
    pub const fn r8_dvp_cr0(&self) -> &R8DvpCr0 {
        &self.r8_dvp_cr0
    }
    #[doc = "0x01 - DVP control register1"]
    #[inline(always)]
    pub const fn r8_dvp_cr1(&self) -> &R8DvpCr1 {
        &self.r8_dvp_cr1
    }
    #[doc = "0x02 - DVP interrupt enable register"]
    #[inline(always)]
    pub const fn r8_dvp_int_en(&self) -> &R8DvpIntEn {
        &self.r8_dvp_int_en
    }
    #[doc = "0x04 - DVP row number of a frame indicator register"]
    #[inline(always)]
    pub const fn r16_dvp_row_num(&self) -> &R16DvpRowNum {
        &self.r16_dvp_row_num
    }
    #[doc = "0x06 - DVP row number of a frame indicator register"]
    #[inline(always)]
    pub const fn r16_dvp_col_num(&self) -> &R16DvpColNum {
        &self.r16_dvp_col_num
    }
    #[doc = "0x08 - DVP dma buffer0 addr"]
    #[inline(always)]
    pub const fn r32_dvp_dma_buf0(&self) -> &R32DvpDmaBuf0 {
        &self.r32_dvp_dma_buf0
    }
    #[doc = "0x0c - DVP dma buffer1 addr"]
    #[inline(always)]
    pub const fn r32_dvp_dma_buf1(&self) -> &R32DvpDmaBuf1 {
        &self.r32_dvp_dma_buf1
    }
    #[doc = "0x10 - DVP interrupt flag register"]
    #[inline(always)]
    pub const fn r8_dvp_int_flag(&self) -> &R8DvpIntFlag {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x11 - DVP receive fifo status"]
    #[inline(always)]
    pub const fn r8_dvp_fifo_st(&self) -> &R8DvpFifoSt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x14 - DVP row count value"]
    #[inline(always)]
    pub const fn r16_dvp_row_cnt(&self) -> &R16DvpRowCnt {
        &self.r16_dvp_row_cnt
    }
    #[doc = "0x16 - DVP col count value"]
    #[inline(always)]
    pub const fn r16_dvp_col_cnt(&self) -> &R16DvpColCnt {
        &self.r16_dvp_col_cnt
    }
}
#[doc = "R8_DVP_CR0 (rw) register accessor: DVP control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_dvp_cr0`] module"]
#[doc(alias = "R8_DVP_CR0")]
pub type R8DvpCr0 = crate::Reg<r8_dvp_cr0::R8DvpCr0Spec>;
#[doc = "DVP control register0"]
pub mod r8_dvp_cr0;
#[doc = "R8_DVP_CR1 (rw) register accessor: DVP control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_dvp_cr1`] module"]
#[doc(alias = "R8_DVP_CR1")]
pub type R8DvpCr1 = crate::Reg<r8_dvp_cr1::R8DvpCr1Spec>;
#[doc = "DVP control register1"]
pub mod r8_dvp_cr1;
#[doc = "R8_DVP_INT_EN (rw) register accessor: DVP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_dvp_int_en`] module"]
#[doc(alias = "R8_DVP_INT_EN")]
pub type R8DvpIntEn = crate::Reg<r8_dvp_int_en::R8DvpIntEnSpec>;
#[doc = "DVP interrupt enable register"]
pub mod r8_dvp_int_en;
#[doc = "R16_DVP_ROW_NUM (rw) register accessor: DVP row number of a frame indicator register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_row_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_dvp_row_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_dvp_row_num`] module"]
#[doc(alias = "R16_DVP_ROW_NUM")]
pub type R16DvpRowNum = crate::Reg<r16_dvp_row_num::R16DvpRowNumSpec>;
#[doc = "DVP row number of a frame indicator register"]
pub mod r16_dvp_row_num;
#[doc = "R16_DVP_COL_NUM (rw) register accessor: DVP row number of a frame indicator register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_col_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_dvp_col_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_dvp_col_num`] module"]
#[doc(alias = "R16_DVP_COL_NUM")]
pub type R16DvpColNum = crate::Reg<r16_dvp_col_num::R16DvpColNumSpec>;
#[doc = "DVP row number of a frame indicator register"]
pub mod r16_dvp_col_num;
#[doc = "R32_DVP_DMA_BUF0 (rw) register accessor: DVP dma buffer0 addr\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_dvp_dma_buf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_dvp_dma_buf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_dvp_dma_buf0`] module"]
#[doc(alias = "R32_DVP_DMA_BUF0")]
pub type R32DvpDmaBuf0 = crate::Reg<r32_dvp_dma_buf0::R32DvpDmaBuf0Spec>;
#[doc = "DVP dma buffer0 addr"]
pub mod r32_dvp_dma_buf0;
#[doc = "R32_DVP_DMA_BUF1 (rw) register accessor: DVP dma buffer1 addr\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_dvp_dma_buf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_dvp_dma_buf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_dvp_dma_buf1`] module"]
#[doc(alias = "R32_DVP_DMA_BUF1")]
pub type R32DvpDmaBuf1 = crate::Reg<r32_dvp_dma_buf1::R32DvpDmaBuf1Spec>;
#[doc = "DVP dma buffer1 addr"]
pub mod r32_dvp_dma_buf1;
#[doc = "R8_DVP_INT_FLAG (rw) register accessor: DVP interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_dvp_int_flag`] module"]
#[doc(alias = "R8_DVP_INT_FLAG")]
pub type R8DvpIntFlag = crate::Reg<r8_dvp_int_flag::R8DvpIntFlagSpec>;
#[doc = "DVP interrupt flag register"]
pub mod r8_dvp_int_flag;
#[doc = "R8_DVP_FIFO_ST (r) register accessor: DVP receive fifo status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_fifo_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_dvp_fifo_st`] module"]
#[doc(alias = "R8_DVP_FIFO_ST")]
pub type R8DvpFifoSt = crate::Reg<r8_dvp_fifo_st::R8DvpFifoStSpec>;
#[doc = "DVP receive fifo status"]
pub mod r8_dvp_fifo_st;
#[doc = "R16_DVP_ROW_CNT (r) register accessor: DVP row count value\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_row_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_dvp_row_cnt`] module"]
#[doc(alias = "R16_DVP_ROW_CNT")]
pub type R16DvpRowCnt = crate::Reg<r16_dvp_row_cnt::R16DvpRowCntSpec>;
#[doc = "DVP row count value"]
pub mod r16_dvp_row_cnt;
#[doc = "R16_DVP_COL_CNT (r) register accessor: DVP col count value\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_col_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_dvp_col_cnt`] module"]
#[doc(alias = "R16_DVP_COL_CNT")]
pub type R16DvpColCnt = crate::Reg<r16_dvp_col_cnt::R16DvpColCntSpec>;
#[doc = "DVP col count value"]
pub mod r16_dvp_col_cnt;
