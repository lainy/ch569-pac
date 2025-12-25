#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r16_ecec_ctrl: R16EcecCtrl,
    r8_ecdc_int_en: R8EcdcIntEn,
    _reserved2: [u8; 0x03],
    r8_ecdc_int_fg: R8EcdcIntFg,
    _reserved3: [u8; 0x01],
    r32_ecdc_key_255t224: R32EcdcKey255t224,
    r32_ecdc_key_223t192: R32EcdcKey223t192,
    r32_ecdc_key_191t160: R32EcdcKey191t160,
    r32_ecdc_key_159t128: R32EcdcKey159t128,
    r32_ecdc_key_127t96: R32EcdcKey127t96,
    r32_ecdc_key_95t64: R32EcdcKey95t64,
    r32_ecdc_key_63t32: R32EcdcKey63t32,
    r32_ecdc_key_31t0: R32EcdcKey31t0,
    r32_ecdc_iv_127t96: R32EcdcIv127t96,
    r32_ecdc_iv_95t64: R32EcdcIv95t64,
    r32_ecdc_iv_63t32: R32EcdcIv63t32,
    r32_ecdc_iv_31t0: R32EcdcIv31t0,
    _reserved15: [u8; 0x08],
    r32_ecdc_sgsd_127t96: R32EcdcSgsd127t96,
    r32_ecdc_sgsd_95t64: R32EcdcSgsd95t64,
    r32_ecdc_sgsd_63t32: R32EcdcSgsd63t32,
    r32_ecdc_sgsd_31t0: R32EcdcSgsd31t0,
    r32_ecdc_sgrt_127t96: R32EcdcSgrt127t96,
    r32_ecdc_sgrt_95t64: R32EcdcSgrt95t64,
    r32_ecdc_sgrt_63t32: R32EcdcSgrt63t32,
    rb_ecdc_sgrt_31t0: RbEcdcSgrt31t0,
    r32_ecdc_sram_addr: R32EcdcSramAddr,
    r32_ecdc_sram_len: R32EcdcSramLen,
}
impl RegisterBlock {
    #[doc = "0x00 - ECED AES/SM4 register"]
    #[inline(always)]
    pub const fn r16_ecec_ctrl(&self) -> &R16EcecCtrl {
        &self.r16_ecec_ctrl
    }
    #[doc = "0x02 - Interupt enable register"]
    #[inline(always)]
    pub const fn r8_ecdc_int_en(&self) -> &R8EcdcIntEn {
        &self.r8_ecdc_int_en
    }
    #[doc = "0x06 - Interupt flag register"]
    #[inline(always)]
    pub const fn r8_ecdc_int_fg(&self) -> &R8EcdcIntFg {
        &self.r8_ecdc_int_fg
    }
    #[doc = "0x08 - User key 224-255 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_255t224(&self) -> &R32EcdcKey255t224 {
        &self.r32_ecdc_key_255t224
    }
    #[doc = "0x0c - User key 192-223 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_223t192(&self) -> &R32EcdcKey223t192 {
        &self.r32_ecdc_key_223t192
    }
    #[doc = "0x10 - User key 160-191 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_191t160(&self) -> &R32EcdcKey191t160 {
        &self.r32_ecdc_key_191t160
    }
    #[doc = "0x14 - User key 128-159 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_159t128(&self) -> &R32EcdcKey159t128 {
        &self.r32_ecdc_key_159t128
    }
    #[doc = "0x18 - User key 96-127 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_127t96(&self) -> &R32EcdcKey127t96 {
        &self.r32_ecdc_key_127t96
    }
    #[doc = "0x1c - User key 64-95 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_95t64(&self) -> &R32EcdcKey95t64 {
        &self.r32_ecdc_key_95t64
    }
    #[doc = "0x20 - User key 32-63 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_63t32(&self) -> &R32EcdcKey63t32 {
        &self.r32_ecdc_key_63t32
    }
    #[doc = "0x24 - User key 0-31 register"]
    #[inline(always)]
    pub const fn r32_ecdc_key_31t0(&self) -> &R32EcdcKey31t0 {
        &self.r32_ecdc_key_31t0
    }
    #[doc = "0x28 - CTR mode count 96-127 register"]
    #[inline(always)]
    pub const fn r32_ecdc_iv_127t96(&self) -> &R32EcdcIv127t96 {
        &self.r32_ecdc_iv_127t96
    }
    #[doc = "0x2c - CTR mode count 64-95 register"]
    #[inline(always)]
    pub const fn r32_ecdc_iv_95t64(&self) -> &R32EcdcIv95t64 {
        &self.r32_ecdc_iv_95t64
    }
    #[doc = "0x30 - CTR mode count 32-63 register"]
    #[inline(always)]
    pub const fn r32_ecdc_iv_63t32(&self) -> &R32EcdcIv63t32 {
        &self.r32_ecdc_iv_63t32
    }
    #[doc = "0x34 - CTR mode count 0-31 register"]
    #[inline(always)]
    pub const fn r32_ecdc_iv_31t0(&self) -> &R32EcdcIv31t0 {
        &self.r32_ecdc_iv_31t0
    }
    #[doc = "0x40 - Single encryption and decryption of original data 96-127 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgsd_127t96(&self) -> &R32EcdcSgsd127t96 {
        &self.r32_ecdc_sgsd_127t96
    }
    #[doc = "0x44 - Single encryption and decryption of original data 64-95 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgsd_95t64(&self) -> &R32EcdcSgsd95t64 {
        &self.r32_ecdc_sgsd_95t64
    }
    #[doc = "0x48 - Single encryption and decryption of original data 32-63 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgsd_63t32(&self) -> &R32EcdcSgsd63t32 {
        &self.r32_ecdc_sgsd_63t32
    }
    #[doc = "0x4c - Single encryption and decryption of original data 0-31 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgsd_31t0(&self) -> &R32EcdcSgsd31t0 {
        &self.r32_ecdc_sgsd_31t0
    }
    #[doc = "0x50 - Single encryption and decryption result 96-127 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgrt_127t96(&self) -> &R32EcdcSgrt127t96 {
        &self.r32_ecdc_sgrt_127t96
    }
    #[doc = "0x54 - Single encryption and decryption result 64-95 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgrt_95t64(&self) -> &R32EcdcSgrt95t64 {
        &self.r32_ecdc_sgrt_95t64
    }
    #[doc = "0x58 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub const fn r32_ecdc_sgrt_63t32(&self) -> &R32EcdcSgrt63t32 {
        &self.r32_ecdc_sgrt_63t32
    }
    #[doc = "0x5c - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub const fn rb_ecdc_sgrt_31t0(&self) -> &RbEcdcSgrt31t0 {
        &self.rb_ecdc_sgrt_31t0
    }
    #[doc = "0x60 - encryption and decryption sram start address register"]
    #[inline(always)]
    pub const fn r32_ecdc_sram_addr(&self) -> &R32EcdcSramAddr {
        &self.r32_ecdc_sram_addr
    }
    #[doc = "0x64 - encryption and decryption sram size register"]
    #[inline(always)]
    pub const fn r32_ecdc_sram_len(&self) -> &R32EcdcSramLen {
        &self.r32_ecdc_sram_len
    }
}
#[doc = "R16_ECEC_CTRL (rw) register accessor: ECED AES/SM4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_ecec_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_ecec_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_ecec_ctrl`] module"]
#[doc(alias = "R16_ECEC_CTRL")]
pub type R16EcecCtrl = crate::Reg<r16_ecec_ctrl::R16EcecCtrlSpec>;
#[doc = "ECED AES/SM4 register"]
pub mod r16_ecec_ctrl;
#[doc = "R8_ECDC_INT_EN (rw) register accessor: Interupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_ecdc_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_ecdc_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_ecdc_int_en`] module"]
#[doc(alias = "R8_ECDC_INT_EN")]
pub type R8EcdcIntEn = crate::Reg<r8_ecdc_int_en::R8EcdcIntEnSpec>;
#[doc = "Interupt enable register"]
pub mod r8_ecdc_int_en;
#[doc = "R8_ECDC_INT_FG (rw) register accessor: Interupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_ecdc_int_fg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_ecdc_int_fg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_ecdc_int_fg`] module"]
#[doc(alias = "R8_ECDC_INT_FG")]
pub type R8EcdcIntFg = crate::Reg<r8_ecdc_int_fg::R8EcdcIntFgSpec>;
#[doc = "Interupt flag register"]
pub mod r8_ecdc_int_fg;
#[doc = "R32_ECDC_KEY_255T224 (rw) register accessor: User key 224-255 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_255t224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_255t224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_255t224`] module"]
#[doc(alias = "R32_ECDC_KEY_255T224")]
pub type R32EcdcKey255t224 = crate::Reg<r32_ecdc_key_255t224::R32EcdcKey255t224Spec>;
#[doc = "User key 224-255 register"]
pub mod r32_ecdc_key_255t224;
#[doc = "R32_ECDC_KEY_223T192 (rw) register accessor: User key 192-223 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_223t192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_223t192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_223t192`] module"]
#[doc(alias = "R32_ECDC_KEY_223T192")]
pub type R32EcdcKey223t192 = crate::Reg<r32_ecdc_key_223t192::R32EcdcKey223t192Spec>;
#[doc = "User key 192-223 register"]
pub mod r32_ecdc_key_223t192;
#[doc = "R32_ECDC_KEY_191T160 (rw) register accessor: User key 160-191 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_191t160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_191t160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_191t160`] module"]
#[doc(alias = "R32_ECDC_KEY_191T160")]
pub type R32EcdcKey191t160 = crate::Reg<r32_ecdc_key_191t160::R32EcdcKey191t160Spec>;
#[doc = "User key 160-191 register"]
pub mod r32_ecdc_key_191t160;
#[doc = "R32_ECDC_KEY_159T128 (rw) register accessor: User key 128-159 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_159t128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_159t128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_159t128`] module"]
#[doc(alias = "R32_ECDC_KEY_159T128")]
pub type R32EcdcKey159t128 = crate::Reg<r32_ecdc_key_159t128::R32EcdcKey159t128Spec>;
#[doc = "User key 128-159 register"]
pub mod r32_ecdc_key_159t128;
#[doc = "R32_ECDC_KEY_127T96 (rw) register accessor: User key 96-127 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_127t96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_127t96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_127t96`] module"]
#[doc(alias = "R32_ECDC_KEY_127T96")]
pub type R32EcdcKey127t96 = crate::Reg<r32_ecdc_key_127t96::R32EcdcKey127t96Spec>;
#[doc = "User key 96-127 register"]
pub mod r32_ecdc_key_127t96;
#[doc = "R32_ECDC_KEY_95T64 (rw) register accessor: User key 64-95 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_95t64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_95t64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_95t64`] module"]
#[doc(alias = "R32_ECDC_KEY_95T64")]
pub type R32EcdcKey95t64 = crate::Reg<r32_ecdc_key_95t64::R32EcdcKey95t64Spec>;
#[doc = "User key 64-95 register"]
pub mod r32_ecdc_key_95t64;
#[doc = "R32_ECDC_KEY_63T32 (rw) register accessor: User key 32-63 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_63t32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_63t32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_63t32`] module"]
#[doc(alias = "R32_ECDC_KEY_63T32")]
pub type R32EcdcKey63t32 = crate::Reg<r32_ecdc_key_63t32::R32EcdcKey63t32Spec>;
#[doc = "User key 32-63 register"]
pub mod r32_ecdc_key_63t32;
#[doc = "R32_ECDC_KEY_31T0 (rw) register accessor: User key 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_31t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_31t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_key_31t0`] module"]
#[doc(alias = "R32_ECDC_KEY_31T0")]
pub type R32EcdcKey31t0 = crate::Reg<r32_ecdc_key_31t0::R32EcdcKey31t0Spec>;
#[doc = "User key 0-31 register"]
pub mod r32_ecdc_key_31t0;
#[doc = "R32_ECDC_IV_127T96 (rw) register accessor: CTR mode count 96-127 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_iv_127t96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_iv_127t96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_iv_127t96`] module"]
#[doc(alias = "R32_ECDC_IV_127T96")]
pub type R32EcdcIv127t96 = crate::Reg<r32_ecdc_iv_127t96::R32EcdcIv127t96Spec>;
#[doc = "CTR mode count 96-127 register"]
pub mod r32_ecdc_iv_127t96;
#[doc = "R32_ECDC_IV_95T64 (rw) register accessor: CTR mode count 64-95 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_iv_95t64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_iv_95t64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_iv_95t64`] module"]
#[doc(alias = "R32_ECDC_IV_95T64")]
pub type R32EcdcIv95t64 = crate::Reg<r32_ecdc_iv_95t64::R32EcdcIv95t64Spec>;
#[doc = "CTR mode count 64-95 register"]
pub mod r32_ecdc_iv_95t64;
#[doc = "R32_ECDC_IV_63T32 (rw) register accessor: CTR mode count 32-63 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_iv_63t32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_iv_63t32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_iv_63t32`] module"]
#[doc(alias = "R32_ECDC_IV_63T32")]
pub type R32EcdcIv63t32 = crate::Reg<r32_ecdc_iv_63t32::R32EcdcIv63t32Spec>;
#[doc = "CTR mode count 32-63 register"]
pub mod r32_ecdc_iv_63t32;
#[doc = "R32_ECDC_IV_31T0 (rw) register accessor: CTR mode count 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_iv_31t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_iv_31t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_iv_31t0`] module"]
#[doc(alias = "R32_ECDC_IV_31T0")]
pub type R32EcdcIv31t0 = crate::Reg<r32_ecdc_iv_31t0::R32EcdcIv31t0Spec>;
#[doc = "CTR mode count 0-31 register"]
pub mod r32_ecdc_iv_31t0;
#[doc = "R32_ECDC_SGSD_127T96 (rw) register accessor: Single encryption and decryption of original data 96-127 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgsd_127t96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgsd_127t96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgsd_127t96`] module"]
#[doc(alias = "R32_ECDC_SGSD_127T96")]
pub type R32EcdcSgsd127t96 = crate::Reg<r32_ecdc_sgsd_127t96::R32EcdcSgsd127t96Spec>;
#[doc = "Single encryption and decryption of original data 96-127 register"]
pub mod r32_ecdc_sgsd_127t96;
#[doc = "R32_ECDC_SGSD_95T64 (rw) register accessor: Single encryption and decryption of original data 64-95 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgsd_95t64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgsd_95t64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgsd_95t64`] module"]
#[doc(alias = "R32_ECDC_SGSD_95T64")]
pub type R32EcdcSgsd95t64 = crate::Reg<r32_ecdc_sgsd_95t64::R32EcdcSgsd95t64Spec>;
#[doc = "Single encryption and decryption of original data 64-95 register"]
pub mod r32_ecdc_sgsd_95t64;
#[doc = "R32_ECDC_SGSD_63T32 (rw) register accessor: Single encryption and decryption of original data 32-63 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgsd_63t32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgsd_63t32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgsd_63t32`] module"]
#[doc(alias = "R32_ECDC_SGSD_63T32")]
pub type R32EcdcSgsd63t32 = crate::Reg<r32_ecdc_sgsd_63t32::R32EcdcSgsd63t32Spec>;
#[doc = "Single encryption and decryption of original data 32-63 register"]
pub mod r32_ecdc_sgsd_63t32;
#[doc = "R32_ECDC_SGSD_31T0 (rw) register accessor: Single encryption and decryption of original data 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgsd_31t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgsd_31t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgsd_31t0`] module"]
#[doc(alias = "R32_ECDC_SGSD_31T0")]
pub type R32EcdcSgsd31t0 = crate::Reg<r32_ecdc_sgsd_31t0::R32EcdcSgsd31t0Spec>;
#[doc = "Single encryption and decryption of original data 0-31 register"]
pub mod r32_ecdc_sgsd_31t0;
#[doc = "R32_ECDC_SGRT_127T96 (rw) register accessor: Single encryption and decryption result 96-127 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgrt_127t96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgrt_127t96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgrt_127t96`] module"]
#[doc(alias = "R32_ECDC_SGRT_127T96")]
pub type R32EcdcSgrt127t96 = crate::Reg<r32_ecdc_sgrt_127t96::R32EcdcSgrt127t96Spec>;
#[doc = "Single encryption and decryption result 96-127 register"]
pub mod r32_ecdc_sgrt_127t96;
#[doc = "R32_ECDC_SGRT_95T64 (rw) register accessor: Single encryption and decryption result 64-95 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgrt_95t64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgrt_95t64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgrt_95t64`] module"]
#[doc(alias = "R32_ECDC_SGRT_95T64")]
pub type R32EcdcSgrt95t64 = crate::Reg<r32_ecdc_sgrt_95t64::R32EcdcSgrt95t64Spec>;
#[doc = "Single encryption and decryption result 64-95 register"]
pub mod r32_ecdc_sgrt_95t64;
#[doc = "R32_ECDC_SGRT_63T32 (rw) register accessor: Single encryption and decryption result 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgrt_63t32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgrt_63t32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sgrt_63t32`] module"]
#[doc(alias = "R32_ECDC_SGRT_63T32")]
pub type R32EcdcSgrt63t32 = crate::Reg<r32_ecdc_sgrt_63t32::R32EcdcSgrt63t32Spec>;
#[doc = "Single encryption and decryption result 0-31 register"]
pub mod r32_ecdc_sgrt_63t32;
#[doc = "RB_ECDC_SGRT_31T0 (rw) register accessor: Single encryption and decryption result 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rb_ecdc_sgrt_31t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rb_ecdc_sgrt_31t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb_ecdc_sgrt_31t0`] module"]
#[doc(alias = "RB_ECDC_SGRT_31T0")]
pub type RbEcdcSgrt31t0 = crate::Reg<rb_ecdc_sgrt_31t0::RbEcdcSgrt31t0Spec>;
#[doc = "Single encryption and decryption result 0-31 register"]
pub mod rb_ecdc_sgrt_31t0;
#[doc = "R32_ECDC_SRAM_ADDR (rw) register accessor: encryption and decryption sram start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sram_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sram_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sram_addr`] module"]
#[doc(alias = "R32_ECDC_SRAM_ADDR")]
pub type R32EcdcSramAddr = crate::Reg<r32_ecdc_sram_addr::R32EcdcSramAddrSpec>;
#[doc = "encryption and decryption sram start address register"]
pub mod r32_ecdc_sram_addr;
#[doc = "R32_ECDC_SRAM_LEN (rw) register accessor: encryption and decryption sram size register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sram_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sram_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_ecdc_sram_len`] module"]
#[doc(alias = "R32_ECDC_SRAM_LEN")]
pub type R32EcdcSramLen = crate::Reg<r32_ecdc_sram_len::R32EcdcSramLenSpec>;
#[doc = "encryption and decryption sram size register"]
pub mod r32_ecdc_sram_len;
