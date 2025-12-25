#[doc = "Register `R32_HSPI_RX_ADDR0` reader"]
pub type R = crate::R<R32HspiRxAddr0Spec>;
#[doc = "Register `R32_HSPI_RX_ADDR0` writer"]
pub type W = crate::W<R32HspiRxAddr0Spec>;
#[doc = "Field `RB_HSPI_RX_ADDR0` reader - parallel if dma rx addr0"]
pub type RbHspiRxAddr0R = crate::FieldReader<u32>;
#[doc = "Field `RB_HSPI_RX_ADDR0` writer - parallel if dma rx addr0"]
pub type RbHspiRxAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - parallel if dma rx addr0"]
    #[inline(always)]
    pub fn rb_hspi_rx_addr0(&self) -> RbHspiRxAddr0R {
        RbHspiRxAddr0R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - parallel if dma rx addr0"]
    #[inline(always)]
    pub fn rb_hspi_rx_addr0(&mut self) -> RbHspiRxAddr0W<'_, R32HspiRxAddr0Spec> {
        RbHspiRxAddr0W::new(self, 0)
    }
}
#[doc = "parallel if dma rx addr0\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_rx_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_rx_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32HspiRxAddr0Spec;
impl crate::RegisterSpec for R32HspiRxAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_hspi_rx_addr0::R`](R) reader structure"]
impl crate::Readable for R32HspiRxAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_hspi_rx_addr0::W`](W) writer structure"]
impl crate::Writable for R32HspiRxAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_HSPI_RX_ADDR0 to value 0"]
impl crate::Resettable for R32HspiRxAddr0Spec {}
