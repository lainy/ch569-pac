#[doc = "Register `R8_SPI0_FIFO_COUNT` reader"]
pub type R = crate::R<R8Spi0FifoCountSpec>;
#[doc = "Register `R8_SPI0_FIFO_COUNT` writer"]
pub type W = crate::W<R8Spi0FifoCountSpec>;
#[doc = "Field `R8_SPI0_FIFO_COUNT` reader - SPI FIFO count status"]
pub type R8Spi0FifoCountR = crate::FieldReader;
#[doc = "Field `R8_SPI0_FIFO_COUNT` writer - SPI FIFO count status"]
pub type R8Spi0FifoCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn r8_spi0_fifo_count(&self) -> R8Spi0FifoCountR {
        R8Spi0FifoCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn r8_spi0_fifo_count(&mut self) -> R8Spi0FifoCountW<'_, R8Spi0FifoCountSpec> {
        R8Spi0FifoCountW::new(self, 0)
    }
}
#[doc = "SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_fifo_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0FifoCountSpec;
impl crate::RegisterSpec for R8Spi0FifoCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_fifo_count::R`](R) reader structure"]
impl crate::Readable for R8Spi0FifoCountSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi0_fifo_count::W`](W) writer structure"]
impl crate::Writable for R8Spi0FifoCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI0_FIFO_COUNT to value 0"]
impl crate::Resettable for R8Spi0FifoCountSpec {}
