#[doc = "Register `R8_CLK_CFG_CTRL` reader"]
pub type R = crate::R<R8ClkCfgCtrlSpec>;
#[doc = "Register `R8_CLK_CFG_CTRL` writer"]
pub type W = crate::W<R8ClkCfgCtrlSpec>;
#[doc = "Field `RB_CLK_PLL_SLEEP` reader - PLL sleep control"]
pub type RbClkPllSleepR = crate::BitReader;
#[doc = "Field `RB_CLK_PLL_SLEEP` writer - PLL sleep control"]
pub type RbClkPllSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_CLK_SEL_PLL` reader - clock source selection"]
pub type RbClkSelPllR = crate::BitReader;
#[doc = "Field `RB_CLK_SEL_PLL` writer - clock source selection"]
pub type RbClkSelPllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL sleep control"]
    #[inline(always)]
    pub fn rb_clk_pll_sleep(&self) -> RbClkPllSleepR {
        RbClkPllSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock source selection"]
    #[inline(always)]
    pub fn rb_clk_sel_pll(&self) -> RbClkSelPllR {
        RbClkSelPllR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL sleep control"]
    #[inline(always)]
    pub fn rb_clk_pll_sleep(&mut self) -> RbClkPllSleepW<'_, R8ClkCfgCtrlSpec> {
        RbClkPllSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - clock source selection"]
    #[inline(always)]
    pub fn rb_clk_sel_pll(&mut self) -> RbClkSelPllW<'_, R8ClkCfgCtrlSpec> {
        RbClkSelPllW::new(self, 1)
    }
}
#[doc = "clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_clk_cfg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_clk_cfg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8ClkCfgCtrlSpec;
impl crate::RegisterSpec for R8ClkCfgCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_clk_cfg_ctrl::R`](R) reader structure"]
impl crate::Readable for R8ClkCfgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_clk_cfg_ctrl::W`](W) writer structure"]
impl crate::Writable for R8ClkCfgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_CLK_CFG_CTRL to value 0x80"]
impl crate::Resettable for R8ClkCfgCtrlSpec {
    const RESET_VALUE: u8 = 0x80;
}
