#[doc = "Register `R32_TMR1_DMA_BEG` reader"]
pub type R = crate::R<R32Tmr1DmaBegSpec>;
#[doc = "Register `R32_TMR1_DMA_BEG` writer"]
pub type W = crate::W<R32Tmr1DmaBegSpec>;
#[doc = "Field `R16_TMR1_DMA_BEG` reader - TMR1 DMA begin address"]
pub type R16Tmr1DmaBegR = crate::FieldReader<u32>;
#[doc = "Field `R16_TMR1_DMA_BEG` writer - TMR1 DMA begin address"]
pub type R16Tmr1DmaBegW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - TMR1 DMA begin address"]
    #[inline(always)]
    pub fn r16_tmr1_dma_beg(&self) -> R16Tmr1DmaBegR {
        R16Tmr1DmaBegR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - TMR1 DMA begin address"]
    #[inline(always)]
    pub fn r16_tmr1_dma_beg(&mut self) -> R16Tmr1DmaBegW<'_, R32Tmr1DmaBegSpec> {
        R16Tmr1DmaBegW::new(self, 0)
    }
}
#[doc = "TMR1 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_dma_beg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr1_dma_beg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr1DmaBegSpec;
impl crate::RegisterSpec for R32Tmr1DmaBegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr1_dma_beg::R`](R) reader structure"]
impl crate::Readable for R32Tmr1DmaBegSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr1_dma_beg::W`](W) writer structure"]
impl crate::Writable for R32Tmr1DmaBegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR1_DMA_BEG to value 0"]
impl crate::Resettable for R32Tmr1DmaBegSpec {}
