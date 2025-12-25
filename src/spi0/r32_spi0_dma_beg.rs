#[doc = "Register `R32_SPI0_DMA_BEG` reader"]
pub type R = crate::R<R32Spi0DmaBegSpec>;
#[doc = "Register `R32_SPI0_DMA_BEG` writer"]
pub type W = crate::W<R32Spi0DmaBegSpec>;
#[doc = "Field `R16_SPI0_DMA_BEG` reader - SPI DMA begin address"]
pub type R16Spi0DmaBegR = crate::FieldReader<u32>;
#[doc = "Field `R16_SPI0_DMA_BEG` writer - SPI DMA begin address"]
pub type R16Spi0DmaBegW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - SPI DMA begin address"]
    #[inline(always)]
    pub fn r16_spi0_dma_beg(&self) -> R16Spi0DmaBegR {
        R16Spi0DmaBegR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - SPI DMA begin address"]
    #[inline(always)]
    pub fn r16_spi0_dma_beg(&mut self) -> R16Spi0DmaBegW<'_, R32Spi0DmaBegSpec> {
        R16Spi0DmaBegW::new(self, 0)
    }
}
#[doc = "SPI0 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_spi0_dma_beg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_spi0_dma_beg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Spi0DmaBegSpec;
impl crate::RegisterSpec for R32Spi0DmaBegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_spi0_dma_beg::R`](R) reader structure"]
impl crate::Readable for R32Spi0DmaBegSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_spi0_dma_beg::W`](W) writer structure"]
impl crate::Writable for R32Spi0DmaBegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_SPI0_DMA_BEG to value 0"]
impl crate::Resettable for R32Spi0DmaBegSpec {}
