#[doc = "Register `R8_TMR2_CTRL_DMA` reader"]
pub type R = crate::R<R8Tmr2CtrlDmaSpec>;
#[doc = "Register `R8_TMR2_CTRL_DMA` writer"]
pub type W = crate::W<R8Tmr2CtrlDmaSpec>;
#[doc = "Field `RB_TMR_DMA_ENABLE` reader - timer1/2 DMA enable"]
pub type RbTmrDmaEnableR = crate::BitReader;
#[doc = "Field `RB_TMR_DMA_ENABLE` writer - timer1/2 DMA enable"]
pub type RbTmrDmaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_DMA_LOOP` reader - timer1/2 DMA address loop enable"]
pub type RbTmrDmaLoopR = crate::BitReader;
#[doc = "Field `RB_TMR_DMA_LOOP` writer - timer1/2 DMA address loop enable"]
pub type RbTmrDmaLoopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - timer1/2 DMA enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_enable(&self) -> RbTmrDmaEnableR {
        RbTmrDmaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - timer1/2 DMA address loop enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_loop(&self) -> RbTmrDmaLoopR {
        RbTmrDmaLoopR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - timer1/2 DMA enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_enable(&mut self) -> RbTmrDmaEnableW<'_, R8Tmr2CtrlDmaSpec> {
        RbTmrDmaEnableW::new(self, 0)
    }
    #[doc = "Bit 2 - timer1/2 DMA address loop enable"]
    #[inline(always)]
    pub fn rb_tmr_dma_loop(&mut self) -> RbTmrDmaLoopW<'_, R8Tmr2CtrlDmaSpec> {
        RbTmrDmaLoopW::new(self, 2)
    }
}
#[doc = "TMR2 DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_ctrl_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr2_ctrl_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr2CtrlDmaSpec;
impl crate::RegisterSpec for R8Tmr2CtrlDmaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr2_ctrl_dma::R`](R) reader structure"]
impl crate::Readable for R8Tmr2CtrlDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tmr2_ctrl_dma::W`](W) writer structure"]
impl crate::Writable for R8Tmr2CtrlDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TMR2_CTRL_DMA to value 0"]
impl crate::Resettable for R8Tmr2CtrlDmaSpec {}
