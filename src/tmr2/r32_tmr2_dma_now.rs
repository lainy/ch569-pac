#[doc = "Register `R32_TMR2_DMA_NOW` reader"]
pub type R = crate::R<R32Tmr2DmaNowSpec>;
#[doc = "Register `R32_TMR2_DMA_NOW` writer"]
pub type W = crate::W<R32Tmr2DmaNowSpec>;
#[doc = "Field `R16_TMR2_DMA_NOW` reader - TMR DMA current address"]
pub type R16Tmr2DmaNowR = crate::FieldReader<u32>;
#[doc = "Field `R16_TMR2_DMA_NOW` writer - TMR DMA current address"]
pub type R16Tmr2DmaNowW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - TMR DMA current address"]
    #[inline(always)]
    pub fn r16_tmr2_dma_now(&self) -> R16Tmr2DmaNowR {
        R16Tmr2DmaNowR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - TMR DMA current address"]
    #[inline(always)]
    pub fn r16_tmr2_dma_now(&mut self) -> R16Tmr2DmaNowW<'_, R32Tmr2DmaNowSpec> {
        R16Tmr2DmaNowW::new(self, 0)
    }
}
#[doc = "TMR2 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr2_dma_now::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr2_dma_now::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr2DmaNowSpec;
impl crate::RegisterSpec for R32Tmr2DmaNowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr2_dma_now::R`](R) reader structure"]
impl crate::Readable for R32Tmr2DmaNowSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr2_dma_now::W`](W) writer structure"]
impl crate::Writable for R32Tmr2DmaNowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR2_DMA_NOW to value 0"]
impl crate::Resettable for R32Tmr2DmaNowSpec {}
