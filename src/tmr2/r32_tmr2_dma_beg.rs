#[doc = "Register `R32_TMR2_DMA_BEG` reader"]
pub type R = crate::R<R32Tmr2DmaBegSpec>;
#[doc = "Register `R32_TMR2_DMA_BEG` writer"]
pub type W = crate::W<R32Tmr2DmaBegSpec>;
#[doc = "Field `R16_TMR2_DMA_BEG` reader - TMR2 DMA begin address"]
pub type R16Tmr2DmaBegR = crate::FieldReader<u32>;
#[doc = "Field `R16_TMR2_DMA_BEG` writer - TMR2 DMA begin address"]
pub type R16Tmr2DmaBegW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - TMR2 DMA begin address"]
    #[inline(always)]
    pub fn r16_tmr2_dma_beg(&self) -> R16Tmr2DmaBegR {
        R16Tmr2DmaBegR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - TMR2 DMA begin address"]
    #[inline(always)]
    pub fn r16_tmr2_dma_beg(&mut self) -> R16Tmr2DmaBegW<'_, R32Tmr2DmaBegSpec> {
        R16Tmr2DmaBegW::new(self, 0)
    }
}
#[doc = "TMR2 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_dma_beg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_dma_beg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr2DmaBegSpec;
impl crate::RegisterSpec for R32Tmr2DmaBegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr2_dma_beg::R`](R) reader structure"]
impl crate::Readable for R32Tmr2DmaBegSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr2_dma_beg::W`](W) writer structure"]
impl crate::Writable for R32Tmr2DmaBegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR2_DMA_BEG to value 0"]
impl crate::Resettable for R32Tmr2DmaBegSpec {}
