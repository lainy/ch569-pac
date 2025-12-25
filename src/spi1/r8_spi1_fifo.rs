#[doc = "Register `R8_SPI1_FIFO` reader"]
pub type R = crate::R<R8Spi1FifoSpec>;
#[doc = "Register `R8_SPI1_FIFO` writer"]
pub type W = crate::W<R8Spi1FifoSpec>;
#[doc = "Field `R8_SPI1_FIFO` reader - SPI FIFO register"]
pub type R8Spi1FifoR = crate::FieldReader;
#[doc = "Field `R8_SPI1_FIFO` writer - SPI FIFO register"]
pub type R8Spi1FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI FIFO register"]
    #[inline(always)]
    pub fn r8_spi1_fifo(&self) -> R8Spi1FifoR {
        R8Spi1FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO register"]
    #[inline(always)]
    pub fn r8_spi1_fifo(&mut self) -> R8Spi1FifoW<'_, R8Spi1FifoSpec> {
        R8Spi1FifoW::new(self, 0)
    }
}
#[doc = "SPI1 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1FifoSpec;
impl crate::RegisterSpec for R8Spi1FifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_fifo::R`](R) reader structure"]
impl crate::Readable for R8Spi1FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_fifo::W`](W) writer structure"]
impl crate::Writable for R8Spi1FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_FIFO to value 0"]
impl crate::Resettable for R8Spi1FifoSpec {}
