#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_tmr0_ctrl_mod: R8Tmr0CtrlMod,
    _reserved1: [u8; 0x01],
    r8_tmr0_inter_en: R8Tmr0InterEn,
    _reserved2: [u8; 0x03],
    r8_tmr0_int_flag: R8Tmr0IntFlag,
    r8_tmr0_fifo_count: R8Tmr0FifoCount,
    r32_tmr0_count: R32Tmr0Count,
    r32_tmr0_cnt_end: R32Tmr0CntEnd,
    r32_tmr0_fifo: R32Tmr0Fifo,
}
impl RegisterBlock {
    #[doc = "0x00 - TMR0 mode control"]
    #[inline(always)]
    pub const fn r8_tmr0_ctrl_mod(&self) -> &R8Tmr0CtrlMod {
        &self.r8_tmr0_ctrl_mod
    }
    #[doc = "0x02 - TMR0 interrupt enable"]
    #[inline(always)]
    pub const fn r8_tmr0_inter_en(&self) -> &R8Tmr0InterEn {
        &self.r8_tmr0_inter_en
    }
    #[doc = "0x06 - TMR0 interrupt flag"]
    #[inline(always)]
    pub const fn r8_tmr0_int_flag(&self) -> &R8Tmr0IntFlag {
        &self.r8_tmr0_int_flag
    }
    #[doc = "0x07 - TMR0 FIFO count status"]
    #[inline(always)]
    pub const fn r8_tmr0_fifo_count(&self) -> &R8Tmr0FifoCount {
        &self.r8_tmr0_fifo_count
    }
    #[doc = "0x08 - TMR0 current count"]
    #[inline(always)]
    pub const fn r32_tmr0_count(&self) -> &R32Tmr0Count {
        &self.r32_tmr0_count
    }
    #[doc = "0x0c - TMR0 end count value, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr0_cnt_end(&self) -> &R32Tmr0CntEnd {
        &self.r32_tmr0_cnt_end
    }
    #[doc = "0x10 - TMR0 FIFO register, only low 26 bit"]
    #[inline(always)]
    pub const fn r32_tmr0_fifo(&self) -> &R32Tmr0Fifo {
        &self.r32_tmr0_fifo
    }
}
#[doc = "R8_TMR0_CTRL_MOD (rw) register accessor: TMR0 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr0_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr0_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr0_ctrl_mod`] module"]
#[doc(alias = "R8_TMR0_CTRL_MOD")]
pub type R8Tmr0CtrlMod = crate::Reg<r8_tmr0_ctrl_mod::R8Tmr0CtrlModSpec>;
#[doc = "TMR0 mode control"]
pub mod r8_tmr0_ctrl_mod;
#[doc = "R8_TMR0_INTER_EN (rw) register accessor: TMR0 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr0_inter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr0_inter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr0_inter_en`] module"]
#[doc(alias = "R8_TMR0_INTER_EN")]
pub type R8Tmr0InterEn = crate::Reg<r8_tmr0_inter_en::R8Tmr0InterEnSpec>;
#[doc = "TMR0 interrupt enable"]
pub mod r8_tmr0_inter_en;
#[doc = "R8_TMR0_INT_FLAG (rw) register accessor: TMR0 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr0_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr0_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr0_int_flag`] module"]
#[doc(alias = "R8_TMR0_INT_FLAG")]
pub type R8Tmr0IntFlag = crate::Reg<r8_tmr0_int_flag::R8Tmr0IntFlagSpec>;
#[doc = "TMR0 interrupt flag"]
pub mod r8_tmr0_int_flag;
#[doc = "R8_TMR0_FIFO_COUNT (r) register accessor: TMR0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr0_fifo_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_tmr0_fifo_count`] module"]
#[doc(alias = "R8_TMR0_FIFO_COUNT")]
pub type R8Tmr0FifoCount = crate::Reg<r8_tmr0_fifo_count::R8Tmr0FifoCountSpec>;
#[doc = "TMR0 FIFO count status"]
pub mod r8_tmr0_fifo_count;
#[doc = "R32_TMR0_COUNT (r) register accessor: TMR0 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr0_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr0_count`] module"]
#[doc(alias = "R32_TMR0_COUNT")]
pub type R32Tmr0Count = crate::Reg<r32_tmr0_count::R32Tmr0CountSpec>;
#[doc = "TMR0 current count"]
pub mod r32_tmr0_count;
#[doc = "R32_TMR0_CNT_END (rw) register accessor: TMR0 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr0_cnt_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr0_cnt_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr0_cnt_end`] module"]
#[doc(alias = "R32_TMR0_CNT_END")]
pub type R32Tmr0CntEnd = crate::Reg<r32_tmr0_cnt_end::R32Tmr0CntEndSpec>;
#[doc = "TMR0 end count value, only low 26 bit"]
pub mod r32_tmr0_cnt_end;
#[doc = "R32_TMR0_FIFO (rw) register accessor: TMR0 FIFO register, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr0_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr0_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_tmr0_fifo`] module"]
#[doc(alias = "R32_TMR0_FIFO")]
pub type R32Tmr0Fifo = crate::Reg<r32_tmr0_fifo::R32Tmr0FifoSpec>;
#[doc = "TMR0 FIFO register, only low 26 bit"]
pub mod r32_tmr0_fifo;
