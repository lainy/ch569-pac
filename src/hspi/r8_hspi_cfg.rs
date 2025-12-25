#[doc = "Register `R8_HSPI_CFG` reader"]
pub type R = crate::R<R8HspiCfgSpec>;
#[doc = "Register `R8_HSPI_CFG` writer"]
pub type W = crate::W<R8HspiCfgSpec>;
#[doc = "Field `RB_HSPI_MODE` reader - parallel if mode"]
pub type RbHspiModeR = crate::BitReader;
#[doc = "Field `RB_HSPI_MODE` writer - parallel if mode"]
pub type RbHspiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_DUALDMA` reader - parallel if dualdma mode enable"]
pub type RbHspiDualdmaR = crate::BitReader;
#[doc = "Field `RB_HSPI_DUALDMA` writer - parallel if dualdma mode enable"]
pub type RbHspiDualdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_MSK_SIZE` reader - parallel if data mode"]
pub type RbHspiMskSizeR = crate::FieldReader;
#[doc = "Field `RB_HSPI_MSK_SIZE` writer - parallel if data mode"]
pub type RbHspiMskSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_HSPI_TX_TOG_EN` reader - parallel if tx addr toggle enable"]
pub type RbHspiTxTogEnR = crate::BitReader;
#[doc = "Field `RB_HSPI_TX_TOG_EN` writer - parallel if tx addr toggle enable"]
pub type RbHspiTxTogEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_RX_TOG_EN` reader - parallel if rx addr toggle enable"]
pub type RbHspiRxTogEnR = crate::BitReader;
#[doc = "Field `RB_HSPI_RX_TOG_EN` writer - parallel if rx addr toggle enable"]
pub type RbHspiRxTogEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_HW_ACK` reader - parallel if tx ack by hardware"]
pub type RbHspiHwAckR = crate::BitReader;
#[doc = "Field `RB_HSPI_HW_ACK` writer - parallel if tx ack by hardware"]
pub type RbHspiHwAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - parallel if mode"]
    #[inline(always)]
    pub fn rb_hspi_mode(&self) -> RbHspiModeR {
        RbHspiModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if dualdma mode enable"]
    #[inline(always)]
    pub fn rb_hspi_dualdma(&self) -> RbHspiDualdmaR {
        RbHspiDualdmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - parallel if data mode"]
    #[inline(always)]
    pub fn rb_hspi_msk_size(&self) -> RbHspiMskSizeR {
        RbHspiMskSizeR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 5 - parallel if tx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog_en(&self) -> RbHspiTxTogEnR {
        RbHspiTxTogEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - parallel if rx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog_en(&self) -> RbHspiRxTogEnR {
        RbHspiRxTogEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - parallel if tx ack by hardware"]
    #[inline(always)]
    pub fn rb_hspi_hw_ack(&self) -> RbHspiHwAckR {
        RbHspiHwAckR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if mode"]
    #[inline(always)]
    pub fn rb_hspi_mode(&mut self) -> RbHspiModeW<'_, R8HspiCfgSpec> {
        RbHspiModeW::new(self, 0)
    }
    #[doc = "Bit 1 - parallel if dualdma mode enable"]
    #[inline(always)]
    pub fn rb_hspi_dualdma(&mut self) -> RbHspiDualdmaW<'_, R8HspiCfgSpec> {
        RbHspiDualdmaW::new(self, 1)
    }
    #[doc = "Bits 2:3 - parallel if data mode"]
    #[inline(always)]
    pub fn rb_hspi_msk_size(&mut self) -> RbHspiMskSizeW<'_, R8HspiCfgSpec> {
        RbHspiMskSizeW::new(self, 2)
    }
    #[doc = "Bit 5 - parallel if tx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog_en(&mut self) -> RbHspiTxTogEnW<'_, R8HspiCfgSpec> {
        RbHspiTxTogEnW::new(self, 5)
    }
    #[doc = "Bit 6 - parallel if rx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog_en(&mut self) -> RbHspiRxTogEnW<'_, R8HspiCfgSpec> {
        RbHspiRxTogEnW::new(self, 6)
    }
    #[doc = "Bit 7 - parallel if tx ack by hardware"]
    #[inline(always)]
    pub fn rb_hspi_hw_ack(&mut self) -> RbHspiHwAckW<'_, R8HspiCfgSpec> {
        RbHspiHwAckW::new(self, 7)
    }
}
#[doc = "parallel if tx/rx cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiCfgSpec;
impl crate::RegisterSpec for R8HspiCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_cfg::R`](R) reader structure"]
impl crate::Readable for R8HspiCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_cfg::W`](W) writer structure"]
impl crate::Writable for R8HspiCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_CFG to value 0x82"]
impl crate::Resettable for R8HspiCfgSpec {
    const RESET_VALUE: u8 = 0x82;
}
