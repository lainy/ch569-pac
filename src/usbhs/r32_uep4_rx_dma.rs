#[doc = "Register `R32_UEP4_RX_DMA` reader"]
pub type R = crate::R<R32Uep4RxDmaSpec>;
#[doc = "Register `R32_UEP4_RX_DMA` writer"]
pub type W = crate::W<R32Uep4RxDmaSpec>;
#[doc = "Field `UEP4_RX_DMA` reader - endpoint 4 DMA buffer address"]
pub type Uep4RxDmaR = crate::FieldReader<u32>;
#[doc = "Field `UEP4_RX_DMA` writer - endpoint 4 DMA buffer address"]
pub type Uep4RxDmaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - endpoint 4 DMA buffer address"]
    #[inline(always)]
    pub fn uep4_rx_dma(&self) -> Uep4RxDmaR {
        Uep4RxDmaR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 4 DMA buffer address"]
    #[inline(always)]
    pub fn uep4_rx_dma(&mut self) -> Uep4RxDmaW<'_, R32Uep4RxDmaSpec> {
        Uep4RxDmaW::new(self, 0)
    }
}
#[doc = "endpoint 4 DMA buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep4_rx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep4_rx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Uep4RxDmaSpec;
impl crate::RegisterSpec for R32Uep4RxDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_uep4_rx_dma::R`](R) reader structure"]
impl crate::Readable for R32Uep4RxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_uep4_rx_dma::W`](W) writer structure"]
impl crate::Writable for R32Uep4RxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_UEP4_RX_DMA to value 0"]
impl crate::Resettable for R32Uep4RxDmaSpec {}
