#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_safe_access_sig: R8SafeAccessSig,
    r8_chip_id: R8ChipId,
    r8_safe_access_id: R8SafeAccessId,
    r8_wdog_count: R8WdogCount,
    r8_glob_rom_cfg: R8GlobRomCfg,
    r8_rst_boot_stat: R8RstBootStat,
    r8_rst_wdog_ctrl: R8RstWdogCtrl,
    r8_glob_reset_keep: R8GlobResetKeep,
    r8_clk_pll_div: R8ClkPllDiv,
    _reserved9: [u8; 0x01],
    r8_clk_cfg_ctrl: R8ClkCfgCtrl,
    r8_clk_mod_aux: R8ClkModAux,
    r8_slp_clk_off0: R8SlpClkOff0,
    r8_slp_clk_off1: R8SlpClkOff1,
    r8_slp_wake_ctrl: R8SlpWakeCtrl,
    r8_slp_power_ctrl: R8SlpPowerCtrl,
    r8_xbus_config: R8XbusConfig,
    _reserved16: [u8; 0x01],
    r8_pin_alternate: R8PinAlternate,
    _reserved17: [u8; 0x09],
    r8_gpio_int_flag: R8GpioIntFlag,
    r8_gpio_int_enable: R8GpioIntEnable,
    r8_gpio_int_mode: R8GpioIntMode,
    r8_gpio_int_polar: R8GpioIntPolar,
    r16_serd_ana_cfg1: R16SerdAnaCfg1,
    _reserved22: [u8; 0x02],
    r32_serd_ana_cfg2: R32SerdAnaCfg2,
    _reserved23: [u8; 0x18],
    r32_pa_dir: R32PaDir,
    r32_pa_pin: R32PaPin,
    r32_pa_out: R32PaOut,
    r32_pa_clr: R32PaClr,
    r32_pa_pu: R32PaPu,
    r32_pa_pd: R32PaPd,
    r32_pa_drv: R32PaDrv,
    r32_pa_smt: R32PaSmt,
    r32_pb_dir: R32PbDir,
    r32_pb_pin: R32PbPin,
    r32_pb_out: R32PbOut,
    r32_pb_clr: R32PbClr,
    r32_pb_pu: R32PbPu,
    r32_pb_pd: R32PbPd,
    r32_pb_drv: R32PbDrv,
    r32_pb_smt: R32PbSmt,
}
impl RegisterBlock {
    #[doc = "0x00 - safe accessing sign register"]
    #[inline(always)]
    pub const fn r8_safe_access_sig(&self) -> &R8SafeAccessSig {
        &self.r8_safe_access_sig
    }
    #[doc = "0x01 - chip ID register"]
    #[inline(always)]
    pub const fn r8_chip_id(&self) -> &R8ChipId {
        &self.r8_chip_id
    }
    #[doc = "0x02 - safe accessing ID register"]
    #[inline(always)]
    pub const fn r8_safe_access_id(&self) -> &R8SafeAccessId {
        &self.r8_safe_access_id
    }
    #[doc = "0x03 - watch-dog count register"]
    #[inline(always)]
    pub const fn r8_wdog_count(&self) -> &R8WdogCount {
        &self.r8_wdog_count
    }
    #[doc = "0x04 - flash ROM configuration register"]
    #[inline(always)]
    pub const fn r8_glob_rom_cfg(&self) -> &R8GlobRomCfg {
        &self.r8_glob_rom_cfg
    }
    #[doc = "0x05 - reset status and boot/debug status"]
    #[inline(always)]
    pub const fn r8_rst_boot_stat(&self) -> &R8RstBootStat {
        &self.r8_rst_boot_stat
    }
    #[doc = "0x06 - reset and watch-dog control"]
    #[inline(always)]
    pub const fn r8_rst_wdog_ctrl(&self) -> &R8RstWdogCtrl {
        &self.r8_rst_wdog_ctrl
    }
    #[doc = "0x07 - value keeper during global reset"]
    #[inline(always)]
    pub const fn r8_glob_reset_keep(&self) -> &R8GlobResetKeep {
        &self.r8_glob_reset_keep
    }
    #[doc = "0x08 - output clock divider from PLL"]
    #[inline(always)]
    pub const fn r8_clk_pll_div(&self) -> &R8ClkPllDiv {
        &self.r8_clk_pll_div
    }
    #[doc = "0x0a - clock control"]
    #[inline(always)]
    pub const fn r8_clk_cfg_ctrl(&self) -> &R8ClkCfgCtrl {
        &self.r8_clk_cfg_ctrl
    }
    #[doc = "0x0b - clock mode aux register"]
    #[inline(always)]
    pub const fn r8_clk_mod_aux(&self) -> &R8ClkModAux {
        &self.r8_clk_mod_aux
    }
    #[doc = "0x0c - sleep clock off control byte 0"]
    #[inline(always)]
    pub const fn r8_slp_clk_off0(&self) -> &R8SlpClkOff0 {
        &self.r8_slp_clk_off0
    }
    #[doc = "0x0d - sleep clock off control byte 1"]
    #[inline(always)]
    pub const fn r8_slp_clk_off1(&self) -> &R8SlpClkOff1 {
        &self.r8_slp_clk_off1
    }
    #[doc = "0x0e - wake control"]
    #[inline(always)]
    pub const fn r8_slp_wake_ctrl(&self) -> &R8SlpWakeCtrl {
        &self.r8_slp_wake_ctrl
    }
    #[doc = "0x0f - power control"]
    #[inline(always)]
    pub const fn r8_slp_power_ctrl(&self) -> &R8SlpPowerCtrl {
        &self.r8_slp_power_ctrl
    }
    #[doc = "0x10 - external bus configuration"]
    #[inline(always)]
    pub const fn r8_xbus_config(&self) -> &R8XbusConfig {
        &self.r8_xbus_config
    }
    #[doc = "0x12 - alternate pin control"]
    #[inline(always)]
    pub const fn r8_pin_alternate(&self) -> &R8PinAlternate {
        &self.r8_pin_alternate
    }
    #[doc = "0x1c - GPIO interrupt control"]
    #[inline(always)]
    pub const fn r8_gpio_int_flag(&self) -> &R8GpioIntFlag {
        &self.r8_gpio_int_flag
    }
    #[doc = "0x1d - GPIO interrupt enable"]
    #[inline(always)]
    pub const fn r8_gpio_int_enable(&self) -> &R8GpioIntEnable {
        &self.r8_gpio_int_enable
    }
    #[doc = "0x1e - GPIO interrupt mode"]
    #[inline(always)]
    pub const fn r8_gpio_int_mode(&self) -> &R8GpioIntMode {
        &self.r8_gpio_int_mode
    }
    #[doc = "0x1f - GPIO interrupt polarity"]
    #[inline(always)]
    pub const fn r8_gpio_int_polar(&self) -> &R8GpioIntPolar {
        &self.r8_gpio_int_polar
    }
    #[doc = "0x20 - Serdes Analog parameter configuration1"]
    #[inline(always)]
    pub const fn r16_serd_ana_cfg1(&self) -> &R16SerdAnaCfg1 {
        &self.r16_serd_ana_cfg1
    }
    #[doc = "0x24 - Serdes Analog parameter configuration2"]
    #[inline(always)]
    pub const fn r32_serd_ana_cfg2(&self) -> &R32SerdAnaCfg2 {
        &self.r32_serd_ana_cfg2
    }
    #[doc = "0x40 - GPIO PA I/O direction"]
    #[inline(always)]
    pub const fn r32_pa_dir(&self) -> &R32PaDir {
        &self.r32_pa_dir
    }
    #[doc = "0x44 - GPIO PA input"]
    #[inline(always)]
    pub const fn r32_pa_pin(&self) -> &R32PaPin {
        &self.r32_pa_pin
    }
    #[doc = "0x48 - GPIO PA output"]
    #[inline(always)]
    pub const fn r32_pa_out(&self) -> &R32PaOut {
        &self.r32_pa_out
    }
    #[doc = "0x4c - GPIO PA clear output"]
    #[inline(always)]
    pub const fn r32_pa_clr(&self) -> &R32PaClr {
        &self.r32_pa_clr
    }
    #[doc = "0x50 - GPIO PA pullup resistance enable"]
    #[inline(always)]
    pub const fn r32_pa_pu(&self) -> &R32PaPu {
        &self.r32_pa_pu
    }
    #[doc = "0x54 - GPIO PA output open-drain_input pulldown resistance enable"]
    #[inline(always)]
    pub const fn r32_pa_pd(&self) -> &R32PaPd {
        &self.r32_pa_pd
    }
    #[doc = "0x58 - GPIO PA driving capability"]
    #[inline(always)]
    pub const fn r32_pa_drv(&self) -> &R32PaDrv {
        &self.r32_pa_drv
    }
    #[doc = "0x5c - GPIO PA output slew rate_input schmitt trigger"]
    #[inline(always)]
    pub const fn r32_pa_smt(&self) -> &R32PaSmt {
        &self.r32_pa_smt
    }
    #[doc = "0x60 - GPIO PB I/O direction"]
    #[inline(always)]
    pub const fn r32_pb_dir(&self) -> &R32PbDir {
        &self.r32_pb_dir
    }
    #[doc = "0x64 - GPIO PB input"]
    #[inline(always)]
    pub const fn r32_pb_pin(&self) -> &R32PbPin {
        &self.r32_pb_pin
    }
    #[doc = "0x68 - GPIO PB output"]
    #[inline(always)]
    pub const fn r32_pb_out(&self) -> &R32PbOut {
        &self.r32_pb_out
    }
    #[doc = "0x6c - GPIO PB clear output"]
    #[inline(always)]
    pub const fn r32_pb_clr(&self) -> &R32PbClr {
        &self.r32_pb_clr
    }
    #[doc = "0x70 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub const fn r32_pb_pu(&self) -> &R32PbPu {
        &self.r32_pb_pu
    }
    #[doc = "0x74 - GPIO PB output open-drain_input pulldown resistance enable"]
    #[inline(always)]
    pub const fn r32_pb_pd(&self) -> &R32PbPd {
        &self.r32_pb_pd
    }
    #[doc = "0x78 - GPIO PB driving capability"]
    #[inline(always)]
    pub const fn r32_pb_drv(&self) -> &R32PbDrv {
        &self.r32_pb_drv
    }
    #[doc = "0x7c - GPIO PB output slew rate_input schmitt trigger"]
    #[inline(always)]
    pub const fn r32_pb_smt(&self) -> &R32PbSmt {
        &self.r32_pb_smt
    }
}
#[doc = "R8_SAFE_ACCESS_SIG (r) register accessor: safe accessing sign register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_sig::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_safe_access_sig`] module"]
#[doc(alias = "R8_SAFE_ACCESS_SIG")]
pub type R8SafeAccessSig = crate::Reg<r8_safe_access_sig::R8SafeAccessSigSpec>;
#[doc = "safe accessing sign register"]
pub mod r8_safe_access_sig;
#[doc = "R8_CHIP_ID (r) register accessor: chip ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_chip_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_chip_id`] module"]
#[doc(alias = "R8_CHIP_ID")]
pub type R8ChipId = crate::Reg<r8_chip_id::R8ChipIdSpec>;
#[doc = "chip ID register"]
pub mod r8_chip_id;
#[doc = "R8_SAFE_ACCESS_ID (r) register accessor: safe accessing ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_safe_access_id`] module"]
#[doc(alias = "R8_SAFE_ACCESS_ID")]
pub type R8SafeAccessId = crate::Reg<r8_safe_access_id::R8SafeAccessIdSpec>;
#[doc = "safe accessing ID register"]
pub mod r8_safe_access_id;
#[doc = "R8_WDOG_COUNT (rw) register accessor: watch-dog count register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_wdog_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_wdog_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_wdog_count`] module"]
#[doc(alias = "R8_WDOG_COUNT")]
pub type R8WdogCount = crate::Reg<r8_wdog_count::R8WdogCountSpec>;
#[doc = "watch-dog count register"]
pub mod r8_wdog_count;
#[doc = "R8_GLOB_ROM_CFG (rw) register accessor: flash ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_rom_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_glob_rom_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_glob_rom_cfg`] module"]
#[doc(alias = "R8_GLOB_ROM_CFG")]
pub type R8GlobRomCfg = crate::Reg<r8_glob_rom_cfg::R8GlobRomCfgSpec>;
#[doc = "flash ROM configuration register"]
pub mod r8_glob_rom_cfg;
#[doc = "R8_RST_BOOT_STAT (r) register accessor: reset status and boot/debug status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rst_boot_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_rst_boot_stat`] module"]
#[doc(alias = "R8_RST_BOOT_STAT")]
pub type R8RstBootStat = crate::Reg<r8_rst_boot_stat::R8RstBootStatSpec>;
#[doc = "reset status and boot/debug status"]
pub mod r8_rst_boot_stat;
#[doc = "R8_RST_WDOG_CTRL (rw) register accessor: reset and watch-dog control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rst_wdog_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rst_wdog_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_rst_wdog_ctrl`] module"]
#[doc(alias = "R8_RST_WDOG_CTRL")]
pub type R8RstWdogCtrl = crate::Reg<r8_rst_wdog_ctrl::R8RstWdogCtrlSpec>;
#[doc = "reset and watch-dog control"]
pub mod r8_rst_wdog_ctrl;
#[doc = "R8_GLOB_RESET_KEEP (rw) register accessor: value keeper during global reset\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_reset_keep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_glob_reset_keep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_glob_reset_keep`] module"]
#[doc(alias = "R8_GLOB_RESET_KEEP")]
pub type R8GlobResetKeep = crate::Reg<r8_glob_reset_keep::R8GlobResetKeepSpec>;
#[doc = "value keeper during global reset"]
pub mod r8_glob_reset_keep;
#[doc = "R8_CLK_PLL_DIV (rw) register accessor: output clock divider from PLL\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_clk_pll_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_clk_pll_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_clk_pll_div`] module"]
#[doc(alias = "R8_CLK_PLL_DIV")]
pub type R8ClkPllDiv = crate::Reg<r8_clk_pll_div::R8ClkPllDivSpec>;
#[doc = "output clock divider from PLL"]
pub mod r8_clk_pll_div;
#[doc = "R8_CLK_CFG_CTRL (rw) register accessor: clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_clk_cfg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_clk_cfg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_clk_cfg_ctrl`] module"]
#[doc(alias = "R8_CLK_CFG_CTRL")]
pub type R8ClkCfgCtrl = crate::Reg<r8_clk_cfg_ctrl::R8ClkCfgCtrlSpec>;
#[doc = "clock control"]
pub mod r8_clk_cfg_ctrl;
#[doc = "R8_CLK_MOD_AUX (rw) register accessor: clock mode aux register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_clk_mod_aux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_clk_mod_aux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_clk_mod_aux`] module"]
#[doc(alias = "R8_CLK_MOD_AUX")]
pub type R8ClkModAux = crate::Reg<r8_clk_mod_aux::R8ClkModAuxSpec>;
#[doc = "clock mode aux register"]
pub mod r8_clk_mod_aux;
#[doc = "R8_SLP_CLK_OFF0 (rw) register accessor: sleep clock off control byte 0\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_clk_off0`] module"]
#[doc(alias = "R8_SLP_CLK_OFF0")]
pub type R8SlpClkOff0 = crate::Reg<r8_slp_clk_off0::R8SlpClkOff0Spec>;
#[doc = "sleep clock off control byte 0"]
pub mod r8_slp_clk_off0;
#[doc = "R8_SLP_CLK_OFF1 (rw) register accessor: sleep clock off control byte 1\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_clk_off1`] module"]
#[doc(alias = "R8_SLP_CLK_OFF1")]
pub type R8SlpClkOff1 = crate::Reg<r8_slp_clk_off1::R8SlpClkOff1Spec>;
#[doc = "sleep clock off control byte 1"]
pub mod r8_slp_clk_off1;
#[doc = "R8_SLP_WAKE_CTRL (rw) register accessor: wake control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_wake_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_wake_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_wake_ctrl`] module"]
#[doc(alias = "R8_SLP_WAKE_CTRL")]
pub type R8SlpWakeCtrl = crate::Reg<r8_slp_wake_ctrl::R8SlpWakeCtrlSpec>;
#[doc = "wake control"]
pub mod r8_slp_wake_ctrl;
#[doc = "R8_SLP_POWER_CTRL (rw) register accessor: power control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_slp_power_ctrl`] module"]
#[doc(alias = "R8_SLP_POWER_CTRL")]
pub type R8SlpPowerCtrl = crate::Reg<r8_slp_power_ctrl::R8SlpPowerCtrlSpec>;
#[doc = "power control"]
pub mod r8_slp_power_ctrl;
#[doc = "R8_XBUS_CONFIG (rw) register accessor: external bus configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_xbus_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_xbus_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_xbus_config`] module"]
#[doc(alias = "R8_XBUS_CONFIG")]
pub type R8XbusConfig = crate::Reg<r8_xbus_config::R8XbusConfigSpec>;
#[doc = "external bus configuration"]
pub mod r8_xbus_config;
#[doc = "R16_SERD_ANA_CFG1 (rw) register accessor: Serdes Analog parameter configuration1\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_serd_ana_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_serd_ana_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_serd_ana_cfg1`] module"]
#[doc(alias = "R16_SERD_ANA_CFG1")]
pub type R16SerdAnaCfg1 = crate::Reg<r16_serd_ana_cfg1::R16SerdAnaCfg1Spec>;
#[doc = "Serdes Analog parameter configuration1"]
pub mod r16_serd_ana_cfg1;
#[doc = "R32_SERD_ANA_CFG2 (rw) register accessor: Serdes Analog parameter configuration2\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_serd_ana_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_serd_ana_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_serd_ana_cfg2`] module"]
#[doc(alias = "R32_SERD_ANA_CFG2")]
pub type R32SerdAnaCfg2 = crate::Reg<r32_serd_ana_cfg2::R32SerdAnaCfg2Spec>;
#[doc = "Serdes Analog parameter configuration2"]
pub mod r32_serd_ana_cfg2;
#[doc = "R8_GPIO_INT_FLAG (rw) register accessor: GPIO interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_gpio_int_flag`] module"]
#[doc(alias = "R8_GPIO_INT_FLAG")]
pub type R8GpioIntFlag = crate::Reg<r8_gpio_int_flag::R8GpioIntFlagSpec>;
#[doc = "GPIO interrupt control"]
pub mod r8_gpio_int_flag;
#[doc = "R8_GPIO_INT_ENABLE (rw) register accessor: GPIO interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_gpio_int_enable`] module"]
#[doc(alias = "R8_GPIO_INT_ENABLE")]
pub type R8GpioIntEnable = crate::Reg<r8_gpio_int_enable::R8GpioIntEnableSpec>;
#[doc = "GPIO interrupt enable"]
pub mod r8_gpio_int_enable;
#[doc = "R8_GPIO_INT_MODE (rw) register accessor: GPIO interrupt mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_gpio_int_mode`] module"]
#[doc(alias = "R8_GPIO_INT_MODE")]
pub type R8GpioIntMode = crate::Reg<r8_gpio_int_mode::R8GpioIntModeSpec>;
#[doc = "GPIO interrupt mode"]
pub mod r8_gpio_int_mode;
#[doc = "R8_GPIO_INT_POLAR (rw) register accessor: GPIO interrupt polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_polar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_polar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_gpio_int_polar`] module"]
#[doc(alias = "R8_GPIO_INT_POLAR")]
pub type R8GpioIntPolar = crate::Reg<r8_gpio_int_polar::R8GpioIntPolarSpec>;
#[doc = "GPIO interrupt polarity"]
pub mod r8_gpio_int_polar;
#[doc = "R32_PA_DIR (rw) register accessor: GPIO PA I/O direction\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_dir`] module"]
#[doc(alias = "R32_PA_DIR")]
pub type R32PaDir = crate::Reg<r32_pa_dir::R32PaDirSpec>;
#[doc = "GPIO PA I/O direction"]
pub mod r32_pa_dir;
#[doc = "R32_PA_PIN (r) register accessor: GPIO PA input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_pin`] module"]
#[doc(alias = "R32_PA_PIN")]
pub type R32PaPin = crate::Reg<r32_pa_pin::R32PaPinSpec>;
#[doc = "GPIO PA input"]
pub mod r32_pa_pin;
#[doc = "R32_PA_OUT (rw) register accessor: GPIO PA output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_out`] module"]
#[doc(alias = "R32_PA_OUT")]
pub type R32PaOut = crate::Reg<r32_pa_out::R32PaOutSpec>;
#[doc = "GPIO PA output"]
pub mod r32_pa_out;
#[doc = "R32_PA_CLR (w) register accessor: GPIO PA clear output\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_clr`] module"]
#[doc(alias = "R32_PA_CLR")]
pub type R32PaClr = crate::Reg<r32_pa_clr::R32PaClrSpec>;
#[doc = "GPIO PA clear output"]
pub mod r32_pa_clr;
#[doc = "R32_PA_PU (rw) register accessor: GPIO PA pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_pu`] module"]
#[doc(alias = "R32_PA_PU")]
pub type R32PaPu = crate::Reg<r32_pa_pu::R32PaPuSpec>;
#[doc = "GPIO PA pullup resistance enable"]
pub mod r32_pa_pu;
#[doc = "R32_PA_PD (rw) register accessor: GPIO PA output open-drain_input pulldown resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_pd`] module"]
#[doc(alias = "R32_PA_PD")]
pub type R32PaPd = crate::Reg<r32_pa_pd::R32PaPdSpec>;
#[doc = "GPIO PA output open-drain_input pulldown resistance enable"]
pub mod r32_pa_pd;
#[doc = "R32_PA_DRV (rw) register accessor: GPIO PA driving capability\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_drv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_drv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_drv`] module"]
#[doc(alias = "R32_PA_DRV")]
pub type R32PaDrv = crate::Reg<r32_pa_drv::R32PaDrvSpec>;
#[doc = "GPIO PA driving capability"]
pub mod r32_pa_drv;
#[doc = "R32_PA_SMT (rw) register accessor: GPIO PA output slew rate_input schmitt trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_smt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_smt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pa_smt`] module"]
#[doc(alias = "R32_PA_SMT")]
pub type R32PaSmt = crate::Reg<r32_pa_smt::R32PaSmtSpec>;
#[doc = "GPIO PA output slew rate_input schmitt trigger"]
pub mod r32_pa_smt;
#[doc = "R32_PB_DIR (rw) register accessor: GPIO PB I/O direction\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_dir`] module"]
#[doc(alias = "R32_PB_DIR")]
pub type R32PbDir = crate::Reg<r32_pb_dir::R32PbDirSpec>;
#[doc = "GPIO PB I/O direction"]
pub mod r32_pb_dir;
#[doc = "R32_PB_PIN (r) register accessor: GPIO PB input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_pin`] module"]
#[doc(alias = "R32_PB_PIN")]
pub type R32PbPin = crate::Reg<r32_pb_pin::R32PbPinSpec>;
#[doc = "GPIO PB input"]
pub mod r32_pb_pin;
#[doc = "R32_PB_OUT (rw) register accessor: GPIO PB output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_out`] module"]
#[doc(alias = "R32_PB_OUT")]
pub type R32PbOut = crate::Reg<r32_pb_out::R32PbOutSpec>;
#[doc = "GPIO PB output"]
pub mod r32_pb_out;
#[doc = "R32_PB_CLR (w) register accessor: GPIO PB clear output\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_clr`] module"]
#[doc(alias = "R32_PB_CLR")]
pub type R32PbClr = crate::Reg<r32_pb_clr::R32PbClrSpec>;
#[doc = "GPIO PB clear output"]
pub mod r32_pb_clr;
#[doc = "R32_PB_PU (rw) register accessor: GPIO PB pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_pu`] module"]
#[doc(alias = "R32_PB_PU")]
pub type R32PbPu = crate::Reg<r32_pb_pu::R32PbPuSpec>;
#[doc = "GPIO PB pullup resistance enable"]
pub mod r32_pb_pu;
#[doc = "R32_PB_PD (rw) register accessor: GPIO PB output open-drain_input pulldown resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_pd`] module"]
#[doc(alias = "R32_PB_PD")]
pub type R32PbPd = crate::Reg<r32_pb_pd::R32PbPdSpec>;
#[doc = "GPIO PB output open-drain_input pulldown resistance enable"]
pub mod r32_pb_pd;
#[doc = "R32_PB_DRV (rw) register accessor: GPIO PB driving capability\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_drv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_drv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_drv`] module"]
#[doc(alias = "R32_PB_DRV")]
pub type R32PbDrv = crate::Reg<r32_pb_drv::R32PbDrvSpec>;
#[doc = "GPIO PB driving capability"]
pub mod r32_pb_drv;
#[doc = "R32_PB_SMT (rw) register accessor: GPIO PB output slew rate_input schmitt trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_smt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_smt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pb_smt`] module"]
#[doc(alias = "R32_PB_SMT")]
pub type R32PbSmt = crate::Reg<r32_pb_smt::R32PbSmtSpec>;
#[doc = "GPIO PB output slew rate_input schmitt trigger"]
pub mod r32_pb_smt;
#[doc = "R8_PIN_ALTERNATE (rw) register accessor: alternate pin control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pin_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pin_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pin_alternate`] module"]
#[doc(alias = "R8_PIN_ALTERNATE")]
pub type R8PinAlternate = crate::Reg<r8_pin_alternate::R8PinAlternateSpec>;
#[doc = "alternate pin control"]
pub mod r8_pin_alternate;
