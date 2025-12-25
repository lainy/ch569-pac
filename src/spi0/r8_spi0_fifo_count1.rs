#[doc = "Register `R8_SPI0_FIFO_COUNT1` reader"]
pub type R = crate::R<R8Spi0FifoCount1Spec>;
#[doc = "Register `R8_SPI0_FIFO_COUNT1` writer"]
pub type W = crate::W<R8Spi0FifoCount1Spec>;
#[doc = "Field `R8_SPI0_FIFO_COUNT1` reader - SPI FIFO count statu"]
pub type R8Spi0FifoCount1R = crate::FieldReader;
#[doc = "Field `R8_SPI0_FIFO_COUNT1` writer - SPI FIFO count statu"]
pub type R8Spi0FifoCount1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI FIFO count statu"]
    #[inline(always)]
    pub fn r8_spi0_fifo_count1(&self) -> R8Spi0FifoCount1R {
        R8Spi0FifoCount1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO count statu"]
    #[inline(always)]
    pub fn r8_spi0_fifo_count1(&mut self) -> R8Spi0FifoCount1W<'_, R8Spi0FifoCount1Spec> {
        R8Spi0FifoCount1W::new(self, 0)
    }
}
#[doc = "SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_fifo_count1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0FifoCount1Spec;
impl crate::RegisterSpec for R8Spi0FifoCount1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_fifo_count1::R`](R) reader structure"]
impl crate::Readable for R8Spi0FifoCount1Spec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi0_fifo_count1::W`](W) writer structure"]
impl crate::Writable for R8Spi0FifoCount1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI0_FIFO_COUNT1 to value 0"]
impl crate::Resettable for R8Spi0FifoCount1Spec {}
