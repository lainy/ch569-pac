#[doc = "Register `R32_UEP3_RX_DMA` reader"]
pub type R = crate::R<R32Uep3RxDmaSpec>;
#[doc = "Register `R32_UEP3_RX_DMA` writer"]
pub type W = crate::W<R32Uep3RxDmaSpec>;
#[doc = "Field `UEP3_RX_DMA` reader - endpoint 3 DMA buffer address"]
pub type Uep3RxDmaR = crate::FieldReader<u32>;
#[doc = "Field `UEP3_RX_DMA` writer - endpoint 3 DMA buffer address"]
pub type Uep3RxDmaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - endpoint 3 DMA buffer address"]
    #[inline(always)]
    pub fn uep3_rx_dma(&self) -> Uep3RxDmaR {
        Uep3RxDmaR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 3 DMA buffer address"]
    #[inline(always)]
    pub fn uep3_rx_dma(&mut self) -> Uep3RxDmaW<'_, R32Uep3RxDmaSpec> {
        Uep3RxDmaW::new(self, 0)
    }
}
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_uep3_rx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_uep3_rx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Uep3RxDmaSpec;
impl crate::RegisterSpec for R32Uep3RxDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_uep3_rx_dma::R`](R) reader structure"]
impl crate::Readable for R32Uep3RxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_uep3_rx_dma::W`](W) writer structure"]
impl crate::Writable for R32Uep3RxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_UEP3_RX_DMA to value 0"]
impl crate::Resettable for R32Uep3RxDmaSpec {}
