#[doc = "Register `R8_TMR1_INT_FLAG` reader"]
pub type R = crate::R<R8Tmr1IntFlagSpec>;
#[doc = "Register `R8_TMR1_INT_FLAG` writer"]
pub type W = crate::W<R8Tmr1IntFlagSpec>;
#[doc = "Field `RB_TMR_IF_CYC_END` reader - interrupt flag for timer capture count timeout or PWM cycle end"]
pub type RbTmrIfCycEndR = crate::BitReader;
#[doc = "Field `RB_TMR_IF_CYC_END` writer - interrupt flag for timer capture count timeout or PWM cycle end"]
pub type RbTmrIfCycEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IF_DATA_ACT` reader - interrupt flag for timer capture input action or PWM trigger"]
pub type RbTmrIfDataActR = crate::BitReader;
#[doc = "Field `RB_TMR_IF_DATA_ACT` writer - interrupt flag for timer capture input action or PWM trigger"]
pub type RbTmrIfDataActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IF_FIFO_HF` reader - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
pub type RbTmrIfFifoHfR = crate::BitReader;
#[doc = "Field `RB_TMR_IF_FIFO_HF` writer - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
pub type RbTmrIfFifoHfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IF_DMA_END` reader - interrupt flag for timer1/2 DMA completion"]
pub type RbTmrIfDmaEndR = crate::BitReader;
#[doc = "Field `RB_TMR_IF_DMA_END` writer - interrupt flag for timer1/2 DMA completion"]
pub type RbTmrIfDmaEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_TMR_IF_FIFO_OV` reader - interrupt flag for timer FIFO overflow"]
pub type RbTmrIfFifoOvR = crate::BitReader;
#[doc = "Field `RB_TMR_IF_FIFO_OV` writer - interrupt flag for timer FIFO overflow"]
pub type RbTmrIfFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt flag for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn rb_tmr_if_cyc_end(&self) -> RbTmrIfCycEndR {
        RbTmrIfCycEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn rb_tmr_if_data_act(&self) -> RbTmrIfDataActR {
        RbTmrIfDataActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_hf(&self) -> RbTmrIfFifoHfR {
        RbTmrIfFifoHfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn rb_tmr_if_dma_end(&self) -> RbTmrIfDmaEndR {
        RbTmrIfDmaEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt flag for timer FIFO overflow"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_ov(&self) -> RbTmrIfFifoOvR {
        RbTmrIfFifoOvR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn rb_tmr_if_cyc_end(&mut self) -> RbTmrIfCycEndW<'_, R8Tmr1IntFlagSpec> {
        RbTmrIfCycEndW::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt flag for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn rb_tmr_if_data_act(&mut self) -> RbTmrIfDataActW<'_, R8Tmr1IntFlagSpec> {
        RbTmrIfDataActW::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_hf(&mut self) -> RbTmrIfFifoHfW<'_, R8Tmr1IntFlagSpec> {
        RbTmrIfFifoHfW::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt flag for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn rb_tmr_if_dma_end(&mut self) -> RbTmrIfDmaEndW<'_, R8Tmr1IntFlagSpec> {
        RbTmrIfDmaEndW::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt flag for timer FIFO overflow"]
    #[inline(always)]
    pub fn rb_tmr_if_fifo_ov(&mut self) -> RbTmrIfFifoOvW<'_, R8Tmr1IntFlagSpec> {
        RbTmrIfFifoOvW::new(self, 4)
    }
}
#[doc = "TMR1 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_int_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_tmr1_int_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr1IntFlagSpec;
impl crate::RegisterSpec for R8Tmr1IntFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr1_int_flag::R`](R) reader structure"]
impl crate::Readable for R8Tmr1IntFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_tmr1_int_flag::W`](W) writer structure"]
impl crate::Writable for R8Tmr1IntFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_TMR1_INT_FLAG to value 0"]
impl crate::Resettable for R8Tmr1IntFlagSpec {}
