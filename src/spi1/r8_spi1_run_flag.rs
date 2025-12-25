#[doc = "Register `R8_SPI1_RUN_FLAG` reader"]
pub type R = crate::R<R8Spi1RunFlagSpec>;
#[doc = "Register `R8_SPI1_RUN_FLAG` writer"]
pub type W = crate::W<R8Spi1RunFlagSpec>;
#[doc = "Field `RB_SPI_SLV_CMD_ACT` reader - SPI slave command flag"]
pub type RbSpiSlvCmdActR = crate::BitReader;
#[doc = "Field `RB_SPI_SLV_CMD_ACT` writer - SPI slave command flag"]
pub type RbSpiSlvCmdActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_FIFO_READY` reader - SPI FIFO ready status"]
pub type RbSpiFifoReadyR = crate::BitReader;
#[doc = "Field `RB_SPI_FIFO_READY` writer - SPI FIFO ready status"]
pub type RbSpiFifoReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_SLV_CS_LOAD` reader - SPI slave chip-select loading status"]
pub type RbSpiSlvCsLoadR = crate::BitReader;
#[doc = "Field `RB_SPI_SLV_CS_LOAD` writer - SPI slave chip-select loading status"]
pub type RbSpiSlvCsLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_SLV_SELECT` reader - SPI slave selection status"]
pub type RbSpiSlvSelectR = crate::BitReader;
#[doc = "Field `RB_SPI_SLV_SELECT` writer - SPI slave selection status"]
pub type RbSpiSlvSelectW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 4 - SPI slave command flag"]
    #[inline(always)]
    pub fn rb_spi_slv_cmd_act(&mut self) -> RbSpiSlvCmdActW<'_, R8Spi1RunFlagSpec> {
        RbSpiSlvCmdActW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI FIFO ready status"]
    #[inline(always)]
    pub fn rb_spi_fifo_ready(&mut self) -> RbSpiFifoReadyW<'_, R8Spi1RunFlagSpec> {
        RbSpiFifoReadyW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI slave chip-select loading status"]
    #[inline(always)]
    pub fn rb_spi_slv_cs_load(&mut self) -> RbSpiSlvCsLoadW<'_, R8Spi1RunFlagSpec> {
        RbSpiSlvCsLoadW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI slave selection status"]
    #[inline(always)]
    pub fn rb_spi_slv_select(&mut self) -> RbSpiSlvSelectW<'_, R8Spi1RunFlagSpec> {
        RbSpiSlvSelectW::new(self, 7)
    }
}
#[doc = "SPI1 work flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_run_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_run_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1RunFlagSpec;
impl crate::RegisterSpec for R8Spi1RunFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_run_flag::R`](R) reader structure"]
impl crate::Readable for R8Spi1RunFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_run_flag::W`](W) writer structure"]
impl crate::Writable for R8Spi1RunFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_RUN_FLAG to value 0"]
impl crate::Resettable for R8Spi1RunFlagSpec {}
