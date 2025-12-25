#[doc = "Register `R8_SPI1_INTER_EN` reader"]
pub type R = crate::R<R8Spi1InterEnSpec>;
#[doc = "Register `R8_SPI1_INTER_EN` writer"]
pub type W = crate::W<R8Spi1InterEnSpec>;
#[doc = "Field `RB_SPI_IE_CNT_END` reader - enable interrupt for SPI total byte count end"]
pub type RbSpiIeCntEndR = crate::BitReader;
#[doc = "Field `RB_SPI_IE_CNT_END` writer - enable interrupt for SPI total byte count end"]
pub type RbSpiIeCntEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IE_BYTE_END` reader - enable interrupt for SPI byte exchanged"]
pub type RbSpiIeByteEndR = crate::BitReader;
#[doc = "Field `RB_SPI_IE_BYTE_END` writer - enable interrupt for SPI byte exchanged"]
pub type RbSpiIeByteEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IE_FIFO_HF` reader - enable interrupt for SPI FIFO half"]
pub type RbSpiIeFifoHfR = crate::BitReader;
#[doc = "Field `RB_SPI_IE_FIFO_HF` writer - enable interrupt for SPI FIFO half"]
pub type RbSpiIeFifoHfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IE_DMA_END` reader - enable interrupt for SPI DMA completion"]
pub type RbSpiIeDmaEndR = crate::BitReader;
#[doc = "Field `RB_SPI_IE_DMA_END` writer - enable interrupt for SPI DMA completion"]
pub type RbSpiIeDmaEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IE_FIFO_OV` reader - enable interrupt for SPI FIFO overflow"]
pub type RbSpiIeFifoOvR = crate::BitReader;
#[doc = "Field `RB_SPI_IE_FIFO_OV` writer - enable interrupt for SPI FIFO overflow"]
pub type RbSpiIeFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IE_FST_BYTE` reader - enable interrupt for SPI slave mode first byte received"]
pub type RbSpiIeFstByteR = crate::BitReader;
#[doc = "Field `RB_SPI_IE_FST_BYTE` writer - enable interrupt for SPI slave mode first byte received"]
pub type RbSpiIeFstByteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable interrupt for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_ie_cnt_end(&self) -> RbSpiIeCntEndR {
        RbSpiIeCntEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_ie_byte_end(&self) -> RbSpiIeByteEndR {
        RbSpiIeByteEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_hf(&self) -> RbSpiIeFifoHfR {
        RbSpiIeFifoHfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for SPI DMA completion"]
    #[inline(always)]
    pub fn rb_spi_ie_dma_end(&self) -> RbSpiIeDmaEndR {
        RbSpiIeDmaEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for SPI FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_ov(&self) -> RbSpiIeFifoOvR {
        RbSpiIeFifoOvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for SPI slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_ie_fst_byte(&self) -> RbSpiIeFstByteR {
        RbSpiIeFstByteR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_ie_cnt_end(&mut self) -> RbSpiIeCntEndW<'_, R8Spi1InterEnSpec> {
        RbSpiIeCntEndW::new(self, 0)
    }
    #[doc = "Bit 1 - enable interrupt for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_ie_byte_end(&mut self) -> RbSpiIeByteEndW<'_, R8Spi1InterEnSpec> {
        RbSpiIeByteEndW::new(self, 1)
    }
    #[doc = "Bit 2 - enable interrupt for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_hf(&mut self) -> RbSpiIeFifoHfW<'_, R8Spi1InterEnSpec> {
        RbSpiIeFifoHfW::new(self, 2)
    }
    #[doc = "Bit 3 - enable interrupt for SPI DMA completion"]
    #[inline(always)]
    pub fn rb_spi_ie_dma_end(&mut self) -> RbSpiIeDmaEndW<'_, R8Spi1InterEnSpec> {
        RbSpiIeDmaEndW::new(self, 3)
    }
    #[doc = "Bit 4 - enable interrupt for SPI FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_ie_fifo_ov(&mut self) -> RbSpiIeFifoOvW<'_, R8Spi1InterEnSpec> {
        RbSpiIeFifoOvW::new(self, 4)
    }
    #[doc = "Bit 7 - enable interrupt for SPI slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_ie_fst_byte(&mut self) -> RbSpiIeFstByteW<'_, R8Spi1InterEnSpec> {
        RbSpiIeFstByteW::new(self, 7)
    }
}
#[doc = "SPI1 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_inter_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_inter_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1InterEnSpec;
impl crate::RegisterSpec for R8Spi1InterEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_inter_en::R`](R) reader structure"]
impl crate::Readable for R8Spi1InterEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_inter_en::W`](W) writer structure"]
impl crate::Writable for R8Spi1InterEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_INTER_EN to value 0"]
impl crate::Resettable for R8Spi1InterEnSpec {}
