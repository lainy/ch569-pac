#[doc = "Register `R8_TMR1_CTRL_MOD` reader"]
pub type R = crate::R<R8Tmr1CtrlModSpec>;
#[doc = "Register `R8_TMR1_CTRL_MOD` writer"]
pub type W = crate::W<R8Tmr1CtrlModSpec>;
#[doc = "Field `RB_TMR_MODE_IN` reader - timer in mode"]
pub type RbTmrModeInR = crate::BitReader;
#[doc = "Field `RB_TMR_MODE_IN` writer - timer in mode"]
pub type RbTmrModeInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_ALL_CLEAR` reader - force clear timer FIFO and count"]
pub type RbTmrAllClearR = crate::BitReader;
#[doc = "Field `RB_TMR_ALL_CLEAR` writer - force clear timer FIFO and count"]
pub type RbTmrAllClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_COUNT_EN` reader - timer count enable"]
pub type RbTmrCountEnR = crate::BitReader;
#[doc = "Field `RB_TMR_COUNT_EN` writer - timer count enable"]
pub type RbTmrCountEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_OUT_EN` reader - timer output enable"]
pub type RbTmrOutEnR = crate::BitReader;
#[doc = "Field `RB_TMR_OUT_EN` writer - timer output enable"]
pub type RbTmrOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT` reader - timer PWM output polarity / Count sub-mode"]
pub type RbTmrOutPolarRbTmrCapCountR = crate::BitReader;
#[doc = "Field `RB_TMR_OUT_POLAR_RB_TMR_CAP_COUNT` writer - timer PWM output polarity / Count sub-mode"]
pub type RbTmrOutPolarRbTmrCapCountW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE` reader - timer PWM repeat mode / timer capture edge mode"]
pub type RbTmrPwmRepeatRbTmrCapEdgeR = crate::FieldReader;
#[doc = "Field `RB_TMR_PWM_REPEAT_RB_TMR_CAP_EDGE` writer - timer PWM repeat mode / timer capture edge mode"]
pub type RbTmrPwmRepeatRbTmrCapEdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - timer in mode"]
    #[inline(always)]
    pub fn rb_tmr_mode_in(&self) -> RbTmrModeInR {
        RbTmrModeInR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear timer FIFO and count"]
    #[inline(always)]
    pub fn rb_tmr_all_clear(&self) -> RbTmrAllClearR {
        RbTmrAllClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - timer count enable"]
    #[inline(always)]
    pub fn rb_tmr_count_en(&self) -> RbTmrCountEnR {
        RbTmrCountEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - timer output enable"]
    #[inline(always)]
    pub fn rb_tmr_out_en(&self) -> RbTmrOutEnR {
        RbTmrOutEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - timer PWM output polarity / Count sub-mode"]
    #[inline(always)]
    pub fn rb_tmr_out_polar_rb_tmr_cap_count(&self) -> RbTmrOutPolarRbTmrCapCountR {
        RbTmrOutPolarRbTmrCapCountR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - timer PWM repeat mode / timer capture edge mode"]
    #[inline(always)]
    pub fn rb_tmr_pwm_repeat_rb_tmr_cap_edge(&self) -> RbTmrPwmRepeatRbTmrCapEdgeR {
        RbTmrPwmRepeatRbTmrCapEdgeR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - timer in mode"]
    #[inline(always)]
    pub fn rb_tmr_mode_in(&mut self) -> RbTmrModeInW<'_, R8Tmr1CtrlModSpec> {
        RbTmrModeInW::new(self, 0)
    }
    #[doc = "Bit 1 - force clear timer FIFO and count"]
    #[inline(always)]
    pub fn rb_tmr_all_clear(&mut self) -> RbTmrAllClearW<'_, R8Tmr1CtrlModSpec> {
        RbTmrAllClearW::new(self, 1)
    }
    #[doc = "Bit 2 - timer count enable"]
    #[inline(always)]
    pub fn rb_tmr_count_en(&mut self) -> RbTmrCountEnW<'_, R8Tmr1CtrlModSpec> {
        RbTmrCountEnW::new(self, 2)
    }
    #[doc = "Bit 3 - timer output enable"]
    #[inline(always)]
    pub fn rb_tmr_out_en(&mut self) -> RbTmrOutEnW<'_, R8Tmr1CtrlModSpec> {
        RbTmrOutEnW::new(self, 3)
    }
    #[doc = "Bit 4 - timer PWM output polarity / Count sub-mode"]
    #[inline(always)]
    pub fn rb_tmr_out_polar_rb_tmr_cap_count(
        &mut self,
    ) -> RbTmrOutPolarRbTmrCapCountW<'_, R8Tmr1CtrlModSpec> {
        RbTmrOutPolarRbTmrCapCountW::new(self, 4)
    }
    #[doc = "Bits 6:7 - timer PWM repeat mode / timer capture edge mode"]
    #[inline(always)]
    pub fn rb_tmr_pwm_repeat_rb_tmr_cap_edge(
        &mut self,
    ) -> RbTmrPwmRepeatRbTmrCapEdgeW<'_, R8Tmr1CtrlModSpec> {
        RbTmrPwmRepeatRbTmrCapEdgeW::new(self, 6)
    }
}
#[doc = "TMR1 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_ctrl_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr1_ctrl_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr1CtrlModSpec;
impl crate::RegisterSpec for R8Tmr1CtrlModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr1_ctrl_mod::R`](R) reader structure"]
impl crate::Readable for R8Tmr1CtrlModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tmr1_ctrl_mod::W`](W) writer structure"]
impl crate::Writable for R8Tmr1CtrlModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TMR1_CTRL_MOD to value 0x02"]
impl crate::Resettable for R8Tmr1CtrlModSpec {
    const RESET_VALUE: u8 = 0x02;
}
