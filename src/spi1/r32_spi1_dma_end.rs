#[doc = "Register `R32_SPI1_DMA_END` reader"]
pub type R = crate::R<R32Spi1DmaEndSpec>;
#[doc = "Register `R32_SPI1_DMA_END` writer"]
pub type W = crate::W<R32Spi1DmaEndSpec>;
#[doc = "Field `R16_SPI1_DMA_END` reader - SPI DMA end address"]
pub type R16Spi1DmaEndR = crate::FieldReader<u32>;
#[doc = "Field `R16_SPI1_DMA_END` writer - SPI DMA end address"]
pub type R16Spi1DmaEndW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - SPI DMA end address"]
    #[inline(always)]
    pub fn r16_spi1_dma_end(&self) -> R16Spi1DmaEndR {
        R16Spi1DmaEndR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - SPI DMA end address"]
    #[inline(always)]
    pub fn r16_spi1_dma_end(&mut self) -> R16Spi1DmaEndW<'_, R32Spi1DmaEndSpec> {
        R16Spi1DmaEndW::new(self, 0)
    }
}
#[doc = "SPI1 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi1_dma_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi1_dma_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Spi1DmaEndSpec;
impl crate::RegisterSpec for R32Spi1DmaEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_spi1_dma_end::R`](R) reader structure"]
impl crate::Readable for R32Spi1DmaEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_spi1_dma_end::W`](W) writer structure"]
impl crate::Writable for R32Spi1DmaEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_SPI1_DMA_END to value 0"]
impl crate::Resettable for R32Spi1DmaEndSpec {}
