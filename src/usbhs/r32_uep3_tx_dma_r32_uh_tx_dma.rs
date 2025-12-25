#[doc = "Register `R32_UEP3_TX_DMA_R32_UH_TX_DMA` reader"]
pub type R = crate::R<R32Uep3TxDmaR32UhTxDmaSpec>;
#[doc = "Register `R32_UEP3_TX_DMA_R32_UH_TX_DMA` writer"]
pub type W = crate::W<R32Uep3TxDmaR32UhTxDmaSpec>;
#[doc = "Field `UEP3_TX_DMA_UH_TX_DMA` reader - endpoint 3 DMA TX buffer address / host tx endpoint buffer start address"]
pub type Uep3TxDmaUhTxDmaR = crate::FieldReader<u32>;
#[doc = "Field `UEP3_TX_DMA_UH_TX_DMA` writer - endpoint 3 DMA TX buffer address / host tx endpoint buffer start address"]
pub type Uep3TxDmaUhTxDmaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - endpoint 3 DMA TX buffer address / host tx endpoint buffer start address"]
    #[inline(always)]
    pub fn uep3_tx_dma_uh_tx_dma(&self) -> Uep3TxDmaUhTxDmaR {
        Uep3TxDmaUhTxDmaR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 3 DMA TX buffer address / host tx endpoint buffer start address"]
    #[inline(always)]
    pub fn uep3_tx_dma_uh_tx_dma(&mut self) -> Uep3TxDmaUhTxDmaW<'_, R32Uep3TxDmaR32UhTxDmaSpec> {
        Uep3TxDmaUhTxDmaW::new(self, 0)
    }
}
#[doc = "endpoint 3 DMA TX buffer address / host tx endpoint buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep3_tx_dma_r32_uh_tx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep3_tx_dma_r32_uh_tx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Uep3TxDmaR32UhTxDmaSpec;
impl crate::RegisterSpec for R32Uep3TxDmaR32UhTxDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_uep3_tx_dma_r32_uh_tx_dma::R`](R) reader structure"]
impl crate::Readable for R32Uep3TxDmaR32UhTxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_uep3_tx_dma_r32_uh_tx_dma::W`](W) writer structure"]
impl crate::Writable for R32Uep3TxDmaR32UhTxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_UEP3_TX_DMA_R32_UH_TX_DMA to value 0"]
impl crate::Resettable for R32Uep3TxDmaR32UhTxDmaSpec {}
