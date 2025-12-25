#[doc = "Register `R32_SERD_ANA_CFG2` reader"]
pub type R = crate::R<R32SerdAnaCfg2Spec>;
#[doc = "Register `R32_SERD_ANA_CFG2` writer"]
pub type W = crate::W<R32SerdAnaCfg2Spec>;
#[doc = "Field `RB_SERD_TRX_CFG` reader - Tx and RX parameter setting"]
pub type RbSerdTrxCfgR = crate::FieldReader<u32>;
#[doc = "Field `RB_SERD_TRX_CFG` writer - Tx and RX parameter setting"]
pub type RbSerdTrxCfgW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Tx and RX parameter setting"]
    #[inline(always)]
    pub fn rb_serd_trx_cfg(&self) -> RbSerdTrxCfgR {
        RbSerdTrxCfgR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Tx and RX parameter setting"]
    #[inline(always)]
    pub fn rb_serd_trx_cfg(&mut self) -> RbSerdTrxCfgW<'_, R32SerdAnaCfg2Spec> {
        RbSerdTrxCfgW::new(self, 0)
    }
}
#[doc = "Serdes Analog parameter configuration2\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_serd_ana_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_serd_ana_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32SerdAnaCfg2Spec;
impl crate::RegisterSpec for R32SerdAnaCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_serd_ana_cfg2::R`](R) reader structure"]
impl crate::Readable for R32SerdAnaCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_serd_ana_cfg2::W`](W) writer structure"]
impl crate::Writable for R32SerdAnaCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_SERD_ANA_CFG2 to value 0x0042_3015"]
impl crate::Resettable for R32SerdAnaCfg2Spec {
    const RESET_VALUE: u32 = 0x0042_3015;
}
