#[doc = "Register `R8_PWM_CTRL_CFG` reader"]
pub type R = crate::R<R8PwmCtrlCfgSpec>;
#[doc = "Register `R8_PWM_CTRL_CFG` writer"]
pub type W = crate::W<R8PwmCtrlCfgSpec>;
#[doc = "Field `RB_PWM_CYCLE_SEL` reader - PWM cycle selection"]
pub type RbPwmCycleSelR = crate::BitReader;
#[doc = "Field `RB_PWM_CYCLE_SEL` writer - PWM cycle selection"]
pub type RbPwmCycleSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM cycle selection"]
    #[inline(always)]
    pub fn rb_pwm_cycle_sel(&self) -> RbPwmCycleSelR {
        RbPwmCycleSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM cycle selection"]
    #[inline(always)]
    pub fn rb_pwm_cycle_sel(&mut self) -> RbPwmCycleSelW<'_, R8PwmCtrlCfgSpec> {
        RbPwmCycleSelW::new(self, 0)
    }
}
#[doc = "PWM configuration control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_ctrl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_ctrl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmCtrlCfgSpec;
impl crate::RegisterSpec for R8PwmCtrlCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_ctrl_cfg::R`](R) reader structure"]
impl crate::Readable for R8PwmCtrlCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_ctrl_cfg::W`](W) writer structure"]
impl crate::Writable for R8PwmCtrlCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_CTRL_CFG to value 0"]
impl crate::Resettable for R8PwmCtrlCfgSpec {}
