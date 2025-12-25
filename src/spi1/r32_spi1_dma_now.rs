#[doc = "Register `R32_SPI1_DMA_NOW` reader"]
pub type R = crate::R<R32Spi1DmaNowSpec>;
#[doc = "Register `R32_SPI1_DMA_NOW` writer"]
pub type W = crate::W<R32Spi1DmaNowSpec>;
#[doc = "Field `R16_SPI1_DMA_NOW` reader - SPI DMA current address"]
pub type R16Spi1DmaNowR = crate::FieldReader<u32>;
#[doc = "Field `R16_SPI1_DMA_NOW` writer - SPI DMA current address"]
pub type R16Spi1DmaNowW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - SPI DMA current address"]
    #[inline(always)]
    pub fn r16_spi1_dma_now(&self) -> R16Spi1DmaNowR {
        R16Spi1DmaNowR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - SPI DMA current address"]
    #[inline(always)]
    pub fn r16_spi1_dma_now(&mut self) -> R16Spi1DmaNowW<'_, R32Spi1DmaNowSpec> {
        R16Spi1DmaNowW::new(self, 0)
    }
}
#[doc = "SPI1 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi1_dma_now::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi1_dma_now::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Spi1DmaNowSpec;
impl crate::RegisterSpec for R32Spi1DmaNowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_spi1_dma_now::R`](R) reader structure"]
impl crate::Readable for R32Spi1DmaNowSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_spi1_dma_now::W`](W) writer structure"]
impl crate::Writable for R32Spi1DmaNowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_SPI1_DMA_NOW to value 0"]
impl crate::Resettable for R32Spi1DmaNowSpec {}
