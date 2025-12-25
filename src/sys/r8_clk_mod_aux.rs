#[doc = "Register `R8_CLK_MOD_AUX` reader"]
pub type R = crate::R<R8ClkModAuxSpec>;
#[doc = "Register `R8_CLK_MOD_AUX` writer"]
pub type W = crate::W<R8ClkModAuxSpec>;
#[doc = "Field `RB_INT_125M_EN` reader - clock from USB_PHY PCLK(125MHz)"]
pub type RbInt125mEnR = crate::BitReader;
#[doc = "Field `RB_INT_125M_EN` writer - clock from USB_PHY PCLK(125MHz)"]
pub type RbInt125mEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EXT_125M_EN` reader - clock from pin_PA\\[16\\]"]
pub type RbExt125mEnR = crate::BitReader;
#[doc = "Field `RB_EXT_125M_EN` writer - clock from pin_PA\\[16\\]"]
pub type RbExt125mEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCO_SEL_MSK` reader - MCO output selection"]
pub type RbMcoSelMskR = crate::FieldReader;
#[doc = "Field `RB_MCO_SEL_MSK` writer - MCO output selection"]
pub type RbMcoSelMskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_MCO_EN` reader - MCO output enable"]
pub type RbMcoEnR = crate::BitReader;
#[doc = "Field `RB_MCO_EN` writer - MCO output enable"]
pub type RbMcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clock from USB_PHY PCLK(125MHz)"]
    #[inline(always)]
    pub fn rb_int_125m_en(&self) -> RbInt125mEnR {
        RbInt125mEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock from pin_PA\\[16\\]"]
    #[inline(always)]
    pub fn rb_ext_125m_en(&self) -> RbExt125mEnR {
        RbExt125mEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MCO output selection"]
    #[inline(always)]
    pub fn rb_mco_sel_msk(&self) -> RbMcoSelMskR {
        RbMcoSelMskR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - MCO output enable"]
    #[inline(always)]
    pub fn rb_mco_en(&self) -> RbMcoEnR {
        RbMcoEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock from USB_PHY PCLK(125MHz)"]
    #[inline(always)]
    pub fn rb_int_125m_en(&mut self) -> RbInt125mEnW<'_, R8ClkModAuxSpec> {
        RbInt125mEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clock from pin_PA\\[16\\]"]
    #[inline(always)]
    pub fn rb_ext_125m_en(&mut self) -> RbExt125mEnW<'_, R8ClkModAuxSpec> {
        RbExt125mEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - MCO output selection"]
    #[inline(always)]
    pub fn rb_mco_sel_msk(&mut self) -> RbMcoSelMskW<'_, R8ClkModAuxSpec> {
        RbMcoSelMskW::new(self, 2)
    }
    #[doc = "Bit 4 - MCO output enable"]
    #[inline(always)]
    pub fn rb_mco_en(&mut self) -> RbMcoEnW<'_, R8ClkModAuxSpec> {
        RbMcoEnW::new(self, 4)
    }
}
#[doc = "clock mode aux register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_clk_mod_aux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_clk_mod_aux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8ClkModAuxSpec;
impl crate::RegisterSpec for R8ClkModAuxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_clk_mod_aux::R`](R) reader structure"]
impl crate::Readable for R8ClkModAuxSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_clk_mod_aux::W`](W) writer structure"]
impl crate::Writable for R8ClkModAuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_CLK_MOD_AUX to value 0"]
impl crate::Resettable for R8ClkModAuxSpec {}
