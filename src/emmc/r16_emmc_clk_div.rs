#[doc = "Register `R16_EMMC_CLK_DIV` reader"]
pub type R = crate::R<R16EmmcClkDivSpec>;
#[doc = "Register `R16_EMMC_CLK_DIV` writer"]
pub type W = crate::W<R16EmmcClkDivSpec>;
#[doc = "Field `RB_EMMC_DIV_MASK` reader - clk div"]
pub type RbEmmcDivMaskR = crate::FieldReader;
#[doc = "Field `RB_EMMC_DIV_MASK` writer - clk div"]
pub type RbEmmcDivMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RB_EMMC_CLKOE` reader - chip output sdclk oe"]
pub type RbEmmcClkoeR = crate::BitReader;
#[doc = "Field `RB_EMMC_CLKOE` writer - chip output sdclk oe"]
pub type RbEmmcClkoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_CLKMode` reader - EMMC clock frequency mode selection bit"]
pub type RbEmmcClkmodeR = crate::BitReader;
#[doc = "Field `RB_EMMC_CLKMode` writer - EMMC clock frequency mode selection bit"]
pub type RbEmmcClkmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_PHASEINV` reader - invert chip output sdclk phase"]
pub type RbEmmcPhaseinvR = crate::BitReader;
#[doc = "Field `RB_EMMC_PHASEINV` writer - invert chip output sdclk phase"]
pub type RbEmmcPhaseinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - clk div"]
    #[inline(always)]
    pub fn rb_emmc_div_mask(&self) -> RbEmmcDivMaskR {
        RbEmmcDivMaskR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - chip output sdclk oe"]
    #[inline(always)]
    pub fn rb_emmc_clkoe(&self) -> RbEmmcClkoeR {
        RbEmmcClkoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EMMC clock frequency mode selection bit"]
    #[inline(always)]
    pub fn rb_emmc_clkmode(&self) -> RbEmmcClkmodeR {
        RbEmmcClkmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - invert chip output sdclk phase"]
    #[inline(always)]
    pub fn rb_emmc_phaseinv(&self) -> RbEmmcPhaseinvR {
        RbEmmcPhaseinvR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk div"]
    #[inline(always)]
    pub fn rb_emmc_div_mask(&mut self) -> RbEmmcDivMaskW<'_, R16EmmcClkDivSpec> {
        RbEmmcDivMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - chip output sdclk oe"]
    #[inline(always)]
    pub fn rb_emmc_clkoe(&mut self) -> RbEmmcClkoeW<'_, R16EmmcClkDivSpec> {
        RbEmmcClkoeW::new(self, 8)
    }
    #[doc = "Bit 9 - EMMC clock frequency mode selection bit"]
    #[inline(always)]
    pub fn rb_emmc_clkmode(&mut self) -> RbEmmcClkmodeW<'_, R16EmmcClkDivSpec> {
        RbEmmcClkmodeW::new(self, 9)
    }
    #[doc = "Bit 10 - invert chip output sdclk phase"]
    #[inline(always)]
    pub fn rb_emmc_phaseinv(&mut self) -> RbEmmcPhaseinvW<'_, R16EmmcClkDivSpec> {
        RbEmmcPhaseinvW::new(self, 10)
    }
}
#[doc = "SD clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16EmmcClkDivSpec;
impl crate::RegisterSpec for R16EmmcClkDivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_emmc_clk_div::R`](R) reader structure"]
impl crate::Readable for R16EmmcClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_emmc_clk_div::W`](W) writer structure"]
impl crate::Writable for R16EmmcClkDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_EMMC_CLK_DIV to value 0x0213"]
impl crate::Resettable for R16EmmcClkDivSpec {
    const RESET_VALUE: u16 = 0x0213;
}
