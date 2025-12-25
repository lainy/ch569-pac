#[doc = "Register `R16_HSPI_RX_LEN0` reader"]
pub type R = crate::R<R16HspiRxLen0Spec>;
#[doc = "Register `R16_HSPI_RX_LEN0` writer"]
pub type W = crate::W<R16HspiRxLen0Spec>;
#[doc = "Field `RB_HSPI_RX_LEN0` reader - parallel if dma length0"]
pub type RbHspiRxLen0R = crate::FieldReader<u16>;
#[doc = "Field `RB_HSPI_RX_LEN0` writer - parallel if dma length0"]
pub type RbHspiRxLen0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - parallel if dma length0"]
    #[inline(always)]
    pub fn rb_hspi_rx_len0(&self) -> RbHspiRxLen0R {
        RbHspiRxLen0R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - parallel if dma length0"]
    #[inline(always)]
    pub fn rb_hspi_rx_len0(&mut self) -> RbHspiRxLen0W<'_, R16HspiRxLen0Spec> {
        RbHspiRxLen0W::new(self, 0)
    }
}
#[doc = "parallel if receive length0\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_rx_len0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_rx_len0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16HspiRxLen0Spec;
impl crate::RegisterSpec for R16HspiRxLen0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_hspi_rx_len0::R`](R) reader structure"]
impl crate::Readable for R16HspiRxLen0Spec {}
#[doc = "`write(|w| ..)` method takes [`r16_hspi_rx_len0::W`](W) writer structure"]
impl crate::Writable for R16HspiRxLen0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_HSPI_RX_LEN0 to value 0"]
impl crate::Resettable for R16HspiRxLen0Spec {}
