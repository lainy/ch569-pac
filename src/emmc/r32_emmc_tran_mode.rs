#[doc = "Register `R32_EMMC_TRAN_MODE` reader"]
pub type R = crate::R<R32EmmcTranModeSpec>;
#[doc = "Register `R32_EMMC_TRAN_MODE` writer"]
pub type W = crate::W<R32EmmcTranModeSpec>;
#[doc = "Field `RB_EMMC_DMA_DIR` reader - set DMA direction is controller to emmc card"]
pub type RbEmmcDmaDirR = crate::BitReader;
#[doc = "Field `RB_EMMC_DMA_DIR` writer - set DMA direction is controller to emmc card"]
pub type RbEmmcDmaDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_GAP_STOP` reader - clock stop mode after block completion"]
pub type RbEmmcGapStopR = crate::BitReader;
#[doc = "Field `RB_EMMC_GAP_STOP` writer - clock stop mode after block completion"]
pub type RbEmmcGapStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_MODE_BOOT` reader - enable emmc boot mode"]
pub type RbEmmcModeBootR = crate::BitReader;
#[doc = "Field `RB_EMMC_MODE_BOOT` writer - enable emmc boot mode"]
pub type RbEmmcModeBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_AUTOGAPSTOP` reader - enable auto set bTM_GAP_STOP when tran start"]
pub type RbEmmcAutogapstopR = crate::BitReader;
#[doc = "Field `RB_EMMC_AUTOGAPSTOP` writer - enable auto set bTM_GAP_STOP when tran start"]
pub type RbEmmcAutogapstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_FIFO_RDY` reader - FIFO ready select signal when writing EMMC"]
pub type RbEmmcFifoRdyR = crate::FieldReader;
#[doc = "Field `RB_EMMC_FIFO_RDY` writer - FIFO ready select signal when writing EMMC"]
pub type RbEmmcFifoRdyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_EMMC_DMATN_CNT` reader - in double buffer mode,set the block count value of buffer switch"]
pub type RbEmmcDmatnCntR = crate::FieldReader;
#[doc = "Field `RB_EMMC_DMATN_CNT` writer - in double buffer mode,set the block count value of buffer switch"]
pub type RbEmmcDmatnCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RB_EMMC_DULEDMA_EN` reader - enable double buffer dma"]
pub type RbEmmcDuledmaEnR = crate::BitReader;
#[doc = "Field `RB_EMMC_DULEDMA_EN` writer - enable double buffer dma"]
pub type RbEmmcDuledmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set DMA direction is controller to emmc card"]
    #[inline(always)]
    pub fn rb_emmc_dma_dir(&self) -> RbEmmcDmaDirR {
        RbEmmcDmaDirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock stop mode after block completion"]
    #[inline(always)]
    pub fn rb_emmc_gap_stop(&self) -> RbEmmcGapStopR {
        RbEmmcGapStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable emmc boot mode"]
    #[inline(always)]
    pub fn rb_emmc_mode_boot(&self) -> RbEmmcModeBootR {
        RbEmmcModeBootR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - enable auto set bTM_GAP_STOP when tran start"]
    #[inline(always)]
    pub fn rb_emmc_autogapstop(&self) -> RbEmmcAutogapstopR {
        RbEmmcAutogapstopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - FIFO ready select signal when writing EMMC"]
    #[inline(always)]
    pub fn rb_emmc_fifo_rdy(&self) -> RbEmmcFifoRdyR {
        RbEmmcFifoRdyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - in double buffer mode,set the block count value of buffer switch"]
    #[inline(always)]
    pub fn rb_emmc_dmatn_cnt(&self) -> RbEmmcDmatnCntR {
        RbEmmcDmatnCntR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - enable double buffer dma"]
    #[inline(always)]
    pub fn rb_emmc_duledma_en(&self) -> RbEmmcDuledmaEnR {
        RbEmmcDuledmaEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set DMA direction is controller to emmc card"]
    #[inline(always)]
    pub fn rb_emmc_dma_dir(&mut self) -> RbEmmcDmaDirW<'_, R32EmmcTranModeSpec> {
        RbEmmcDmaDirW::new(self, 0)
    }
    #[doc = "Bit 1 - clock stop mode after block completion"]
    #[inline(always)]
    pub fn rb_emmc_gap_stop(&mut self) -> RbEmmcGapStopW<'_, R32EmmcTranModeSpec> {
        RbEmmcGapStopW::new(self, 1)
    }
    #[doc = "Bit 2 - enable emmc boot mode"]
    #[inline(always)]
    pub fn rb_emmc_mode_boot(&mut self) -> RbEmmcModeBootW<'_, R32EmmcTranModeSpec> {
        RbEmmcModeBootW::new(self, 2)
    }
    #[doc = "Bit 4 - enable auto set bTM_GAP_STOP when tran start"]
    #[inline(always)]
    pub fn rb_emmc_autogapstop(&mut self) -> RbEmmcAutogapstopW<'_, R32EmmcTranModeSpec> {
        RbEmmcAutogapstopW::new(self, 4)
    }
    #[doc = "Bits 6:7 - FIFO ready select signal when writing EMMC"]
    #[inline(always)]
    pub fn rb_emmc_fifo_rdy(&mut self) -> RbEmmcFifoRdyW<'_, R32EmmcTranModeSpec> {
        RbEmmcFifoRdyW::new(self, 6)
    }
    #[doc = "Bits 8:14 - in double buffer mode,set the block count value of buffer switch"]
    #[inline(always)]
    pub fn rb_emmc_dmatn_cnt(&mut self) -> RbEmmcDmatnCntW<'_, R32EmmcTranModeSpec> {
        RbEmmcDmatnCntW::new(self, 8)
    }
    #[doc = "Bit 16 - enable double buffer dma"]
    #[inline(always)]
    pub fn rb_emmc_duledma_en(&mut self) -> RbEmmcDuledmaEnW<'_, R32EmmcTranModeSpec> {
        RbEmmcDuledmaEnW::new(self, 16)
    }
}
#[doc = "SD TRANSFER MODE register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_tran_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_tran_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcTranModeSpec;
impl crate::RegisterSpec for R32EmmcTranModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_tran_mode::R`](R) reader structure"]
impl crate::Readable for R32EmmcTranModeSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_emmc_tran_mode::W`](W) writer structure"]
impl crate::Writable for R32EmmcTranModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_EMMC_TRAN_MODE to value 0"]
impl crate::Resettable for R32EmmcTranModeSpec {}
