#[doc = "Register `R32_UEP1_TX_DMA` reader"]
pub type R = crate::R<R32Uep1TxDmaSpec>;
#[doc = "Register `R32_UEP1_TX_DMA` writer"]
pub type W = crate::W<R32Uep1TxDmaSpec>;
#[doc = "Field `UEP1_TX_DMA` reader - endpoint 1 DMA TX buffer address"]
pub type Uep1TxDmaR = crate::FieldReader<u32>;
#[doc = "Field `UEP1_TX_DMA` writer - endpoint 1 DMA TX buffer address"]
pub type Uep1TxDmaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep1_tx_dma(&self) -> Uep1TxDmaR {
        Uep1TxDmaR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep1_tx_dma(&mut self) -> Uep1TxDmaW<'_, R32Uep1TxDmaSpec> {
        Uep1TxDmaW::new(self, 0)
    }
}
#[doc = "endpoint 1 DMA TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep1_tx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep1_tx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Uep1TxDmaSpec;
impl crate::RegisterSpec for R32Uep1TxDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_uep1_tx_dma::R`](R) reader structure"]
impl crate::Readable for R32Uep1TxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_uep1_tx_dma::W`](W) writer structure"]
impl crate::Writable for R32Uep1TxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_UEP1_TX_DMA to value 0"]
impl crate::Resettable for R32Uep1TxDmaSpec {}
