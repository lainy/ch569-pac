#[doc = "Register `R32_HSPI_RX_ADDR1` reader"]
pub type R = crate::R<R32HspiRxAddr1Spec>;
#[doc = "Register `R32_HSPI_RX_ADDR1` writer"]
pub type W = crate::W<R32HspiRxAddr1Spec>;
#[doc = "Field `RB_HSPI_RX_ADDR1` reader - parallel if dma rx addr1"]
pub type RbHspiRxAddr1R = crate::FieldReader<u32>;
#[doc = "Field `RB_HSPI_RX_ADDR1` writer - parallel if dma rx addr1"]
pub type RbHspiRxAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - parallel if dma rx addr1"]
    #[inline(always)]
    pub fn rb_hspi_rx_addr1(&self) -> RbHspiRxAddr1R {
        RbHspiRxAddr1R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - parallel if dma rx addr1"]
    #[inline(always)]
    pub fn rb_hspi_rx_addr1(&mut self) -> RbHspiRxAddr1W<'_, R32HspiRxAddr1Spec> {
        RbHspiRxAddr1W::new(self, 0)
    }
}
#[doc = "parallel if dma rx addr1\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_rx_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_rx_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32HspiRxAddr1Spec;
impl crate::RegisterSpec for R32HspiRxAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_hspi_rx_addr1::R`](R) reader structure"]
impl crate::Readable for R32HspiRxAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_hspi_rx_addr1::W`](W) writer structure"]
impl crate::Writable for R32HspiRxAddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_HSPI_RX_ADDR1 to value 0"]
impl crate::Resettable for R32HspiRxAddr1Spec {}
