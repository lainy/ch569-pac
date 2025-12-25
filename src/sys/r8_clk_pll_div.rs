#[doc = "Register `R8_CLK_PLL_DIV` reader"]
pub type R = crate::R<R8ClkPllDivSpec>;
#[doc = "Register `R8_CLK_PLL_DIV` writer"]
pub type W = crate::W<R8ClkPllDivSpec>;
#[doc = "Field `R8_CLK_PLL_DIV` reader - output clock divider from PLL"]
pub type R8ClkPllDivR = crate::FieldReader;
#[doc = "Field `R8_CLK_PLL_DIV` writer - output clock divider from PLL"]
pub type R8ClkPllDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - output clock divider from PLL"]
    #[inline(always)]
    pub fn r8_clk_pll_div(&self) -> R8ClkPllDivR {
        R8ClkPllDivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - output clock divider from PLL"]
    #[inline(always)]
    pub fn r8_clk_pll_div(&mut self) -> R8ClkPllDivW<'_, R8ClkPllDivSpec> {
        R8ClkPllDivW::new(self, 0)
    }
}
#[doc = "output clock divider from PLL\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_clk_pll_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_clk_pll_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8ClkPllDivSpec;
impl crate::RegisterSpec for R8ClkPllDivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_clk_pll_div::R`](R) reader structure"]
impl crate::Readable for R8ClkPllDivSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_clk_pll_div::W`](W) writer structure"]
impl crate::Writable for R8ClkPllDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_CLK_PLL_DIV to value 0x42"]
impl crate::Resettable for R8ClkPllDivSpec {
    const RESET_VALUE: u8 = 0x42;
}
