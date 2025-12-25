#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r8_pwm_ctrl_mod: R8PwmCtrlMod,
    r8_pwm_ctrl_cfg: R8PwmCtrlCfg,
    r8_pwm_clock_div: R8PwmClockDiv,
    _reserved3: [u8; 0x01],
    r32_pwm_data: R32PwmData,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM mode control"]
    #[inline(always)]
    pub const fn r8_pwm_ctrl_mod(&self) -> &R8PwmCtrlMod {
        &self.r8_pwm_ctrl_mod
    }
    #[doc = "0x01 - PWM configuration control"]
    #[inline(always)]
    pub const fn r8_pwm_ctrl_cfg(&self) -> &R8PwmCtrlCfg {
        &self.r8_pwm_ctrl_cfg
    }
    #[doc = "0x02 - PWM clock divisor"]
    #[inline(always)]
    pub const fn r8_pwm_clock_div(&self) -> &R8PwmClockDiv {
        &self.r8_pwm_clock_div
    }
    #[doc = "0x04 - PWM data holding"]
    #[inline(always)]
    pub const fn r32_pwm_data(&self) -> &R32PwmData {
        &self.r32_pwm_data
    }
}
#[doc = "R8_PWM_CTRL_MOD (rw) register accessor: PWM mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_ctrl_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_ctrl_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_ctrl_mod`] module"]
#[doc(alias = "R8_PWM_CTRL_MOD")]
pub type R8PwmCtrlMod = crate::Reg<r8_pwm_ctrl_mod::R8PwmCtrlModSpec>;
#[doc = "PWM mode control"]
pub mod r8_pwm_ctrl_mod;
#[doc = "R8_PWM_CTRL_CFG (rw) register accessor: PWM configuration control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_ctrl_cfg`] module"]
#[doc(alias = "R8_PWM_CTRL_CFG")]
pub type R8PwmCtrlCfg = crate::Reg<r8_pwm_ctrl_cfg::R8PwmCtrlCfgSpec>;
#[doc = "PWM configuration control"]
pub mod r8_pwm_ctrl_cfg;
#[doc = "R8_PWM_CLOCK_DIV (rw) register accessor: PWM clock divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_clock_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_clock_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_pwm_clock_div`] module"]
#[doc(alias = "R8_PWM_CLOCK_DIV")]
pub type R8PwmClockDiv = crate::Reg<r8_pwm_clock_div::R8PwmClockDivSpec>;
#[doc = "PWM clock divisor"]
pub mod r8_pwm_clock_div;
#[doc = "R32_PWM_DATA (rw) register accessor: PWM data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pwm_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pwm_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_pwm_data`] module"]
#[doc(alias = "R32_PWM_DATA")]
pub type R32PwmData = crate::Reg<r32_pwm_data::R32PwmDataSpec>;
#[doc = "PWM data holding"]
pub mod r32_pwm_data;
