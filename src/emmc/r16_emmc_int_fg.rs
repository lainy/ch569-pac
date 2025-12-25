#[doc = "Register `R16_EMMC_INT_FG` reader"]
pub type R = crate::R<R16EmmcIntFgSpec>;
#[doc = "Register `R16_EMMC_INT_FG` writer"]
pub type W = crate::W<R16EmmcIntFgSpec>;
#[doc = "Field `RB_EMMC_IF_RE_TMOUT` reader - indicate when expect the response, timeout"]
pub type RbEmmcIfReTmoutR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_RE_TMOUT` writer - indicate when expect the response, timeout"]
pub type RbEmmcIfReTmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_RECRC_WR` reader - indicate CRC error of the response"]
pub type RbEmmcIfRecrcWrR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_RECRC_WR` writer - indicate CRC error of the response"]
pub type RbEmmcIfRecrcWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_REIDX_ER` reader - indicate INDEX error of the response"]
pub type RbEmmcIfReidxErR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_REIDX_ER` writer - indicate INDEX error of the response"]
pub type RbEmmcIfReidxErW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_CMDDONE` reader - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
pub type RbEmmcIfCmddoneR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_CMDDONE` writer - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
pub type RbEmmcIfCmddoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_DATTMO` reader - data line busy timeout"]
pub type RbEmmcIfDattmoR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_DATTMO` writer - data line busy timeout"]
pub type RbEmmcIfDattmoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_TRANERR` reader - last block have encountered a CRC error"]
pub type RbEmmcIfTranerrR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_TRANERR` writer - last block have encountered a CRC error"]
pub type RbEmmcIfTranerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_TRANDONE` reader - all the blocks have been tran/recv successfully"]
pub type RbEmmcIfTrandoneR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_TRANDONE` writer - all the blocks have been tran/recv successfully"]
pub type RbEmmcIfTrandoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_BKGAP` reader - every block gap interrupt when multiple read/write, allow drive change the DMA address at this moment"]
pub type RbEmmcIfBkgapR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_BKGAP` writer - every block gap interrupt when multiple read/write, allow drive change the DMA address at this moment"]
pub type RbEmmcIfBkgapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_FIFO_OV` reader - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
pub type RbEmmcIfFifoOvR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_FIFO_OV` writer - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
pub type RbEmmcIfFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_IF_SDIOINT` reader - interrupt from SDIO card inside"]
pub type RbEmmcIfSdiointR = crate::BitReader;
#[doc = "Field `RB_EMMC_IF_SDIOINT` writer - interrupt from SDIO card inside"]
pub type RbEmmcIfSdiointW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - indicate when expect the response, timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_re_tmout(&self) -> RbEmmcIfReTmoutR {
        RbEmmcIfReTmoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - indicate CRC error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_recrc_wr(&self) -> RbEmmcIfRecrcWrR {
        RbEmmcIfRecrcWrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - indicate INDEX error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_reidx_er(&self) -> RbEmmcIfReidxErR {
        RbEmmcIfReidxErR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
    #[inline(always)]
    pub fn rb_emmc_if_cmddone(&self) -> RbEmmcIfCmddoneR {
        RbEmmcIfCmddoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data line busy timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_dattmo(&self) -> RbEmmcIfDattmoR {
        RbEmmcIfDattmoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - last block have encountered a CRC error"]
    #[inline(always)]
    pub fn rb_emmc_if_tranerr(&self) -> RbEmmcIfTranerrR {
        RbEmmcIfTranerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - all the blocks have been tran/recv successfully"]
    #[inline(always)]
    pub fn rb_emmc_if_trandone(&self) -> RbEmmcIfTrandoneR {
        RbEmmcIfTrandoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - every block gap interrupt when multiple read/write, allow drive change the DMA address at this moment"]
    #[inline(always)]
    pub fn rb_emmc_if_bkgap(&self) -> RbEmmcIfBkgapR {
        RbEmmcIfBkgapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
    #[inline(always)]
    pub fn rb_emmc_if_fifo_ov(&self) -> RbEmmcIfFifoOvR {
        RbEmmcIfFifoOvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt from SDIO card inside"]
    #[inline(always)]
    pub fn rb_emmc_if_sdioint(&self) -> RbEmmcIfSdiointR {
        RbEmmcIfSdiointR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - indicate when expect the response, timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_re_tmout(&mut self) -> RbEmmcIfReTmoutW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfReTmoutW::new(self, 0)
    }
    #[doc = "Bit 1 - indicate CRC error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_recrc_wr(&mut self) -> RbEmmcIfRecrcWrW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfRecrcWrW::new(self, 1)
    }
    #[doc = "Bit 2 - indicate INDEX error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_reidx_er(&mut self) -> RbEmmcIfReidxErW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfReidxErW::new(self, 2)
    }
    #[doc = "Bit 3 - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
    #[inline(always)]
    pub fn rb_emmc_if_cmddone(&mut self) -> RbEmmcIfCmddoneW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfCmddoneW::new(self, 3)
    }
    #[doc = "Bit 4 - data line busy timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_dattmo(&mut self) -> RbEmmcIfDattmoW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfDattmoW::new(self, 4)
    }
    #[doc = "Bit 5 - last block have encountered a CRC error"]
    #[inline(always)]
    pub fn rb_emmc_if_tranerr(&mut self) -> RbEmmcIfTranerrW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfTranerrW::new(self, 5)
    }
    #[doc = "Bit 6 - all the blocks have been tran/recv successfully"]
    #[inline(always)]
    pub fn rb_emmc_if_trandone(&mut self) -> RbEmmcIfTrandoneW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfTrandoneW::new(self, 6)
    }
    #[doc = "Bit 7 - every block gap interrupt when multiple read/write, allow drive change the DMA address at this moment"]
    #[inline(always)]
    pub fn rb_emmc_if_bkgap(&mut self) -> RbEmmcIfBkgapW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfBkgapW::new(self, 7)
    }
    #[doc = "Bit 8 - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
    #[inline(always)]
    pub fn rb_emmc_if_fifo_ov(&mut self) -> RbEmmcIfFifoOvW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfFifoOvW::new(self, 8)
    }
    #[doc = "Bit 9 - interrupt from SDIO card inside"]
    #[inline(always)]
    pub fn rb_emmc_if_sdioint(&mut self) -> RbEmmcIfSdiointW<'_, R16EmmcIntFgSpec> {
        RbEmmcIfSdiointW::new(self, 9)
    }
}
#[doc = "SD 16bits interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_int_fg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_int_fg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16EmmcIntFgSpec;
impl crate::RegisterSpec for R16EmmcIntFgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_emmc_int_fg::R`](R) reader structure"]
impl crate::Readable for R16EmmcIntFgSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_emmc_int_fg::W`](W) writer structure"]
impl crate::Writable for R16EmmcIntFgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_EMMC_INT_FG to value 0"]
impl crate::Resettable for R16EmmcIntFgSpec {}
