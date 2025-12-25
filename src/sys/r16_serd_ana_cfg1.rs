#[doc = "Register `R16_SERD_ANA_CFG1` reader"]
pub type R = crate::R<R16SerdAnaCfg1Spec>;
#[doc = "Register `R16_SERD_ANA_CFG1` writer"]
pub type W = crate::W<R16SerdAnaCfg1Spec>;
#[doc = "Field `RB_SERD_PLL_CFG` reader - SerDes PHY internal configuration bit"]
pub type RbSerdPllCfgR = crate::FieldReader;
#[doc = "Field `RB_SERD_PLL_CFG` writer - SerDes PHY internal configuration bit"]
pub type RbSerdPllCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RB_SERD_30M_SEL` reader - SerDes PHY reference clock source seletion"]
pub type RbSerd30mSelR = crate::BitReader;
#[doc = "Field `RB_SERD_30M_SEL` writer - SerDes PHY reference clock source seletion"]
pub type RbSerd30mSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SERD_DN_TST` reader - Enable SerDes PHY GXM test pin"]
pub type RbSerdDnTstR = crate::BitReader;
#[doc = "Field `RB_SERD_DN_TST` writer - Enable SerDes PHY GXM test pin"]
pub type RbSerdDnTstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - SerDes PHY internal configuration bit"]
    #[inline(always)]
    pub fn rb_serd_pll_cfg(&self) -> RbSerdPllCfgR {
        RbSerdPllCfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - SerDes PHY reference clock source seletion"]
    #[inline(always)]
    pub fn rb_serd_30m_sel(&self) -> RbSerd30mSelR {
        RbSerd30mSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable SerDes PHY GXM test pin"]
    #[inline(always)]
    pub fn rb_serd_dn_tst(&self) -> RbSerdDnTstR {
        RbSerdDnTstR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - SerDes PHY internal configuration bit"]
    #[inline(always)]
    pub fn rb_serd_pll_cfg(&mut self) -> RbSerdPllCfgW<'_, R16SerdAnaCfg1Spec> {
        RbSerdPllCfgW::new(self, 0)
    }
    #[doc = "Bit 8 - SerDes PHY reference clock source seletion"]
    #[inline(always)]
    pub fn rb_serd_30m_sel(&mut self) -> RbSerd30mSelW<'_, R16SerdAnaCfg1Spec> {
        RbSerd30mSelW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable SerDes PHY GXM test pin"]
    #[inline(always)]
    pub fn rb_serd_dn_tst(&mut self) -> RbSerdDnTstW<'_, R16SerdAnaCfg1Spec> {
        RbSerdDnTstW::new(self, 9)
    }
}
#[doc = "Serdes Analog parameter configuration1\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_serd_ana_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_serd_ana_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16SerdAnaCfg1Spec;
impl crate::RegisterSpec for R16SerdAnaCfg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_serd_ana_cfg1::R`](R) reader structure"]
impl crate::Readable for R16SerdAnaCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`r16_serd_ana_cfg1::W`](W) writer structure"]
impl crate::Writable for R16SerdAnaCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_SERD_ANA_CFG1 to value 0x5a"]
impl crate::Resettable for R16SerdAnaCfg1Spec {
    const RESET_VALUE: u16 = 0x5a;
}
