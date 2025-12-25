#[doc = "Register `R8_SPI0_RUN_FLAG` reader"]
pub type R = crate::R<R8Spi0RunFlagSpec>;
#[doc = "Field `RB_SPI_SLV_CMD_ACT` reader - SPI slave command flag"]
pub type RbSpiSlvCmdActR = crate::BitReader;
#[doc = "Field `RB_SPI_FIFO_READY` reader - SPI FIFO ready status"]
pub type RbSpiFifoReadyR = crate::BitReader;
#[doc = "Field `RB_SPI_SLV_CS_LOAD` reader - SPI slave chip-select loading status"]
pub type RbSpiSlvCsLoadR = crate::BitReader;
#[doc = "Field `RB_SPI_SLV_SELECT` reader - SPI slave selection status"]
pub type RbSpiSlvSelectR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - SPI slave command flag"]
    #[inline(always)]
    pub fn rb_spi_slv_cmd_act(&self) -> RbSpiSlvCmdActR {
        RbSpiSlvCmdActR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI FIFO ready status"]
    #[inline(always)]
    pub fn rb_spi_fifo_ready(&self) -> RbSpiFifoReadyR {
        RbSpiFifoReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI slave chip-select loading status"]
    #[inline(always)]
    pub fn rb_spi_slv_cs_load(&self) -> RbSpiSlvCsLoadR {
        RbSpiSlvCsLoadR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI slave selection status"]
    #[inline(always)]
    pub fn rb_spi_slv_select(&self) -> RbSpiSlvSelectR {
        RbSpiSlvSelectR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SPI0 work flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_run_flag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0RunFlagSpec;
impl crate::RegisterSpec for R8Spi0RunFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_run_flag::R`](R) reader structure"]
impl crate::Readable for R8Spi0RunFlagSpec {}
#[doc = "`reset()` method sets R8_SPI0_RUN_FLAG to value 0"]
impl crate::Resettable for R8Spi0RunFlagSpec {}
