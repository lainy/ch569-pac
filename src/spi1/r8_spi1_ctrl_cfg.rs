#[doc = "Register `R8_SPI1_CTRL_CFG` reader"]
pub type R = crate::R<R8Spi1CtrlCfgSpec>;
#[doc = "Register `R8_SPI1_CTRL_CFG` writer"]
pub type W = crate::W<R8Spi1CtrlCfgSpec>;
#[doc = "Field `RB_SPI_DMA_ENABLE` reader - SPI DMA enable"]
pub type RbSpiDmaEnableR = crate::BitReader;
#[doc = "Field `RB_SPI_DMA_ENABLE` writer - SPI DMA enable"]
pub type RbSpiDmaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_DMA_LOOP` reader - SPI DMA address loop enable"]
pub type RbSpiDmaLoopR = crate::BitReader;
#[doc = "Field `RB_SPI_DMA_LOOP` writer - SPI DMA address loop enable"]
pub type RbSpiDmaLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_AUTO_IF` reader - enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
pub type RbSpiAutoIfR = crate::BitReader;
#[doc = "Field `RB_SPI_AUTO_IF` writer - enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
pub type RbSpiAutoIfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_BIT_ORDER` reader - SPI bit data order"]
pub type RbSpiBitOrderR = crate::BitReader;
#[doc = "Field `RB_SPI_BIT_ORDER` writer - SPI bit data order"]
pub type RbSpiBitOrderW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI DMA enable"]
    #[inline(always)]
    pub fn rb_spi_dma_enable(&self) -> RbSpiDmaEnableR {
        RbSpiDmaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SPI DMA address loop enable"]
    #[inline(always)]
    pub fn rb_spi_dma_loop(&self) -> RbSpiDmaLoopR {
        RbSpiDmaLoopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
    #[inline(always)]
    pub fn rb_spi_auto_if(&self) -> RbSpiAutoIfR {
        RbSpiAutoIfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI bit data order"]
    #[inline(always)]
    pub fn rb_spi_bit_order(&self) -> RbSpiBitOrderR {
        RbSpiBitOrderR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI DMA enable"]
    #[inline(always)]
    pub fn rb_spi_dma_enable(&mut self) -> RbSpiDmaEnableW<'_, R8Spi1CtrlCfgSpec> {
        RbSpiDmaEnableW::new(self, 0)
    }
    #[doc = "Bit 2 - SPI DMA address loop enable"]
    #[inline(always)]
    pub fn rb_spi_dma_loop(&mut self) -> RbSpiDmaLoopW<'_, R8Spi1CtrlCfgSpec> {
        RbSpiDmaLoopW::new(self, 2)
    }
    #[doc = "Bit 4 - enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
    #[inline(always)]
    pub fn rb_spi_auto_if(&mut self) -> RbSpiAutoIfW<'_, R8Spi1CtrlCfgSpec> {
        RbSpiAutoIfW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI bit data order"]
    #[inline(always)]
    pub fn rb_spi_bit_order(&mut self) -> RbSpiBitOrderW<'_, R8Spi1CtrlCfgSpec> {
        RbSpiBitOrderW::new(self, 5)
    }
}
#[doc = "SPI1 configuration control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_ctrl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_ctrl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1CtrlCfgSpec;
impl crate::RegisterSpec for R8Spi1CtrlCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_ctrl_cfg::R`](R) reader structure"]
impl crate::Readable for R8Spi1CtrlCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_ctrl_cfg::W`](W) writer structure"]
impl crate::Writable for R8Spi1CtrlCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_CTRL_CFG to value 0"]
impl crate::Resettable for R8Spi1CtrlCfgSpec {}
