#[doc = "Register `R16_SPI0_TOTAL_CNT` reader"]
pub type R = crate::R<R16Spi0TotalCntSpec>;
#[doc = "Register `R16_SPI0_TOTAL_CNT` writer"]
pub type W = crate::W<R16Spi0TotalCntSpec>;
#[doc = "Field `R16_SPI0_TOTAL_CNT` reader - SPI total byte count, only low 12 bit"]
pub type R16Spi0TotalCntR = crate::FieldReader<u16>;
#[doc = "Field `R16_SPI0_TOTAL_CNT` writer - SPI total byte count, only low 12 bit"]
pub type R16Spi0TotalCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SPI total byte count, only low 12 bit"]
    #[inline(always)]
    pub fn r16_spi0_total_cnt(&self) -> R16Spi0TotalCntR {
        R16Spi0TotalCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI total byte count, only low 12 bit"]
    #[inline(always)]
    pub fn r16_spi0_total_cnt(&mut self) -> R16Spi0TotalCntW<'_, R16Spi0TotalCntSpec> {
        R16Spi0TotalCntW::new(self, 0)
    }
}
#[doc = "SPI0 total byte count, only low 12 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_total_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_total_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Spi0TotalCntSpec;
impl crate::RegisterSpec for R16Spi0TotalCntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_spi0_total_cnt::R`](R) reader structure"]
impl crate::Readable for R16Spi0TotalCntSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_spi0_total_cnt::W`](W) writer structure"]
impl crate::Writable for R16Spi0TotalCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_SPI0_TOTAL_CNT to value 0"]
impl crate::Resettable for R16Spi0TotalCntSpec {}
