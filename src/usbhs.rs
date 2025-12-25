#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_usb_ctrl: R8UsbCtrl,
    r8_uhost_ctrl: R8UhostCtrl,
    r8_usb_int_en: R8UsbIntEn,
    r8_usb_dev_ad: R8UsbDevAd,
    r16_usb_frame_no: R16UsbFrameNo,
    r8_usb_suspend: R8UsbSuspend,
    _reserved6: [u8; 0x01],
    r8_usb_spd_type: R8UsbSpdType,
    r8_usb_mis_st: R8UsbMisSt,
    r8_usb_int_fg: R8UsbIntFg,
    r8_usb_int_st: R8UsbIntSt,
    r6_usb_rx_len: R6UsbRxLen,
    _reserved11: [u8; 0x02],
    r8_uep4_1_mod: R8Uep4_1Mod,
    r8_uep2_3_mod_r8_uh_ep_mod: R8Uep2_3ModR8UhEpMod,
    r8_uep5_6_mod: R8Uep5_6Mod,
    r8_uep7_mod: R8Uep7Mod,
    r32_uep0_rt_dma: R32Uep0RtDma,
    r32_uep1_rx_dma: R32Uep1RxDma,
    r32_uep2_rx_dma_r32_uh_rx_dma: R32Uep2RxDmaR32UhRxDma,
    r32_uep3_rx_dma: R32Uep3RxDma,
    r32_uep4_rx_dma: R32Uep4RxDma,
    r32_uep5_rx_dma: R32Uep5RxDma,
    r32_uep6_rx_dma: R32Uep6RxDma,
    r32_uep7_rx_dma: R32Uep7RxDma,
    r32_uep1_tx_dma: R32Uep1TxDma,
    r32_uep2_tx_dma: R32Uep2TxDma,
    r32_uep3_tx_dma_r32_uh_tx_dma: R32Uep3TxDmaR32UhTxDma,
    r32_uep4_tx_dma: R32Uep4TxDma,
    r32_uep5_tx_dma: R32Uep5TxDma,
    r32_uep6_tx_dma: R32Uep6TxDma,
    r32_uep7_tx_dma: R32Uep7TxDma,
    r16_uep0_max_len: R16Uep0MaxLen,
    _reserved31: [u8; 0x02],
    r16_uep1_max_len: R16Uep1MaxLen,
    _reserved32: [u8; 0x02],
    r16_uep2_max_len_r16_uh_max_len: R16Uep2MaxLenR16UhMaxLen,
    _reserved33: [u8; 0x02],
    r16_uep3_max_len: R16Uep3MaxLen,
    _reserved34: [u8; 0x02],
    r16_uep4_max_len: R16Uep4MaxLen,
    _reserved35: [u8; 0x02],
    r16_uep5_max_len: R16Uep5MaxLen,
    _reserved36: [u8; 0x02],
    r16_uep6_max_len: R16Uep6MaxLen,
    _reserved37: [u8; 0x02],
    r16_uep7_max_len: R16Uep7MaxLen,
    _reserved38: [u8; 0x02],
    r16_uep0_t_len: R16Uep0TLen,
    r8_uep0_tx_ctrl: R8Uep0TxCtrl,
    r8_uep0_rx_ctrl: R8Uep0RxCtrl,
    r16_uep1_t_len: R16Uep1TLen,
    r8_uep1_tx_ctrl: R8Uep1TxCtrl,
    r8_uep1_rx_ctrl: R8Uep1RxCtrl,
    r16_uep2_t_len_r16_uh_ep_pid: R16Uep2TLenR16UhEpPid,
    r8_uep2_tx_ctrl: R8Uep2TxCtrl,
    r8_uep2_rx_ctrl_r8_uh_rx_ctrl: R8Uep2RxCtrlR8UhRxCtrl,
    r16_uep3_t_len_r16_uh_tx_len: R16Uep3TLenR16UhTxLen,
    r8_uep3_tx_ctrl_r8_uh_tx_ctrl: R8Uep3TxCtrlR8UhTxCtrl,
    r8_uep3_rx_ctrl: R8Uep3RxCtrl,
    r16_uep4_t_len_r16_uh_split_data: R16Uep4TLenR16UhSplitData,
    r8_uep4_tx_ctrl: R8Uep4TxCtrl,
    r8_uep4_rx_ctrl: R8Uep4RxCtrl,
    r16_uep5_t_len: R16Uep5TLen,
    r8_uep5_tx_ctrl: R8Uep5TxCtrl,
    r8_uep5_rx_ctrl: R8Uep5RxCtrl,
    r16_uep6_t_len: R16Uep6TLen,
    r8_uep6_tx_ctrl: R8Uep6TxCtrl,
    r8_uep6_rx_ctrl: R8Uep6RxCtrl,
    r16_uep7_t_len: R16Uep7TLen,
    r8_uep7_tx_ctrl: R8Uep7TxCtrl,
    r8_uep7_rx_ctrl: R8Uep7RxCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - USB base control"]
    #[inline(always)]
    pub const fn r8_usb_ctrl(&self) -> &R8UsbCtrl {
        &self.r8_usb_ctrl
    }
    #[doc = "0x01 - USB host control register"]
    #[inline(always)]
    pub const fn r8_uhost_ctrl(&self) -> &R8UhostCtrl {
        &self.r8_uhost_ctrl
    }
    #[doc = "0x02 - USB interrupt enable"]
    #[inline(always)]
    pub const fn r8_usb_int_en(&self) -> &R8UsbIntEn {
        &self.r8_usb_int_en
    }
    #[doc = "0x03 - USB device address"]
    #[inline(always)]
    pub const fn r8_usb_dev_ad(&self) -> &R8UsbDevAd {
        &self.r8_usb_dev_ad
    }
    #[doc = "0x04 - USB frame number register"]
    #[inline(always)]
    pub const fn r16_usb_frame_no(&self) -> &R16UsbFrameNo {
        &self.r16_usb_frame_no
    }
    #[doc = "0x06 - USB suspend register"]
    #[inline(always)]
    pub const fn r8_usb_suspend(&self) -> &R8UsbSuspend {
        &self.r8_usb_suspend
    }
    #[doc = "0x08 - USB actual speed register"]
    #[inline(always)]
    pub const fn r8_usb_spd_type(&self) -> &R8UsbSpdType {
        &self.r8_usb_spd_type
    }
    #[doc = "0x09 - USB miscellaneous status"]
    #[inline(always)]
    pub const fn r8_usb_mis_st(&self) -> &R8UsbMisSt {
        &self.r8_usb_mis_st
    }
    #[doc = "0x0a - USB interrupt flag"]
    #[inline(always)]
    pub const fn r8_usb_int_fg(&self) -> &R8UsbIntFg {
        &self.r8_usb_int_fg
    }
    #[doc = "0x0b - USB interrupt status"]
    #[inline(always)]
    pub const fn r8_usb_int_st(&self) -> &R8UsbIntSt {
        &self.r8_usb_int_st
    }
    #[doc = "0x0c - USB receiving length"]
    #[inline(always)]
    pub const fn r6_usb_rx_len(&self) -> &R6UsbRxLen {
        &self.r6_usb_rx_len
    }
    #[doc = "0x10 - endpoint 1(9)/4(8/12) mode"]
    #[inline(always)]
    pub const fn r8_uep4_1_mod(&self) -> &R8Uep4_1Mod {
        &self.r8_uep4_1_mod
    }
    #[doc = "0x11 - endpoint 2(10)/3(11) mode / USB host endpoint mode control register"]
    #[inline(always)]
    pub const fn r8_uep2_3_mod_r8_uh_ep_mod(&self) -> &R8Uep2_3ModR8UhEpMod {
        &self.r8_uep2_3_mod_r8_uh_ep_mod
    }
    #[doc = "0x12 - endpoint 5(13)/6(14) mode"]
    #[inline(always)]
    pub const fn r8_uep5_6_mod(&self) -> &R8Uep5_6Mod {
        &self.r8_uep5_6_mod
    }
    #[doc = "0x13 - endpoint 7(15) mode"]
    #[inline(always)]
    pub const fn r8_uep7_mod(&self) -> &R8Uep7Mod {
        &self.r8_uep7_mod
    }
    #[doc = "0x14 - endpoint 0 DMA buffer address"]
    #[inline(always)]
    pub const fn r32_uep0_rt_dma(&self) -> &R32Uep0RtDma {
        &self.r32_uep0_rt_dma
    }
    #[doc = "0x18 - endpoint 1 DMA buffer address"]
    #[inline(always)]
    pub const fn r32_uep1_rx_dma(&self) -> &R32Uep1RxDma {
        &self.r32_uep1_rx_dma
    }
    #[doc = "0x1c - endpoint 2 DMA buffer address / host rx endpoint buffer start address"]
    #[inline(always)]
    pub const fn r32_uep2_rx_dma_r32_uh_rx_dma(&self) -> &R32Uep2RxDmaR32UhRxDma {
        &self.r32_uep2_rx_dma_r32_uh_rx_dma
    }
    #[doc = "0x20 - endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
    #[inline(always)]
    pub const fn r32_uep3_rx_dma(&self) -> &R32Uep3RxDma {
        &self.r32_uep3_rx_dma
    }
    #[doc = "0x24 - endpoint 4 DMA buffer address"]
    #[inline(always)]
    pub const fn r32_uep4_rx_dma(&self) -> &R32Uep4RxDma {
        &self.r32_uep4_rx_dma
    }
    #[doc = "0x28 - endpoint 5 DMA buffer address"]
    #[inline(always)]
    pub const fn r32_uep5_rx_dma(&self) -> &R32Uep5RxDma {
        &self.r32_uep5_rx_dma
    }
    #[doc = "0x2c - endpoint 6 DMA buffer address"]
    #[inline(always)]
    pub const fn r32_uep6_rx_dma(&self) -> &R32Uep6RxDma {
        &self.r32_uep6_rx_dma
    }
    #[doc = "0x30 - endpoint 7 DMA buffer address"]
    #[inline(always)]
    pub const fn r32_uep7_rx_dma(&self) -> &R32Uep7RxDma {
        &self.r32_uep7_rx_dma
    }
    #[doc = "0x34 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub const fn r32_uep1_tx_dma(&self) -> &R32Uep1TxDma {
        &self.r32_uep1_tx_dma
    }
    #[doc = "0x38 - endpoint 2 DMA TX buffer address"]
    #[inline(always)]
    pub const fn r32_uep2_tx_dma(&self) -> &R32Uep2TxDma {
        &self.r32_uep2_tx_dma
    }
    #[doc = "0x3c - endpoint 3 DMA TX buffer address / host tx endpoint buffer start address"]
    #[inline(always)]
    pub const fn r32_uep3_tx_dma_r32_uh_tx_dma(&self) -> &R32Uep3TxDmaR32UhTxDma {
        &self.r32_uep3_tx_dma_r32_uh_tx_dma
    }
    #[doc = "0x40 - endpoint 4 DMA TX buffer address"]
    #[inline(always)]
    pub const fn r32_uep4_tx_dma(&self) -> &R32Uep4TxDma {
        &self.r32_uep4_tx_dma
    }
    #[doc = "0x44 - endpoint 5 DMA TX buffer address"]
    #[inline(always)]
    pub const fn r32_uep5_tx_dma(&self) -> &R32Uep5TxDma {
        &self.r32_uep5_tx_dma
    }
    #[doc = "0x48 - endpoint 6 DMA TX buffer address"]
    #[inline(always)]
    pub const fn r32_uep6_tx_dma(&self) -> &R32Uep6TxDma {
        &self.r32_uep6_tx_dma
    }
    #[doc = "0x4c - endpoint 7 DMA TX buffer address"]
    #[inline(always)]
    pub const fn r32_uep7_tx_dma(&self) -> &R32Uep7TxDma {
        &self.r32_uep7_tx_dma
    }
    #[doc = "0x50 - endpoint 0 receive max length"]
    #[inline(always)]
    pub const fn r16_uep0_max_len(&self) -> &R16Uep0MaxLen {
        &self.r16_uep0_max_len
    }
    #[doc = "0x54 - endpoint 1 receive max length"]
    #[inline(always)]
    pub const fn r16_uep1_max_len(&self) -> &R16Uep1MaxLen {
        &self.r16_uep1_max_len
    }
    #[doc = "0x58 - endpoint 2 receive max length / USB host receive max packet length register"]
    #[inline(always)]
    pub const fn r16_uep2_max_len_r16_uh_max_len(&self) -> &R16Uep2MaxLenR16UhMaxLen {
        &self.r16_uep2_max_len_r16_uh_max_len
    }
    #[doc = "0x5c - endpoint 3 receive max length"]
    #[inline(always)]
    pub const fn r16_uep3_max_len(&self) -> &R16Uep3MaxLen {
        &self.r16_uep3_max_len
    }
    #[doc = "0x60 - endpoint 4 receive max length"]
    #[inline(always)]
    pub const fn r16_uep4_max_len(&self) -> &R16Uep4MaxLen {
        &self.r16_uep4_max_len
    }
    #[doc = "0x64 - endpoint 5 receive max length"]
    #[inline(always)]
    pub const fn r16_uep5_max_len(&self) -> &R16Uep5MaxLen {
        &self.r16_uep5_max_len
    }
    #[doc = "0x68 - endpoint 6 receive max length"]
    #[inline(always)]
    pub const fn r16_uep6_max_len(&self) -> &R16Uep6MaxLen {
        &self.r16_uep6_max_len
    }
    #[doc = "0x6c - endpoint 7 receive max length"]
    #[inline(always)]
    pub const fn r16_uep7_max_len(&self) -> &R16Uep7MaxLen {
        &self.r16_uep7_max_len
    }
    #[doc = "0x70 - endpoint 0 transmittal length"]
    #[inline(always)]
    pub const fn r16_uep0_t_len(&self) -> &R16Uep0TLen {
        &self.r16_uep0_t_len
    }
    #[doc = "0x72 - endpoint 0 tx control"]
    #[inline(always)]
    pub const fn r8_uep0_tx_ctrl(&self) -> &R8Uep0TxCtrl {
        &self.r8_uep0_tx_ctrl
    }
    #[doc = "0x73 - endpoint 0 rx control"]
    #[inline(always)]
    pub const fn r8_uep0_rx_ctrl(&self) -> &R8Uep0RxCtrl {
        &self.r8_uep0_rx_ctrl
    }
    #[doc = "0x74 - endpoint 1 transmittal length"]
    #[inline(always)]
    pub const fn r16_uep1_t_len(&self) -> &R16Uep1TLen {
        &self.r16_uep1_t_len
    }
    #[doc = "0x76 - endpoint 1 tx control"]
    #[inline(always)]
    pub const fn r8_uep1_tx_ctrl(&self) -> &R8Uep1TxCtrl {
        &self.r8_uep1_tx_ctrl
    }
    #[doc = "0x77 - endpoint 1 rx control"]
    #[inline(always)]
    pub const fn r8_uep1_rx_ctrl(&self) -> &R8Uep1RxCtrl {
        &self.r8_uep1_rx_ctrl
    }
    #[doc = "0x78 - endpoint 2 transmittal length / Set usb host token register"]
    #[inline(always)]
    pub const fn r16_uep2_t_len_r16_uh_ep_pid(&self) -> &R16Uep2TLenR16UhEpPid {
        &self.r16_uep2_t_len_r16_uh_ep_pid
    }
    #[doc = "0x7a - endpoint 2 tx control"]
    #[inline(always)]
    pub const fn r8_uep2_tx_ctrl(&self) -> &R8Uep2TxCtrl {
        &self.r8_uep2_tx_ctrl
    }
    #[doc = "0x7b - endpoint 2 rx control / USb host receive endpoint control register"]
    #[inline(always)]
    pub const fn r8_uep2_rx_ctrl_r8_uh_rx_ctrl(&self) -> &R8Uep2RxCtrlR8UhRxCtrl {
        &self.r8_uep2_rx_ctrl_r8_uh_rx_ctrl
    }
    #[doc = "0x7c - endpoint 3 transmittal length / host transmittal endpoint transmittal length"]
    #[inline(always)]
    pub const fn r16_uep3_t_len_r16_uh_tx_len(&self) -> &R16Uep3TLenR16UhTxLen {
        &self.r16_uep3_t_len_r16_uh_tx_len
    }
    #[doc = "0x7e - endpoint 3 tx control / host transmittal endpoint control"]
    #[inline(always)]
    pub const fn r8_uep3_tx_ctrl_r8_uh_tx_ctrl(&self) -> &R8Uep3TxCtrlR8UhTxCtrl {
        &self.r8_uep3_tx_ctrl_r8_uh_tx_ctrl
    }
    #[doc = "0x7f - endpoint 3 rx control"]
    #[inline(always)]
    pub const fn r8_uep3_rx_ctrl(&self) -> &R8Uep3RxCtrl {
        &self.r8_uep3_rx_ctrl
    }
    #[doc = "0x80 - endpoint 4 transmittal length / USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub const fn r16_uep4_t_len_r16_uh_split_data(&self) -> &R16Uep4TLenR16UhSplitData {
        &self.r16_uep4_t_len_r16_uh_split_data
    }
    #[doc = "0x82 - endpoint 4 tx control"]
    #[inline(always)]
    pub const fn r8_uep4_tx_ctrl(&self) -> &R8Uep4TxCtrl {
        &self.r8_uep4_tx_ctrl
    }
    #[doc = "0x83 - endpoint 4 rx control"]
    #[inline(always)]
    pub const fn r8_uep4_rx_ctrl(&self) -> &R8Uep4RxCtrl {
        &self.r8_uep4_rx_ctrl
    }
    #[doc = "0x84 - endpoint 5 transmittal length"]
    #[inline(always)]
    pub const fn r16_uep5_t_len(&self) -> &R16Uep5TLen {
        &self.r16_uep5_t_len
    }
    #[doc = "0x86 - endpoint 5 tx control"]
    #[inline(always)]
    pub const fn r8_uep5_tx_ctrl(&self) -> &R8Uep5TxCtrl {
        &self.r8_uep5_tx_ctrl
    }
    #[doc = "0x87 - endpoint 5 rx control"]
    #[inline(always)]
    pub const fn r8_uep5_rx_ctrl(&self) -> &R8Uep5RxCtrl {
        &self.r8_uep5_rx_ctrl
    }
    #[doc = "0x88 - endpoint 6 transmittal length"]
    #[inline(always)]
    pub const fn r16_uep6_t_len(&self) -> &R16Uep6TLen {
        &self.r16_uep6_t_len
    }
    #[doc = "0x8a - endpoint 6 tx control"]
    #[inline(always)]
    pub const fn r8_uep6_tx_ctrl(&self) -> &R8Uep6TxCtrl {
        &self.r8_uep6_tx_ctrl
    }
    #[doc = "0x8b - endpoint 6 rx control"]
    #[inline(always)]
    pub const fn r8_uep6_rx_ctrl(&self) -> &R8Uep6RxCtrl {
        &self.r8_uep6_rx_ctrl
    }
    #[doc = "0x8c - endpoint 7 transmittal length"]
    #[inline(always)]
    pub const fn r16_uep7_t_len(&self) -> &R16Uep7TLen {
        &self.r16_uep7_t_len
    }
    #[doc = "0x8e - endpoint 7 tx control"]
    #[inline(always)]
    pub const fn r8_uep7_tx_ctrl(&self) -> &R8Uep7TxCtrl {
        &self.r8_uep7_tx_ctrl
    }
    #[doc = "0x8f - endpoint 7 rx control"]
    #[inline(always)]
    pub const fn r8_uep7_rx_ctrl(&self) -> &R8Uep7RxCtrl {
        &self.r8_uep7_rx_ctrl
    }
}
#[doc = "R8_USB_CTRL (rw) register accessor: USB base control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_ctrl`] module"]
#[doc(alias = "R8_USB_CTRL")]
pub type R8UsbCtrl = crate::Reg<r8_usb_ctrl::R8UsbCtrlSpec>;
#[doc = "USB base control"]
pub mod r8_usb_ctrl;
#[doc = "R8_UHOST_CTRL (rw) register accessor: USB host control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uhost_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uhost_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uhost_ctrl`] module"]
#[doc(alias = "R8_UHOST_CTRL")]
pub type R8UhostCtrl = crate::Reg<r8_uhost_ctrl::R8UhostCtrlSpec>;
#[doc = "USB host control register"]
pub mod r8_uhost_ctrl;
#[doc = "R8_USB_INT_EN (rw) register accessor: USB interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_en`] module"]
#[doc(alias = "R8_USB_INT_EN")]
pub type R8UsbIntEn = crate::Reg<r8_usb_int_en::R8UsbIntEnSpec>;
#[doc = "USB interrupt enable"]
pub mod r8_usb_int_en;
#[doc = "R8_USB_DEV_AD (rw) register accessor: USB device address\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_dev_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_dev_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_dev_ad`] module"]
#[doc(alias = "R8_USB_DEV_AD")]
pub type R8UsbDevAd = crate::Reg<r8_usb_dev_ad::R8UsbDevAdSpec>;
#[doc = "USB device address"]
pub mod r8_usb_dev_ad;
#[doc = "R16_USB_FRAME_NO (r) register accessor: USB frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_usb_frame_no::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_usb_frame_no`] module"]
#[doc(alias = "R16_USB_FRAME_NO")]
pub type R16UsbFrameNo = crate::Reg<r16_usb_frame_no::R16UsbFrameNoSpec>;
#[doc = "USB frame number register"]
pub mod r16_usb_frame_no;
#[doc = "R8_USB_SUSPEND (rw) register accessor: USB suspend register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_suspend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_suspend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_suspend`] module"]
#[doc(alias = "R8_USB_SUSPEND")]
pub type R8UsbSuspend = crate::Reg<r8_usb_suspend::R8UsbSuspendSpec>;
#[doc = "USB suspend register"]
pub mod r8_usb_suspend;
#[doc = "R8_USB_SPD_TYPE (r) register accessor: USB actual speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_spd_type::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_spd_type`] module"]
#[doc(alias = "R8_USB_SPD_TYPE")]
pub type R8UsbSpdType = crate::Reg<r8_usb_spd_type::R8UsbSpdTypeSpec>;
#[doc = "USB actual speed register"]
pub mod r8_usb_spd_type;
#[doc = "R8_USB_MIS_ST (r) register accessor: USB miscellaneous status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_mis_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_mis_st`] module"]
#[doc(alias = "R8_USB_MIS_ST")]
pub type R8UsbMisSt = crate::Reg<r8_usb_mis_st::R8UsbMisStSpec>;
#[doc = "USB miscellaneous status"]
pub mod r8_usb_mis_st;
#[doc = "R8_USB_INT_FG (rw) register accessor: USB interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_fg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_fg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_fg`] module"]
#[doc(alias = "R8_USB_INT_FG")]
pub type R8UsbIntFg = crate::Reg<r8_usb_int_fg::R8UsbIntFgSpec>;
#[doc = "USB interrupt flag"]
pub mod r8_usb_int_fg;
#[doc = "R8_USB_INT_ST (r) register accessor: USB interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_st`] module"]
#[doc(alias = "R8_USB_INT_ST")]
pub type R8UsbIntSt = crate::Reg<r8_usb_int_st::R8UsbIntStSpec>;
#[doc = "USB interrupt status"]
pub mod r8_usb_int_st;
#[doc = "R6_USB_RX_LEN (r) register accessor: USB receiving length\n\nYou can [`read`](crate::Reg::read) this register and get [`r6_usb_rx_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r6_usb_rx_len`] module"]
#[doc(alias = "R6_USB_RX_LEN")]
pub type R6UsbRxLen = crate::Reg<r6_usb_rx_len::R6UsbRxLenSpec>;
#[doc = "USB receiving length"]
pub mod r6_usb_rx_len;
#[doc = "R8_UEP4_1_MOD (rw) register accessor: endpoint 1(9)/4(8/12) mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_1_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_1_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_1_mod`] module"]
#[doc(alias = "R8_UEP4_1_MOD")]
pub type R8Uep4_1Mod = crate::Reg<r8_uep4_1_mod::R8Uep4_1ModSpec>;
#[doc = "endpoint 1(9)/4(8/12) mode"]
pub mod r8_uep4_1_mod;
#[doc = "R8_UEP2_3_MOD_R8_UH_EP_MOD (rw) register accessor: endpoint 2(10)/3(11) mode / USB host endpoint mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_3_mod_r8_uh_ep_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_3_mod_r8_uh_ep_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_3_mod_r8_uh_ep_mod`] module"]
#[doc(alias = "R8_UEP2_3_MOD_R8_UH_EP_MOD")]
pub type R8Uep2_3ModR8UhEpMod = crate::Reg<r8_uep2_3_mod_r8_uh_ep_mod::R8Uep2_3ModR8UhEpModSpec>;
#[doc = "endpoint 2(10)/3(11) mode / USB host endpoint mode control register"]
pub mod r8_uep2_3_mod_r8_uh_ep_mod;
#[doc = "R8_UEP5_6_MOD (rw) register accessor: endpoint 5(13)/6(14) mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep5_6_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep5_6_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_6_mod`] module"]
#[doc(alias = "R8_UEP5_6_MOD")]
pub type R8Uep5_6Mod = crate::Reg<r8_uep5_6_mod::R8Uep5_6ModSpec>;
#[doc = "endpoint 5(13)/6(14) mode"]
pub mod r8_uep5_6_mod;
#[doc = "R8_UEP7_MOD (rw) register accessor: endpoint 7(15) mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep7_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep7_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_mod`] module"]
#[doc(alias = "R8_UEP7_MOD")]
pub type R8Uep7Mod = crate::Reg<r8_uep7_mod::R8Uep7ModSpec>;
#[doc = "endpoint 7(15) mode"]
pub mod r8_uep7_mod;
#[doc = "R32_UEP0_RT_DMA (rw) register accessor: endpoint 0 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep0_rt_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep0_rt_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep0_rt_dma`] module"]
#[doc(alias = "R32_UEP0_RT_DMA")]
pub type R32Uep0RtDma = crate::Reg<r32_uep0_rt_dma::R32Uep0RtDmaSpec>;
#[doc = "endpoint 0 DMA buffer address"]
pub mod r32_uep0_rt_dma;
#[doc = "R32_UEP1_RX_DMA (rw) register accessor: endpoint 1 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep1_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep1_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep1_rx_dma`] module"]
#[doc(alias = "R32_UEP1_RX_DMA")]
pub type R32Uep1RxDma = crate::Reg<r32_uep1_rx_dma::R32Uep1RxDmaSpec>;
#[doc = "endpoint 1 DMA buffer address"]
pub mod r32_uep1_rx_dma;
#[doc = "R32_UEP2_RX_DMA_R32_UH_RX_DMA (rw) register accessor: endpoint 2 DMA buffer address / host rx endpoint buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep2_rx_dma_r32_uh_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep2_rx_dma_r32_uh_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep2_rx_dma_r32_uh_rx_dma`] module"]
#[doc(alias = "R32_UEP2_RX_DMA_R32_UH_RX_DMA")]
pub type R32Uep2RxDmaR32UhRxDma =
    crate::Reg<r32_uep2_rx_dma_r32_uh_rx_dma::R32Uep2RxDmaR32UhRxDmaSpec>;
#[doc = "endpoint 2 DMA buffer address / host rx endpoint buffer start address"]
pub mod r32_uep2_rx_dma_r32_uh_rx_dma;
#[doc = "R32_UEP3_RX_DMA (rw) register accessor: endpoint 3 DMA buffer address;host tx endpoint buffer high address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep3_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep3_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep3_rx_dma`] module"]
#[doc(alias = "R32_UEP3_RX_DMA")]
pub type R32Uep3RxDma = crate::Reg<r32_uep3_rx_dma::R32Uep3RxDmaSpec>;
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
pub mod r32_uep3_rx_dma;
#[doc = "R32_UEP4_RX_DMA (rw) register accessor: endpoint 4 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep4_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep4_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep4_rx_dma`] module"]
#[doc(alias = "R32_UEP4_RX_DMA")]
pub type R32Uep4RxDma = crate::Reg<r32_uep4_rx_dma::R32Uep4RxDmaSpec>;
#[doc = "endpoint 4 DMA buffer address"]
pub mod r32_uep4_rx_dma;
#[doc = "R32_UEP5_RX_DMA (rw) register accessor: endpoint 5 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep5_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep5_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep5_rx_dma`] module"]
#[doc(alias = "R32_UEP5_RX_DMA")]
pub type R32Uep5RxDma = crate::Reg<r32_uep5_rx_dma::R32Uep5RxDmaSpec>;
#[doc = "endpoint 5 DMA buffer address"]
pub mod r32_uep5_rx_dma;
#[doc = "R32_UEP6_RX_DMA (rw) register accessor: endpoint 6 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep6_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep6_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep6_rx_dma`] module"]
#[doc(alias = "R32_UEP6_RX_DMA")]
pub type R32Uep6RxDma = crate::Reg<r32_uep6_rx_dma::R32Uep6RxDmaSpec>;
#[doc = "endpoint 6 DMA buffer address"]
pub mod r32_uep6_rx_dma;
#[doc = "R32_UEP7_RX_DMA (rw) register accessor: endpoint 7 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep7_rx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep7_rx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep7_rx_dma`] module"]
#[doc(alias = "R32_UEP7_RX_DMA")]
pub type R32Uep7RxDma = crate::Reg<r32_uep7_rx_dma::R32Uep7RxDmaSpec>;
#[doc = "endpoint 7 DMA buffer address"]
pub mod r32_uep7_rx_dma;
#[doc = "R32_UEP1_TX_DMA (rw) register accessor: endpoint 1 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep1_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep1_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep1_tx_dma`] module"]
#[doc(alias = "R32_UEP1_TX_DMA")]
pub type R32Uep1TxDma = crate::Reg<r32_uep1_tx_dma::R32Uep1TxDmaSpec>;
#[doc = "endpoint 1 DMA TX buffer address"]
pub mod r32_uep1_tx_dma;
#[doc = "R32_UEP2_TX_DMA (rw) register accessor: endpoint 2 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep2_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep2_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep2_tx_dma`] module"]
#[doc(alias = "R32_UEP2_TX_DMA")]
pub type R32Uep2TxDma = crate::Reg<r32_uep2_tx_dma::R32Uep2TxDmaSpec>;
#[doc = "endpoint 2 DMA TX buffer address"]
pub mod r32_uep2_tx_dma;
#[doc = "R32_UEP3_TX_DMA_R32_UH_TX_DMA (rw) register accessor: endpoint 3 DMA TX buffer address / host tx endpoint buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep3_tx_dma_r32_uh_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep3_tx_dma_r32_uh_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep3_tx_dma_r32_uh_tx_dma`] module"]
#[doc(alias = "R32_UEP3_TX_DMA_R32_UH_TX_DMA")]
pub type R32Uep3TxDmaR32UhTxDma =
    crate::Reg<r32_uep3_tx_dma_r32_uh_tx_dma::R32Uep3TxDmaR32UhTxDmaSpec>;
#[doc = "endpoint 3 DMA TX buffer address / host tx endpoint buffer start address"]
pub mod r32_uep3_tx_dma_r32_uh_tx_dma;
#[doc = "R32_UEP4_TX_DMA (rw) register accessor: endpoint 4 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep4_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep4_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep4_tx_dma`] module"]
#[doc(alias = "R32_UEP4_TX_DMA")]
pub type R32Uep4TxDma = crate::Reg<r32_uep4_tx_dma::R32Uep4TxDmaSpec>;
#[doc = "endpoint 4 DMA TX buffer address"]
pub mod r32_uep4_tx_dma;
#[doc = "R32_UEP5_TX_DMA (rw) register accessor: endpoint 5 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep5_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep5_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep5_tx_dma`] module"]
#[doc(alias = "R32_UEP5_TX_DMA")]
pub type R32Uep5TxDma = crate::Reg<r32_uep5_tx_dma::R32Uep5TxDmaSpec>;
#[doc = "endpoint 5 DMA TX buffer address"]
pub mod r32_uep5_tx_dma;
#[doc = "R32_UEP6_TX_DMA (rw) register accessor: endpoint 6 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep6_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep6_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep6_tx_dma`] module"]
#[doc(alias = "R32_UEP6_TX_DMA")]
pub type R32Uep6TxDma = crate::Reg<r32_uep6_tx_dma::R32Uep6TxDmaSpec>;
#[doc = "endpoint 6 DMA TX buffer address"]
pub mod r32_uep6_tx_dma;
#[doc = "R32_UEP7_TX_DMA (rw) register accessor: endpoint 7 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep7_tx_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep7_tx_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep7_tx_dma`] module"]
#[doc(alias = "R32_UEP7_TX_DMA")]
pub type R32Uep7TxDma = crate::Reg<r32_uep7_tx_dma::R32Uep7TxDmaSpec>;
#[doc = "endpoint 7 DMA TX buffer address"]
pub mod r32_uep7_tx_dma;
#[doc = "R16_UEP0_MAX_LEN (rw) register accessor: endpoint 0 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep0_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep0_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep0_max_len`] module"]
#[doc(alias = "R16_UEP0_MAX_LEN")]
pub type R16Uep0MaxLen = crate::Reg<r16_uep0_max_len::R16Uep0MaxLenSpec>;
#[doc = "endpoint 0 receive max length"]
pub mod r16_uep0_max_len;
#[doc = "R16_UEP1_MAX_LEN (rw) register accessor: endpoint 1 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep1_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep1_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep1_max_len`] module"]
#[doc(alias = "R16_UEP1_MAX_LEN")]
pub type R16Uep1MaxLen = crate::Reg<r16_uep1_max_len::R16Uep1MaxLenSpec>;
#[doc = "endpoint 1 receive max length"]
pub mod r16_uep1_max_len;
#[doc = "R16_UEP2_MAX_LEN_R16_UH_MAX_LEN (rw) register accessor: endpoint 2 receive max length / USB host receive max packet length register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep2_max_len_r16_uh_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep2_max_len_r16_uh_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep2_max_len_r16_uh_max_len`] module"]
#[doc(alias = "R16_UEP2_MAX_LEN_R16_UH_MAX_LEN")]
pub type R16Uep2MaxLenR16UhMaxLen =
    crate::Reg<r16_uep2_max_len_r16_uh_max_len::R16Uep2MaxLenR16UhMaxLenSpec>;
#[doc = "endpoint 2 receive max length / USB host receive max packet length register"]
pub mod r16_uep2_max_len_r16_uh_max_len;
#[doc = "R16_UEP3_MAX_LEN (rw) register accessor: endpoint 3 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep3_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep3_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep3_max_len`] module"]
#[doc(alias = "R16_UEP3_MAX_LEN")]
pub type R16Uep3MaxLen = crate::Reg<r16_uep3_max_len::R16Uep3MaxLenSpec>;
#[doc = "endpoint 3 receive max length"]
pub mod r16_uep3_max_len;
#[doc = "R16_UEP4_MAX_LEN (rw) register accessor: endpoint 4 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep4_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep4_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep4_max_len`] module"]
#[doc(alias = "R16_UEP4_MAX_LEN")]
pub type R16Uep4MaxLen = crate::Reg<r16_uep4_max_len::R16Uep4MaxLenSpec>;
#[doc = "endpoint 4 receive max length"]
pub mod r16_uep4_max_len;
#[doc = "R16_UEP5_MAX_LEN (rw) register accessor: endpoint 5 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep5_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep5_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep5_max_len`] module"]
#[doc(alias = "R16_UEP5_MAX_LEN")]
pub type R16Uep5MaxLen = crate::Reg<r16_uep5_max_len::R16Uep5MaxLenSpec>;
#[doc = "endpoint 5 receive max length"]
pub mod r16_uep5_max_len;
#[doc = "R16_UEP6_MAX_LEN (rw) register accessor: endpoint 6 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep6_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep6_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep6_max_len`] module"]
#[doc(alias = "R16_UEP6_MAX_LEN")]
pub type R16Uep6MaxLen = crate::Reg<r16_uep6_max_len::R16Uep6MaxLenSpec>;
#[doc = "endpoint 6 receive max length"]
pub mod r16_uep6_max_len;
#[doc = "R16_UEP7_MAX_LEN (rw) register accessor: endpoint 7 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep7_max_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep7_max_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep7_max_len`] module"]
#[doc(alias = "R16_UEP7_MAX_LEN")]
pub type R16Uep7MaxLen = crate::Reg<r16_uep7_max_len::R16Uep7MaxLenSpec>;
#[doc = "endpoint 7 receive max length"]
pub mod r16_uep7_max_len;
#[doc = "R16_UEP0_T_LEN (rw) register accessor: endpoint 0 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep0_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep0_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep0_t_len`] module"]
#[doc(alias = "R16_UEP0_T_LEN")]
pub type R16Uep0TLen = crate::Reg<r16_uep0_t_len::R16Uep0TLenSpec>;
#[doc = "endpoint 0 transmittal length"]
pub mod r16_uep0_t_len;
#[doc = "R8_UEP0_TX_CTRL (rw) register accessor: endpoint 0 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep0_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep0_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_tx_ctrl`] module"]
#[doc(alias = "R8_UEP0_TX_CTRL")]
pub type R8Uep0TxCtrl = crate::Reg<r8_uep0_tx_ctrl::R8Uep0TxCtrlSpec>;
#[doc = "endpoint 0 tx control"]
pub mod r8_uep0_tx_ctrl;
#[doc = "R8_UEP0_RX_CTRL (rw) register accessor: endpoint 0 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep0_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep0_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_rx_ctrl`] module"]
#[doc(alias = "R8_UEP0_RX_CTRL")]
pub type R8Uep0RxCtrl = crate::Reg<r8_uep0_rx_ctrl::R8Uep0RxCtrlSpec>;
#[doc = "endpoint 0 rx control"]
pub mod r8_uep0_rx_ctrl;
#[doc = "R16_UEP1_T_LEN (rw) register accessor: endpoint 1 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep1_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep1_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep1_t_len`] module"]
#[doc(alias = "R16_UEP1_T_LEN")]
pub type R16Uep1TLen = crate::Reg<r16_uep1_t_len::R16Uep1TLenSpec>;
#[doc = "endpoint 1 transmittal length"]
pub mod r16_uep1_t_len;
#[doc = "R8_UEP1_TX_CTRL (rw) register accessor: endpoint 1 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep1_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep1_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_tx_ctrl`] module"]
#[doc(alias = "R8_UEP1_TX_CTRL")]
pub type R8Uep1TxCtrl = crate::Reg<r8_uep1_tx_ctrl::R8Uep1TxCtrlSpec>;
#[doc = "endpoint 1 tx control"]
pub mod r8_uep1_tx_ctrl;
#[doc = "R8_UEP1_RX_CTRL (rw) register accessor: endpoint 1 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep1_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep1_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_rx_ctrl`] module"]
#[doc(alias = "R8_UEP1_RX_CTRL")]
pub type R8Uep1RxCtrl = crate::Reg<r8_uep1_rx_ctrl::R8Uep1RxCtrlSpec>;
#[doc = "endpoint 1 rx control"]
pub mod r8_uep1_rx_ctrl;
#[doc = "R16_UEP2_T_LEN_R16_UH_EP_PID (rw) register accessor: endpoint 2 transmittal length / Set usb host token register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep2_t_len_r16_uh_ep_pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep2_t_len_r16_uh_ep_pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep2_t_len_r16_uh_ep_pid`] module"]
#[doc(alias = "R16_UEP2_T_LEN_R16_UH_EP_PID")]
pub type R16Uep2TLenR16UhEpPid =
    crate::Reg<r16_uep2_t_len_r16_uh_ep_pid::R16Uep2TLenR16UhEpPidSpec>;
#[doc = "endpoint 2 transmittal length / Set usb host token register"]
pub mod r16_uep2_t_len_r16_uh_ep_pid;
#[doc = "R8_UEP2_TX_CTRL (rw) register accessor: endpoint 2 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_tx_ctrl`] module"]
#[doc(alias = "R8_UEP2_TX_CTRL")]
pub type R8Uep2TxCtrl = crate::Reg<r8_uep2_tx_ctrl::R8Uep2TxCtrlSpec>;
#[doc = "endpoint 2 tx control"]
pub mod r8_uep2_tx_ctrl;
#[doc = "R8_UEP2_RX_CTRL_R8_UH_RX_CTRL (rw) register accessor: endpoint 2 rx control / USb host receive endpoint control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_rx_ctrl_r8_uh_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_rx_ctrl_r8_uh_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_rx_ctrl_r8_uh_rx_ctrl`] module"]
#[doc(alias = "R8_UEP2_RX_CTRL_R8_UH_RX_CTRL")]
pub type R8Uep2RxCtrlR8UhRxCtrl =
    crate::Reg<r8_uep2_rx_ctrl_r8_uh_rx_ctrl::R8Uep2RxCtrlR8UhRxCtrlSpec>;
#[doc = "endpoint 2 rx control / USb host receive endpoint control register"]
pub mod r8_uep2_rx_ctrl_r8_uh_rx_ctrl;
#[doc = "R16_UEP3_T_LEN_R16_UH_TX_LEN (rw) register accessor: endpoint 3 transmittal length / host transmittal endpoint transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep3_t_len_r16_uh_tx_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep3_t_len_r16_uh_tx_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep3_t_len_r16_uh_tx_len`] module"]
#[doc(alias = "R16_UEP3_T_LEN_R16_UH_TX_LEN")]
pub type R16Uep3TLenR16UhTxLen =
    crate::Reg<r16_uep3_t_len_r16_uh_tx_len::R16Uep3TLenR16UhTxLenSpec>;
#[doc = "endpoint 3 transmittal length / host transmittal endpoint transmittal length"]
pub mod r16_uep3_t_len_r16_uh_tx_len;
#[doc = "R8_UEP3_TX_CTRL_R8_UH_TX_CTRL (rw) register accessor: endpoint 3 tx control / host transmittal endpoint control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_tx_ctrl_r8_uh_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_tx_ctrl_r8_uh_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_tx_ctrl_r8_uh_tx_ctrl`] module"]
#[doc(alias = "R8_UEP3_TX_CTRL_R8_UH_TX_CTRL")]
pub type R8Uep3TxCtrlR8UhTxCtrl =
    crate::Reg<r8_uep3_tx_ctrl_r8_uh_tx_ctrl::R8Uep3TxCtrlR8UhTxCtrlSpec>;
#[doc = "endpoint 3 tx control / host transmittal endpoint control"]
pub mod r8_uep3_tx_ctrl_r8_uh_tx_ctrl;
#[doc = "R8_UEP3_RX_CTRL (rw) register accessor: endpoint 3 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_rx_ctrl`] module"]
#[doc(alias = "R8_UEP3_RX_CTRL")]
pub type R8Uep3RxCtrl = crate::Reg<r8_uep3_rx_ctrl::R8Uep3RxCtrlSpec>;
#[doc = "endpoint 3 rx control"]
pub mod r8_uep3_rx_ctrl;
#[doc = "R16_UEP4_T_LEN_R16_UH_SPLIT_DATA (rw) register accessor: endpoint 4 transmittal length / USB host Tx SPLIT packet data\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep4_t_len_r16_uh_split_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep4_t_len_r16_uh_split_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep4_t_len_r16_uh_split_data`] module"]
#[doc(alias = "R16_UEP4_T_LEN_R16_UH_SPLIT_DATA")]
pub type R16Uep4TLenR16UhSplitData =
    crate::Reg<r16_uep4_t_len_r16_uh_split_data::R16Uep4TLenR16UhSplitDataSpec>;
#[doc = "endpoint 4 transmittal length / USB host Tx SPLIT packet data"]
pub mod r16_uep4_t_len_r16_uh_split_data;
#[doc = "R8_UEP4_TX_CTRL (rw) register accessor: endpoint 4 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_tx_ctrl`] module"]
#[doc(alias = "R8_UEP4_TX_CTRL")]
pub type R8Uep4TxCtrl = crate::Reg<r8_uep4_tx_ctrl::R8Uep4TxCtrlSpec>;
#[doc = "endpoint 4 tx control"]
pub mod r8_uep4_tx_ctrl;
#[doc = "R8_UEP4_RX_CTRL (rw) register accessor: endpoint 4 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_rx_ctrl`] module"]
#[doc(alias = "R8_UEP4_RX_CTRL")]
pub type R8Uep4RxCtrl = crate::Reg<r8_uep4_rx_ctrl::R8Uep4RxCtrlSpec>;
#[doc = "endpoint 4 rx control"]
pub mod r8_uep4_rx_ctrl;
#[doc = "R16_UEP5_T_LEN (rw) register accessor: endpoint 5 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep5_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep5_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep5_t_len`] module"]
#[doc(alias = "R16_UEP5_T_LEN")]
pub type R16Uep5TLen = crate::Reg<r16_uep5_t_len::R16Uep5TLenSpec>;
#[doc = "endpoint 5 transmittal length"]
pub mod r16_uep5_t_len;
#[doc = "R8_UEP5_TX_CTRL (rw) register accessor: endpoint 5 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep5_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep5_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_tx_ctrl`] module"]
#[doc(alias = "R8_UEP5_TX_CTRL")]
pub type R8Uep5TxCtrl = crate::Reg<r8_uep5_tx_ctrl::R8Uep5TxCtrlSpec>;
#[doc = "endpoint 5 tx control"]
pub mod r8_uep5_tx_ctrl;
#[doc = "R8_UEP5_RX_CTRL (rw) register accessor: endpoint 5 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep5_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep5_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_rx_ctrl`] module"]
#[doc(alias = "R8_UEP5_RX_CTRL")]
pub type R8Uep5RxCtrl = crate::Reg<r8_uep5_rx_ctrl::R8Uep5RxCtrlSpec>;
#[doc = "endpoint 5 rx control"]
pub mod r8_uep5_rx_ctrl;
#[doc = "R16_UEP6_T_LEN (rw) register accessor: endpoint 6 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep6_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep6_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep6_t_len`] module"]
#[doc(alias = "R16_UEP6_T_LEN")]
pub type R16Uep6TLen = crate::Reg<r16_uep6_t_len::R16Uep6TLenSpec>;
#[doc = "endpoint 6 transmittal length"]
pub mod r16_uep6_t_len;
#[doc = "R8_UEP6_TX_CTRL (rw) register accessor: endpoint 6 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep6_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep6_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep6_tx_ctrl`] module"]
#[doc(alias = "R8_UEP6_TX_CTRL")]
pub type R8Uep6TxCtrl = crate::Reg<r8_uep6_tx_ctrl::R8Uep6TxCtrlSpec>;
#[doc = "endpoint 6 tx control"]
pub mod r8_uep6_tx_ctrl;
#[doc = "R8_UEP6_RX_CTRL (rw) register accessor: endpoint 6 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep6_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep6_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep6_rx_ctrl`] module"]
#[doc(alias = "R8_UEP6_RX_CTRL")]
pub type R8Uep6RxCtrl = crate::Reg<r8_uep6_rx_ctrl::R8Uep6RxCtrlSpec>;
#[doc = "endpoint 6 rx control"]
pub mod r8_uep6_rx_ctrl;
#[doc = "R16_UEP7_T_LEN (rw) register accessor: endpoint 7 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep7_t_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep7_t_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_uep7_t_len`] module"]
#[doc(alias = "R16_UEP7_T_LEN")]
pub type R16Uep7TLen = crate::Reg<r16_uep7_t_len::R16Uep7TLenSpec>;
#[doc = "endpoint 7 transmittal length"]
pub mod r16_uep7_t_len;
#[doc = "R8_UEP7_TX_CTRL (rw) register accessor: endpoint 7 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep7_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep7_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_tx_ctrl`] module"]
#[doc(alias = "R8_UEP7_TX_CTRL")]
pub type R8Uep7TxCtrl = crate::Reg<r8_uep7_tx_ctrl::R8Uep7TxCtrlSpec>;
#[doc = "endpoint 7 tx control"]
pub mod r8_uep7_tx_ctrl;
#[doc = "R8_UEP7_RX_CTRL (rw) register accessor: endpoint 7 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep7_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep7_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_rx_ctrl`] module"]
#[doc(alias = "R8_UEP7_RX_CTRL")]
pub type R8Uep7RxCtrl = crate::Reg<r8_uep7_rx_ctrl::R8Uep7RxCtrlSpec>;
#[doc = "endpoint 7 rx control"]
pub mod r8_uep7_rx_ctrl;
