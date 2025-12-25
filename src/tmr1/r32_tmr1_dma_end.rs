#[doc = "Register `R32_TMR1_DMA_END` reader"]
pub type R = crate::R<R32Tmr1DmaEndSpec>;
#[doc = "Register `R32_TMR1_DMA_END` writer"]
pub type W = crate::W<R32Tmr1DmaEndSpec>;
#[doc = "Field `R16_TMR1_DMA_END` reader - TMR1 DMA end address"]
pub type R16Tmr1DmaEndR = crate::FieldReader<u32>;
#[doc = "Field `R16_TMR1_DMA_END` writer - TMR1 DMA end address"]
pub type R16Tmr1DmaEndW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - TMR1 DMA end address"]
    #[inline(always)]
    pub fn r16_tmr1_dma_end(&self) -> R16Tmr1DmaEndR {
        R16Tmr1DmaEndR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - TMR1 DMA end address"]
    #[inline(always)]
    pub fn r16_tmr1_dma_end(&mut self) -> R16Tmr1DmaEndW<'_, R32Tmr1DmaEndSpec> {
        R16Tmr1DmaEndW::new(self, 0)
    }
}
#[doc = "TMR1 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_dma_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr1_dma_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr1DmaEndSpec;
impl crate::RegisterSpec for R32Tmr1DmaEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr1_dma_end::R`](R) reader structure"]
impl crate::Readable for R32Tmr1DmaEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr1_dma_end::W`](W) writer structure"]
impl crate::Writable for R32Tmr1DmaEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR1_DMA_END to value 0"]
impl crate::Resettable for R32Tmr1DmaEndSpec {}
