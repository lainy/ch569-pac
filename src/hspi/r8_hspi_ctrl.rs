#[doc = "Register `R8_HSPI_CTRL` reader"]
pub type R = crate::R<R8HspiCtrlSpec>;
#[doc = "Register `R8_HSPI_CTRL` writer"]
pub type W = crate::W<R8HspiCtrlSpec>;
#[doc = "Field `RB_HSPI_ENABLE` reader - parallel if enable"]
pub type RbHspiEnableR = crate::BitReader;
#[doc = "Field `RB_HSPI_ENABLE` writer - parallel if enable"]
pub type RbHspiEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_DMA_EN` reader - parallel if dma enable"]
pub type RbHspiDmaEnR = crate::BitReader;
#[doc = "Field `RB_HSPI_DMA_EN` writer - parallel if dma enable"]
pub type RbHspiDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_SW_ACT` reader - parallel if transmit software trigger"]
pub type RbHspiSwActR = crate::BitReader;
#[doc = "Field `RB_HSPI_SW_ACT` writer - parallel if transmit software trigger"]
pub type RbHspiSwActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_ALL_CLR` reader - parallel if all clear"]
pub type RbHspiAllClrR = crate::BitReader;
#[doc = "Field `RB_HSPI_ALL_CLR` writer - parallel if all clear"]
pub type RbHspiAllClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_TRX_RST` reader - parallel if tx and rx logic clear, high action"]
pub type RbHspiTrxRstR = crate::BitReader;
#[doc = "Field `RB_HSPI_TRX_RST` writer - parallel if tx and rx logic clear, high action"]
pub type RbHspiTrxRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - parallel if enable"]
    #[inline(always)]
    pub fn rb_hspi_enable(&self) -> RbHspiEnableR {
        RbHspiEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if dma enable"]
    #[inline(always)]
    pub fn rb_hspi_dma_en(&self) -> RbHspiDmaEnR {
        RbHspiDmaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - parallel if transmit software trigger"]
    #[inline(always)]
    pub fn rb_hspi_sw_act(&self) -> RbHspiSwActR {
        RbHspiSwActR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - parallel if all clear"]
    #[inline(always)]
    pub fn rb_hspi_all_clr(&self) -> RbHspiAllClrR {
        RbHspiAllClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - parallel if tx and rx logic clear, high action"]
    #[inline(always)]
    pub fn rb_hspi_trx_rst(&self) -> RbHspiTrxRstR {
        RbHspiTrxRstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if enable"]
    #[inline(always)]
    pub fn rb_hspi_enable(&mut self) -> RbHspiEnableW<'_, R8HspiCtrlSpec> {
        RbHspiEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - parallel if dma enable"]
    #[inline(always)]
    pub fn rb_hspi_dma_en(&mut self) -> RbHspiDmaEnW<'_, R8HspiCtrlSpec> {
        RbHspiDmaEnW::new(self, 1)
    }
    #[doc = "Bit 2 - parallel if transmit software trigger"]
    #[inline(always)]
    pub fn rb_hspi_sw_act(&mut self) -> RbHspiSwActW<'_, R8HspiCtrlSpec> {
        RbHspiSwActW::new(self, 2)
    }
    #[doc = "Bit 3 - parallel if all clear"]
    #[inline(always)]
    pub fn rb_hspi_all_clr(&mut self) -> RbHspiAllClrW<'_, R8HspiCtrlSpec> {
        RbHspiAllClrW::new(self, 3)
    }
    #[doc = "Bit 4 - parallel if tx and rx logic clear, high action"]
    #[inline(always)]
    pub fn rb_hspi_trx_rst(&mut self) -> RbHspiTrxRstW<'_, R8HspiCtrlSpec> {
        RbHspiTrxRstW::new(self, 4)
    }
}
#[doc = "parallel if tx/rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiCtrlSpec;
impl crate::RegisterSpec for R8HspiCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_ctrl::R`](R) reader structure"]
impl crate::Readable for R8HspiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_ctrl::W`](W) writer structure"]
impl crate::Writable for R8HspiCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_CTRL to value 0x18"]
impl crate::Resettable for R8HspiCtrlSpec {
    const RESET_VALUE: u8 = 0x18;
}
