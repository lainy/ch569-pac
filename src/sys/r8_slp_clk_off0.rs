#[doc = "Register `R8_SLP_CLK_OFF0` reader"]
pub type R = crate::R<R8SlpClkOff0Spec>;
#[doc = "Register `R8_SLP_CLK_OFF0` writer"]
pub type W = crate::W<R8SlpClkOff0Spec>;
#[doc = "Field `RB_SLP_CLK_TMR0` reader - sleep TMR0 clock"]
pub type RbSlpClkTmr0R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_TMR0` writer - sleep TMR0 clock"]
pub type RbSlpClkTmr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_TMR1` reader - sleep TMR1 clock"]
pub type RbSlpClkTmr1R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_TMR1` writer - sleep TMR1 clock"]
pub type RbSlpClkTmr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_TMR2` reader - sleep TMR2 clock"]
pub type RbSlpClkTmr2R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_TMR2` writer - sleep TMR2 clock"]
pub type RbSlpClkTmr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_PWMX` reader - sleep PWMX clock"]
pub type RbSlpClkPwmxR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_PWMX` writer - sleep PWMX clock"]
pub type RbSlpClkPwmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_UART0` reader - sleep UART0 clock"]
pub type RbSlpClkUart0R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_UART0` writer - sleep UART0 clock"]
pub type RbSlpClkUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_UART1` reader - sleep UART1 clock"]
pub type RbSlpClkUart1R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_UART1` writer - sleep UART1 clock"]
pub type RbSlpClkUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_UART2` reader - sleep UART2 clock"]
pub type RbSlpClkUart2R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_UART2` writer - sleep UART2 clock"]
pub type RbSlpClkUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_UART3` reader - sleep UART3 clock"]
pub type RbSlpClkUart3R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_UART3` writer - sleep UART3 clock"]
pub type RbSlpClkUart3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sleep TMR0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr0(&self) -> RbSlpClkTmr0R {
        RbSlpClkTmr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep TMR1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr1(&self) -> RbSlpClkTmr1R {
        RbSlpClkTmr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep TMR2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr2(&self) -> RbSlpClkTmr2R {
        RbSlpClkTmr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep PWMX clock"]
    #[inline(always)]
    pub fn rb_slp_clk_pwmx(&self) -> RbSlpClkPwmxR {
        RbSlpClkPwmxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sleep UART0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart0(&self) -> RbSlpClkUart0R {
        RbSlpClkUart0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sleep UART1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart1(&self) -> RbSlpClkUart1R {
        RbSlpClkUart1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - sleep UART2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart2(&self) -> RbSlpClkUart2R {
        RbSlpClkUart2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - sleep UART3 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart3(&self) -> RbSlpClkUart3R {
        RbSlpClkUart3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sleep TMR0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr0(&mut self) -> RbSlpClkTmr0W<'_, R8SlpClkOff0Spec> {
        RbSlpClkTmr0W::new(self, 0)
    }
    #[doc = "Bit 1 - sleep TMR1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr1(&mut self) -> RbSlpClkTmr1W<'_, R8SlpClkOff0Spec> {
        RbSlpClkTmr1W::new(self, 1)
    }
    #[doc = "Bit 2 - sleep TMR2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_tmr2(&mut self) -> RbSlpClkTmr2W<'_, R8SlpClkOff0Spec> {
        RbSlpClkTmr2W::new(self, 2)
    }
    #[doc = "Bit 3 - sleep PWMX clock"]
    #[inline(always)]
    pub fn rb_slp_clk_pwmx(&mut self) -> RbSlpClkPwmxW<'_, R8SlpClkOff0Spec> {
        RbSlpClkPwmxW::new(self, 3)
    }
    #[doc = "Bit 4 - sleep UART0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart0(&mut self) -> RbSlpClkUart0W<'_, R8SlpClkOff0Spec> {
        RbSlpClkUart0W::new(self, 4)
    }
    #[doc = "Bit 5 - sleep UART1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart1(&mut self) -> RbSlpClkUart1W<'_, R8SlpClkOff0Spec> {
        RbSlpClkUart1W::new(self, 5)
    }
    #[doc = "Bit 6 - sleep UART2 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart2(&mut self) -> RbSlpClkUart2W<'_, R8SlpClkOff0Spec> {
        RbSlpClkUart2W::new(self, 6)
    }
    #[doc = "Bit 7 - sleep UART3 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_uart3(&mut self) -> RbSlpClkUart3W<'_, R8SlpClkOff0Spec> {
        RbSlpClkUart3W::new(self, 7)
    }
}
#[doc = "sleep clock off control byte 0\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpClkOff0Spec;
impl crate::RegisterSpec for R8SlpClkOff0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_clk_off0::R`](R) reader structure"]
impl crate::Readable for R8SlpClkOff0Spec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_clk_off0::W`](W) writer structure"]
impl crate::Writable for R8SlpClkOff0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_CLK_OFF0 to value 0"]
impl crate::Resettable for R8SlpClkOff0Spec {}
