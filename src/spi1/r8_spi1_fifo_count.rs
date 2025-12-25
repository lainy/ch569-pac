#[doc = "Register `R8_SPI1_FIFO_COUNT` reader"]
pub type R = crate::R<R8Spi1FifoCountSpec>;
#[doc = "Register `R8_SPI1_FIFO_COUNT` writer"]
pub type W = crate::W<R8Spi1FifoCountSpec>;
#[doc = "Field `R8_SPI1_FIFO_COUNT` reader - SPI FIFO count status"]
pub type R8Spi1FifoCountR = crate::FieldReader;
#[doc = "Field `R8_SPI1_FIFO_COUNT` writer - SPI FIFO count status"]
pub type R8Spi1FifoCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn r8_spi1_fifo_count(&self) -> R8Spi1FifoCountR {
        R8Spi1FifoCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn r8_spi1_fifo_count(&mut self) -> R8Spi1FifoCountW<'_, R8Spi1FifoCountSpec> {
        R8Spi1FifoCountW::new(self, 0)
    }
}
#[doc = "SPI1 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_fifo_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_fifo_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1FifoCountSpec;
impl crate::RegisterSpec for R8Spi1FifoCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_fifo_count::R`](R) reader structure"]
impl crate::Readable for R8Spi1FifoCountSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_fifo_count::W`](W) writer structure"]
impl crate::Writable for R8Spi1FifoCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_FIFO_COUNT to value 0"]
impl crate::Resettable for R8Spi1FifoCountSpec {}
