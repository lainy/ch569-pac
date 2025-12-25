#[doc = "Register `R32_UEP2_RX_DMA_R32_UH_RX_DMA` reader"]
pub type R = crate::R<R32Uep2RxDmaR32UhRxDmaSpec>;
#[doc = "Register `R32_UEP2_RX_DMA_R32_UH_RX_DMA` writer"]
pub type W = crate::W<R32Uep2RxDmaR32UhRxDmaSpec>;
#[doc = "Field `UEP2_RX_DMA_UH_RX_DMA` reader - endpoint 2 DMA buffer address / host rx endpoint buffer start address"]
pub type Uep2RxDmaUhRxDmaR = crate::FieldReader<u32>;
#[doc = "Field `UEP2_RX_DMA_UH_RX_DMA` writer - endpoint 2 DMA buffer address / host rx endpoint buffer start address"]
pub type Uep2RxDmaUhRxDmaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - endpoint 2 DMA buffer address / host rx endpoint buffer start address"]
    #[inline(always)]
    pub fn uep2_rx_dma_uh_rx_dma(&self) -> Uep2RxDmaUhRxDmaR {
        Uep2RxDmaUhRxDmaR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 2 DMA buffer address / host rx endpoint buffer start address"]
    #[inline(always)]
    pub fn uep2_rx_dma_uh_rx_dma(&mut self) -> Uep2RxDmaUhRxDmaW<'_, R32Uep2RxDmaR32UhRxDmaSpec> {
        Uep2RxDmaUhRxDmaW::new(self, 0)
    }
}
#[doc = "endpoint 2 DMA buffer address / host rx endpoint buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep2_rx_dma_r32_uh_rx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep2_rx_dma_r32_uh_rx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Uep2RxDmaR32UhRxDmaSpec;
impl crate::RegisterSpec for R32Uep2RxDmaR32UhRxDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_uep2_rx_dma_r32_uh_rx_dma::R`](R) reader structure"]
impl crate::Readable for R32Uep2RxDmaR32UhRxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_uep2_rx_dma_r32_uh_rx_dma::W`](W) writer structure"]
impl crate::Writable for R32Uep2RxDmaR32UhRxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_UEP2_RX_DMA_R32_UH_RX_DMA to value 0"]
impl crate::Resettable for R32Uep2RxDmaR32UhRxDmaSpec {}
