#[doc = "Register `R16_EMMC_INT_EN` reader"]
pub type R = crate::R<R16EmmcIntEnSpec>;
#[doc = "Register `R16_EMMC_INT_EN` writer"]
pub type W = crate::W<R16EmmcIntEnSpec>;
#[doc = "Field `RB_EMMC_IE_RE_TMOUT` reader - command response timeout interrupt enable"]
pub type RbEmmcIeReTmoutR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_RE_TMOUT` writer - command response timeout interrupt enable"]
pub type RbEmmcIeReTmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_RECRC_WR` reader - response CRC check error interrupt enable"]
pub type RbEmmcIeRecrcWrR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_RECRC_WR` writer - response CRC check error interrupt enable"]
pub type RbEmmcIeRecrcWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_REIDX_ER` reader - response index check error interrupt enable"]
pub type RbEmmcIeReidxErR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_REIDX_ER` writer - response index check error interrupt enable"]
pub type RbEmmcIeReidxErW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_CMDDONE` reader - command completion interrupt enable"]
pub type RbEmmcIeCmddoneR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_CMDDONE` writer - command completion interrupt enable"]
pub type RbEmmcIeCmddoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_DATTMO` reader - data timeout interrupt enable"]
pub type RbEmmcIeDattmoR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_DATTMO` writer - data timeout interrupt enable"]
pub type RbEmmcIeDattmoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_TRANERR` reader - blocks transfer CRC error interrupt enable"]
pub type RbEmmcIeTranerrR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_TRANERR` writer - blocks transfer CRC error interrupt enable"]
pub type RbEmmcIeTranerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_TRANDONE` reader - all blocks transfer complete interrupt enable"]
pub type RbEmmcIeTrandoneR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_TRANDONE` writer - all blocks transfer complete interrupt enable"]
pub type RbEmmcIeTrandoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_BKGAP` reader - single block transmission completion interrupt enable"]
pub type RbEmmcIeBkgapR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_BKGAP` writer - single block transmission completion interrupt enable"]
pub type RbEmmcIeBkgapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_FIFO_OV` reader - FIFO overflow interrupt enable"]
pub type RbEmmcIeFifoOvR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_FIFO_OV` writer - FIFO overflow interrupt enable"]
pub type RbEmmcIeFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IE_SDIOINT` reader - SDIO card interrupt enable"]
pub type RbEmmcIeSdiointR = crate::BitReader;
#[doc = "Field `RB_EMMC_IE_SDIOINT` writer - SDIO card interrupt enable"]
pub type RbEmmcIeSdiointW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - command response timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_re_tmout(&self) -> RbEmmcIeReTmoutR {
        RbEmmcIeReTmoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - response CRC check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_recrc_wr(&self) -> RbEmmcIeRecrcWrR {
        RbEmmcIeRecrcWrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - response index check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_reidx_er(&self) -> RbEmmcIeReidxErR {
        RbEmmcIeReidxErR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - command completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_cmddone(&self) -> RbEmmcIeCmddoneR {
        RbEmmcIeCmddoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_dattmo(&self) -> RbEmmcIeDattmoR {
        RbEmmcIeDattmoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - blocks transfer CRC error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_tranerr(&self) -> RbEmmcIeTranerrR {
        RbEmmcIeTranerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - all blocks transfer complete interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_trandone(&self) -> RbEmmcIeTrandoneR {
        RbEmmcIeTrandoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - single block transmission completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_bkgap(&self) -> RbEmmcIeBkgapR {
        RbEmmcIeBkgapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_fifo_ov(&self) -> RbEmmcIeFifoOvR {
        RbEmmcIeFifoOvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO card interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_sdioint(&self) -> RbEmmcIeSdiointR {
        RbEmmcIeSdiointR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - command response timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_re_tmout(&mut self) -> RbEmmcIeReTmoutW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeReTmoutW::new(self, 0)
    }
    #[doc = "Bit 1 - response CRC check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_recrc_wr(&mut self) -> RbEmmcIeRecrcWrW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeRecrcWrW::new(self, 1)
    }
    #[doc = "Bit 2 - response index check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_reidx_er(&mut self) -> RbEmmcIeReidxErW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeReidxErW::new(self, 2)
    }
    #[doc = "Bit 3 - command completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_cmddone(&mut self) -> RbEmmcIeCmddoneW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeCmddoneW::new(self, 3)
    }
    #[doc = "Bit 4 - data timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_dattmo(&mut self) -> RbEmmcIeDattmoW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeDattmoW::new(self, 4)
    }
    #[doc = "Bit 5 - blocks transfer CRC error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_tranerr(&mut self) -> RbEmmcIeTranerrW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeTranerrW::new(self, 5)
    }
    #[doc = "Bit 6 - all blocks transfer complete interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_trandone(&mut self) -> RbEmmcIeTrandoneW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeTrandoneW::new(self, 6)
    }
    #[doc = "Bit 7 - single block transmission completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_bkgap(&mut self) -> RbEmmcIeBkgapW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeBkgapW::new(self, 7)
    }
    #[doc = "Bit 8 - FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_fifo_ov(&mut self) -> RbEmmcIeFifoOvW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeFifoOvW::new(self, 8)
    }
    #[doc = "Bit 9 - SDIO card interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_sdioint(&mut self) -> RbEmmcIeSdiointW<'_, R16EmmcIntEnSpec> {
        RbEmmcIeSdiointW::new(self, 9)
    }
}
#[doc = "SD 16bits interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16EmmcIntEnSpec;
impl crate::RegisterSpec for R16EmmcIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_emmc_int_en::R`](R) reader structure"]
impl crate::Readable for R16EmmcIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_emmc_int_en::W`](W) writer structure"]
impl crate::Writable for R16EmmcIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_EMMC_INT_EN to value 0"]
impl crate::Resettable for R16EmmcIntEnSpec {}
