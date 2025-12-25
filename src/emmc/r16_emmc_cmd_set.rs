#[doc = "Register `R16_EMMC_CMD_SET` reader"]
pub type R = crate::R<R16EmmcCmdSetSpec>;
#[doc = "Register `R16_EMMC_CMD_SET` writer"]
pub type W = crate::W<R16EmmcCmdSetSpec>;
#[doc = "Field `RB_EMMC_CMDIDX_MASK` reader - the index number of the currently sent command"]
pub type RbEmmcCmdidxMaskR = crate::FieldReader;
#[doc = "Field `RB_EMMC_CMDIDX_MASK` writer - the index number of the currently sent command"]
pub type RbEmmcCmdidxMaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RB_EMMC_RPTY_MASK` reader - current respone type"]
pub type RbEmmcRptyMaskR = crate::FieldReader;
#[doc = "Field `RB_EMMC_RPTY_MASK` writer - current respone type"]
pub type RbEmmcRptyMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_EMMC_CKCRC` reader - check the response CRC"]
pub type RbEmmcCkcrcR = crate::BitReader;
#[doc = "Field `RB_EMMC_CKCRC` writer - check the response CRC"]
pub type RbEmmcCkcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_CKIDX` reader - check the response command index"]
pub type RbEmmcCkidxR = crate::BitReader;
#[doc = "Field `RB_EMMC_CKIDX` writer - check the response command index"]
pub type RbEmmcCkidxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - the index number of the currently sent command"]
    #[inline(always)]
    pub fn rb_emmc_cmdidx_mask(&self) -> RbEmmcCmdidxMaskR {
        RbEmmcCmdidxMaskR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - current respone type"]
    #[inline(always)]
    pub fn rb_emmc_rpty_mask(&self) -> RbEmmcRptyMaskR {
        RbEmmcRptyMaskR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - check the response CRC"]
    #[inline(always)]
    pub fn rb_emmc_ckcrc(&self) -> RbEmmcCkcrcR {
        RbEmmcCkcrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - check the response command index"]
    #[inline(always)]
    pub fn rb_emmc_ckidx(&self) -> RbEmmcCkidxR {
        RbEmmcCkidxR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - the index number of the currently sent command"]
    #[inline(always)]
    pub fn rb_emmc_cmdidx_mask(&mut self) -> RbEmmcCmdidxMaskW<'_, R16EmmcCmdSetSpec> {
        RbEmmcCmdidxMaskW::new(self, 0)
    }
    #[doc = "Bits 8:9 - current respone type"]
    #[inline(always)]
    pub fn rb_emmc_rpty_mask(&mut self) -> RbEmmcRptyMaskW<'_, R16EmmcCmdSetSpec> {
        RbEmmcRptyMaskW::new(self, 8)
    }
    #[doc = "Bit 10 - check the response CRC"]
    #[inline(always)]
    pub fn rb_emmc_ckcrc(&mut self) -> RbEmmcCkcrcW<'_, R16EmmcCmdSetSpec> {
        RbEmmcCkcrcW::new(self, 10)
    }
    #[doc = "Bit 11 - check the response command index"]
    #[inline(always)]
    pub fn rb_emmc_ckidx(&mut self) -> RbEmmcCkidxW<'_, R16EmmcCmdSetSpec> {
        RbEmmcCkidxW::new(self, 11)
    }
}
#[doc = "SD 16bits cmd setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_emmc_cmd_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_emmc_cmd_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16EmmcCmdSetSpec;
impl crate::RegisterSpec for R16EmmcCmdSetSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_emmc_cmd_set::R`](R) reader structure"]
impl crate::Readable for R16EmmcCmdSetSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_emmc_cmd_set::W`](W) writer structure"]
impl crate::Writable for R16EmmcCmdSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_EMMC_CMD_SET to value 0"]
impl crate::Resettable for R16EmmcCmdSetSpec {}
