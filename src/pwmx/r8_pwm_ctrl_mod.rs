#[doc = "Register `R8_PWM_CTRL_MOD` reader"]
pub type R = crate::R<R8PwmCtrlModSpec>;
#[doc = "Register `R8_PWM_CTRL_MOD` writer"]
pub type W = crate::W<R8PwmCtrlModSpec>;
#[doc = "Field `RB_PWM0_OUT_EN` reader - PWM0 output enable"]
pub type RbPwm0OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM0_OUT_EN` writer - PWM0 output enable"]
pub type RbPwm0OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM1_OUT_EN` reader - PWM1 output enable"]
pub type RbPwm1OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM1_OUT_EN` writer - PWM1 output enable"]
pub type RbPwm1OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM2_OUT_EN` reader - PWM2 output enable"]
pub type RbPwm2OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM2_OUT_EN` writer - PWM2 output enable"]
pub type RbPwm2OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM3_OUT_EN` reader - PWM3 output enable"]
pub type RbPwm3OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM3_OUT_EN` writer - PWM3 output enable"]
pub type RbPwm3OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM0_POLAR` reader - PWM0 output polarity"]
pub type RbPwm0PolarR = crate::BitReader;
#[doc = "Field `RB_PWM0_POLAR` writer - PWM0 output polarity"]
pub type RbPwm0PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM1_POLAR` reader - PWM1 output polarity"]
pub type RbPwm1PolarR = crate::BitReader;
#[doc = "Field `RB_PWM1_POLAR` writer - PWM1 output polarity"]
pub type RbPwm1PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM2_POLAR` reader - PWM2 output polarity"]
pub type RbPwm2PolarR = crate::BitReader;
#[doc = "Field `RB_PWM2_POLAR` writer - PWM2 output polarity"]
pub type RbPwm2PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM3_POLAR` reader - PWM3 output polarity"]
pub type RbPwm3PolarR = crate::BitReader;
#[doc = "Field `RB_PWM3_POLAR` writer - PWM3 output polarity"]
pub type RbPwm3PolarW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM0 output enable"]
    #[inline(always)]
    pub fn rb_pwm0_out_en(&self) -> RbPwm0OutEnR {
        RbPwm0OutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 output enable"]
    #[inline(always)]
    pub fn rb_pwm1_out_en(&self) -> RbPwm1OutEnR {
        RbPwm1OutEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 output enable"]
    #[inline(always)]
    pub fn rb_pwm2_out_en(&self) -> RbPwm2OutEnR {
        RbPwm2OutEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 output enable"]
    #[inline(always)]
    pub fn rb_pwm3_out_en(&self) -> RbPwm3OutEnR {
        RbPwm3OutEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM0 output polarity"]
    #[inline(always)]
    pub fn rb_pwm0_polar(&self) -> RbPwm0PolarR {
        RbPwm0PolarR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM1 output polarity"]
    #[inline(always)]
    pub fn rb_pwm1_polar(&self) -> RbPwm1PolarR {
        RbPwm1PolarR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM2 output polarity"]
    #[inline(always)]
    pub fn rb_pwm2_polar(&self) -> RbPwm2PolarR {
        RbPwm2PolarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM3 output polarity"]
    #[inline(always)]
    pub fn rb_pwm3_polar(&self) -> RbPwm3PolarR {
        RbPwm3PolarR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 output enable"]
    #[inline(always)]
    pub fn rb_pwm0_out_en(&mut self) -> RbPwm0OutEnW<'_, R8PwmCtrlModSpec> {
        RbPwm0OutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - PWM1 output enable"]
    #[inline(always)]
    pub fn rb_pwm1_out_en(&mut self) -> RbPwm1OutEnW<'_, R8PwmCtrlModSpec> {
        RbPwm1OutEnW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM2 output enable"]
    #[inline(always)]
    pub fn rb_pwm2_out_en(&mut self) -> RbPwm2OutEnW<'_, R8PwmCtrlModSpec> {
        RbPwm2OutEnW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM3 output enable"]
    #[inline(always)]
    pub fn rb_pwm3_out_en(&mut self) -> RbPwm3OutEnW<'_, R8PwmCtrlModSpec> {
        RbPwm3OutEnW::new(self, 3)
    }
    #[doc = "Bit 4 - PWM0 output polarity"]
    #[inline(always)]
    pub fn rb_pwm0_polar(&mut self) -> RbPwm0PolarW<'_, R8PwmCtrlModSpec> {
        RbPwm0PolarW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM1 output polarity"]
    #[inline(always)]
    pub fn rb_pwm1_polar(&mut self) -> RbPwm1PolarW<'_, R8PwmCtrlModSpec> {
        RbPwm1PolarW::new(self, 5)
    }
    #[doc = "Bit 6 - PWM2 output polarity"]
    #[inline(always)]
    pub fn rb_pwm2_polar(&mut self) -> RbPwm2PolarW<'_, R8PwmCtrlModSpec> {
        RbPwm2PolarW::new(self, 6)
    }
    #[doc = "Bit 7 - PWM3 output polarity"]
    #[inline(always)]
    pub fn rb_pwm3_polar(&mut self) -> RbPwm3PolarW<'_, R8PwmCtrlModSpec> {
        RbPwm3PolarW::new(self, 7)
    }
}
#[doc = "PWM mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_ctrl_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_ctrl_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmCtrlModSpec;
impl crate::RegisterSpec for R8PwmCtrlModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_ctrl_mod::R`](R) reader structure"]
impl crate::Readable for R8PwmCtrlModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_ctrl_mod::W`](W) writer structure"]
impl crate::Writable for R8PwmCtrlModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_CTRL_MOD to value 0"]
impl crate::Resettable for R8PwmCtrlModSpec {}
