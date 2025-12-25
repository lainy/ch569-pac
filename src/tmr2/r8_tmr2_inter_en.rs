#[doc = "Register `R8_TMR2_INTER_EN` reader"]
pub type R = crate::R<R8Tmr2InterEnSpec>;
#[doc = "Register `R8_TMR2_INTER_EN` writer"]
pub type W = crate::W<R8Tmr2InterEnSpec>;
#[doc = "Field `RB_TMR_IE_CYC_END` reader - enable interrupt for timer capture count timeout or PWM cycle end"]
pub type RbTmrIeCycEndR = crate::BitReader;
#[doc = "Field `RB_TMR_IE_CYC_END` writer - enable interrupt for timer capture count timeout or PWM cycle end"]
pub type RbTmrIeCycEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IE_DATA_ACT` reader - enable interrupt for timer capture input action or PWM trigger"]
pub type RbTmrIeDataActR = crate::BitReader;
#[doc = "Field `RB_TMR_IE_DATA_ACT` writer - enable interrupt for timer capture input action or PWM trigger"]
pub type RbTmrIeDataActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IE_FIFO_HF` reader - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
pub type RbTmrIeFifoHfR = crate::BitReader;
#[doc = "Field `RB_TMR_IE_FIFO_HF` writer - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
pub type RbTmrIeFifoHfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IE_DMA_END` reader - enable interrupt for timer1/2 DMA completion"]
pub type RbTmrIeDmaEndR = crate::BitReader;
#[doc = "Field `RB_TMR_IE_DMA_END` writer - enable interrupt for timer1/2 DMA completion"]
pub type RbTmrIeDmaEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IE_FIFO_OV` reader - enable interrupt for timer FIFO overflow"]
pub type RbTmrIeFifoOvR = crate::BitReader;
#[doc = "Field `RB_TMR_IE_FIFO_OV` writer - enable interrupt for timer FIFO overflow"]
pub type RbTmrIeFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable interrupt for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn rb_tmr_ie_cyc_end(&self) -> RbTmrIeCycEndR {
        RbTmrIeCycEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn rb_tmr_ie_data_act(&self) -> RbTmrIeDataActR {
        RbTmrIeDataActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
    #[inline(always)]
    pub fn rb_tmr_ie_fifo_hf(&self) -> RbTmrIeFifoHfR {
        RbTmrIeFifoHfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn rb_tmr_ie_dma_end(&self) -> RbTmrIeDmaEndR {
        RbTmrIeDmaEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for timer FIFO overflow"]
    #[inline(always)]
    pub fn rb_tmr_ie_fifo_ov(&self) -> RbTmrIeFifoOvR {
        RbTmrIeFifoOvR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn rb_tmr_ie_cyc_end(&mut self) -> RbTmrIeCycEndW<'_, R8Tmr2InterEnSpec> {
        RbTmrIeCycEndW::new(self, 0)
    }
    #[doc = "Bit 1 - enable interrupt for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn rb_tmr_ie_data_act(&mut self) -> RbTmrIeDataActW<'_, R8Tmr2InterEnSpec> {
        RbTmrIeDataActW::new(self, 1)
    }
    #[doc = "Bit 2 - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
    #[inline(always)]
    pub fn rb_tmr_ie_fifo_hf(&mut self) -> RbTmrIeFifoHfW<'_, R8Tmr2InterEnSpec> {
        RbTmrIeFifoHfW::new(self, 2)
    }
    #[doc = "Bit 3 - enable interrupt for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn rb_tmr_ie_dma_end(&mut self) -> RbTmrIeDmaEndW<'_, R8Tmr2InterEnSpec> {
        RbTmrIeDmaEndW::new(self, 3)
    }
    #[doc = "Bit 4 - enable interrupt for timer FIFO overflow"]
    #[inline(always)]
    pub fn rb_tmr_ie_fifo_ov(&mut self) -> RbTmrIeFifoOvW<'_, R8Tmr2InterEnSpec> {
        RbTmrIeFifoOvW::new(self, 4)
    }
}
#[doc = "TMR2 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr2_inter_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr2_inter_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr2InterEnSpec;
impl crate::RegisterSpec for R8Tmr2InterEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr2_inter_en::R`](R) reader structure"]
impl crate::Readable for R8Tmr2InterEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tmr2_inter_en::W`](W) writer structure"]
impl crate::Writable for R8Tmr2InterEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TMR2_INTER_EN to value 0"]
impl crate::Resettable for R8Tmr2InterEnSpec {}
