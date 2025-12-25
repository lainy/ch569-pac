#[doc = "Register `R8_SPI0_BUFFER` reader"]
pub type R = crate::R<R8Spi0BufferSpec>;
#[doc = "Register `R8_SPI0_BUFFER` writer"]
pub type W = crate::W<R8Spi0BufferSpec>;
#[doc = "Field `R8_SPI0_BUFFER` reader - SPI data buffer"]
pub type R8Spi0BufferR = crate::FieldReader;
#[doc = "Field `R8_SPI0_BUFFER` writer - SPI data buffer"]
pub type R8Spi0BufferW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn r8_spi0_buffer(&self) -> R8Spi0BufferR {
        R8Spi0BufferR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn r8_spi0_buffer(&mut self) -> R8Spi0BufferW<'_, R8Spi0BufferSpec> {
        R8Spi0BufferW::new(self, 0)
    }
}
#[doc = "SPI0 data buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_buffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_buffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0BufferSpec;
impl crate::RegisterSpec for R8Spi0BufferSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_buffer::R`](R) reader structure"]
impl crate::Readable for R8Spi0BufferSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi0_buffer::W`](W) writer structure"]
impl crate::Writable for R8Spi0BufferSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI0_BUFFER to value 0"]
impl crate::Resettable for R8Spi0BufferSpec {}
