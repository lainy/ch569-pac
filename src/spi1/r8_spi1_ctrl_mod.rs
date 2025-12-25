#[doc = "Register `R8_SPI1_CTRL_MOD` reader"]
pub type R = crate::R<R8Spi1CtrlModSpec>;
#[doc = "Register `R8_SPI1_CTRL_MOD` writer"]
pub type W = crate::W<R8Spi1CtrlModSpec>;
#[doc = "Field `RB_SPI_MODE_SLAVE` reader - SPI slave mode"]
pub type RbSpiModeSlaveR = crate::BitReader;
#[doc = "Field `RB_SPI_MODE_SLAVE` writer - SPI slave mode"]
pub type RbSpiModeSlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_ALL_CLEAR` reader - force clear SPI FIFO and count"]
pub type RbSpiAllClearR = crate::BitReader;
#[doc = "Field `RB_SPI_ALL_CLEAR` writer - force clear SPI FIFO and count"]
pub type RbSpiAllClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_2WIRE_MOD` reader - SPI enable 2 wire mode"]
pub type RbSpi2wireModR = crate::BitReader;
#[doc = "Field `RB_SPI_2WIRE_MOD` writer - SPI enable 2 wire mode"]
pub type RbSpi2wireModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD` reader - SPI master clock mode / SPI slave command mode"]
pub type RbSpiMstSckModRbSpiSlvCmdModR = crate::BitReader;
#[doc = "Field `RB_SPI_MST_SCK_MOD_RB_SPI_SLV_CMD_MOD` writer - SPI master clock mode / SPI slave command mode"]
pub type RbSpiMstSckModRbSpiSlvCmdModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_FIFO_DIR` reader - SPI FIFO direction"]
pub type RbSpiFifoDirR = crate::BitReader;
#[doc = "Field `RB_SPI_FIFO_DIR` writer - SPI FIFO direction"]
pub type RbSpiFifoDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_SCK_OE` reader - SPI SCK output enable"]
pub type RbSpiSckOeR = crate::BitReader;
#[doc = "Field `RB_SPI_SCK_OE` writer - SPI SCK output enable"]
pub type RbSpiSckOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_MOSI_OE` reader - SPI MOSI output enable"]
pub type RbSpiMosiOeR = crate::BitReader;
#[doc = "Field `RB_SPI_MOSI_OE` writer - SPI MOSI output enable"]
pub type RbSpiMosiOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_MISO_OE` reader - SPI MISO output enable"]
pub type RbSpiMisoOeR = crate::BitReader;
#[doc = "Field `RB_SPI_MISO_OE` writer - SPI MISO output enable"]
pub type RbSpiMisoOeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI slave mode"]
    #[inline(always)]
    pub fn rb_spi_mode_slave(&self) -> RbSpiModeSlaveR {
        RbSpiModeSlaveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear SPI FIFO and count"]
    #[inline(always)]
    pub fn rb_spi_all_clear(&self) -> RbSpiAllClearR {
        RbSpiAllClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI enable 2 wire mode"]
    #[inline(always)]
    pub fn rb_spi_2wire_mod(&self) -> RbSpi2wireModR {
        RbSpi2wireModR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI master clock mode / SPI slave command mode"]
    #[inline(always)]
    pub fn rb_spi_mst_sck_mod_rb_spi_slv_cmd_mod(&self) -> RbSpiMstSckModRbSpiSlvCmdModR {
        RbSpiMstSckModRbSpiSlvCmdModR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI FIFO direction"]
    #[inline(always)]
    pub fn rb_spi_fifo_dir(&self) -> RbSpiFifoDirR {
        RbSpiFifoDirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI SCK output enable"]
    #[inline(always)]
    pub fn rb_spi_sck_oe(&self) -> RbSpiSckOeR {
        RbSpiSckOeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI MOSI output enable"]
    #[inline(always)]
    pub fn rb_spi_mosi_oe(&self) -> RbSpiMosiOeR {
        RbSpiMosiOeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI MISO output enable"]
    #[inline(always)]
    pub fn rb_spi_miso_oe(&self) -> RbSpiMisoOeR {
        RbSpiMisoOeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI slave mode"]
    #[inline(always)]
    pub fn rb_spi_mode_slave(&mut self) -> RbSpiModeSlaveW<'_, R8Spi1CtrlModSpec> {
        RbSpiModeSlaveW::new(self, 0)
    }
    #[doc = "Bit 1 - force clear SPI FIFO and count"]
    #[inline(always)]
    pub fn rb_spi_all_clear(&mut self) -> RbSpiAllClearW<'_, R8Spi1CtrlModSpec> {
        RbSpiAllClearW::new(self, 1)
    }
    #[doc = "Bit 2 - SPI enable 2 wire mode"]
    #[inline(always)]
    pub fn rb_spi_2wire_mod(&mut self) -> RbSpi2wireModW<'_, R8Spi1CtrlModSpec> {
        RbSpi2wireModW::new(self, 2)
    }
    #[doc = "Bit 3 - SPI master clock mode / SPI slave command mode"]
    #[inline(always)]
    pub fn rb_spi_mst_sck_mod_rb_spi_slv_cmd_mod(
        &mut self,
    ) -> RbSpiMstSckModRbSpiSlvCmdModW<'_, R8Spi1CtrlModSpec> {
        RbSpiMstSckModRbSpiSlvCmdModW::new(self, 3)
    }
    #[doc = "Bit 4 - SPI FIFO direction"]
    #[inline(always)]
    pub fn rb_spi_fifo_dir(&mut self) -> RbSpiFifoDirW<'_, R8Spi1CtrlModSpec> {
        RbSpiFifoDirW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI SCK output enable"]
    #[inline(always)]
    pub fn rb_spi_sck_oe(&mut self) -> RbSpiSckOeW<'_, R8Spi1CtrlModSpec> {
        RbSpiSckOeW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI MOSI output enable"]
    #[inline(always)]
    pub fn rb_spi_mosi_oe(&mut self) -> RbSpiMosiOeW<'_, R8Spi1CtrlModSpec> {
        RbSpiMosiOeW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI MISO output enable"]
    #[inline(always)]
    pub fn rb_spi_miso_oe(&mut self) -> RbSpiMisoOeW<'_, R8Spi1CtrlModSpec> {
        RbSpiMisoOeW::new(self, 7)
    }
}
#[doc = "SPI1 mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_ctrl_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_ctrl_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1CtrlModSpec;
impl crate::RegisterSpec for R8Spi1CtrlModSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_ctrl_mod::R`](R) reader structure"]
impl crate::Readable for R8Spi1CtrlModSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_ctrl_mod::W`](W) writer structure"]
impl crate::Writable for R8Spi1CtrlModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_CTRL_MOD to value 0x02"]
impl crate::Resettable for R8Spi1CtrlModSpec {
    const RESET_VALUE: u8 = 0x02;
}
