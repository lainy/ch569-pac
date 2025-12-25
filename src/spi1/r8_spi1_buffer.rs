#[doc = "Register `R8_SPI1_BUFFER` reader"]
pub type R = crate::R<R8Spi1BufferSpec>;
#[doc = "Register `R8_SPI1_BUFFER` writer"]
pub type W = crate::W<R8Spi1BufferSpec>;
#[doc = "Field `R8_SPI1_BUFFER` reader - SPI data buffer"]
pub type R8Spi1BufferR = crate::FieldReader;
#[doc = "Field `R8_SPI1_BUFFER` writer - SPI data buffer"]
pub type R8Spi1BufferW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn r8_spi1_buffer(&self) -> R8Spi1BufferR {
        R8Spi1BufferR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn r8_spi1_buffer(&mut self) -> R8Spi1BufferW<'_, R8Spi1BufferSpec> {
        R8Spi1BufferW::new(self, 0)
    }
}
#[doc = "SPI1 data buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_buffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_buffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1BufferSpec;
impl crate::RegisterSpec for R8Spi1BufferSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_buffer::R`](R) reader structure"]
impl crate::Readable for R8Spi1BufferSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_buffer::W`](W) writer structure"]
impl crate::Writable for R8Spi1BufferSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_BUFFER to value 0"]
impl crate::Resettable for R8Spi1BufferSpec {}
