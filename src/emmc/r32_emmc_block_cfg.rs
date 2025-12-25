#[doc = "Register `R32_EMMC_BLOCK_CFG` reader"]
pub type R = crate::R<R32EmmcBlockCfgSpec>;
#[doc = "Register `R32_EMMC_BLOCK_CFG` writer"]
pub type W = crate::W<R32EmmcBlockCfgSpec>;
#[doc = "Field `RB_EMMC_BKNUM_MASK` reader - the number of blocks to be transferred"]
pub type RbEmmcBknumMaskR = crate::FieldReader<u16>;
#[doc = "Field `RB_EMMC_BKNUM_MASK` writer - the number of blocks to be transferred"]
pub type RbEmmcBknumMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RB_EMMC_BKSIZE_MASK` reader - single block transfer size"]
pub type RbEmmcBksizeMaskR = crate::FieldReader<u16>;
#[doc = "Field `RB_EMMC_BKSIZE_MASK` writer - single block transfer size"]
pub type RbEmmcBksizeMaskW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of blocks to be transferred"]
    #[inline(always)]
    pub fn rb_emmc_bknum_mask(&self) -> RbEmmcBknumMaskR {
        RbEmmcBknumMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - single block transfer size"]
    #[inline(always)]
    pub fn rb_emmc_bksize_mask(&self) -> RbEmmcBksizeMaskR {
        RbEmmcBksizeMaskR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of blocks to be transferred"]
    #[inline(always)]
    pub fn rb_emmc_bknum_mask(&mut self) -> RbEmmcBknumMaskW<'_, R32EmmcBlockCfgSpec> {
        RbEmmcBknumMaskW::new(self, 0)
    }
    #[doc = "Bits 16:27 - single block transfer size"]
    #[inline(always)]
    pub fn rb_emmc_bksize_mask(&mut self) -> RbEmmcBksizeMaskW<'_, R32EmmcBlockCfgSpec> {
        RbEmmcBksizeMaskW::new(self, 16)
    }
}
#[doc = "SD 32bits data counter, \\[15:0\\] number of blocks this time will tran/recv, \\[27:16\\] block sise(byte number) of every block in this time tran/recv\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_block_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_block_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcBlockCfgSpec;
impl crate::RegisterSpec for R32EmmcBlockCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_block_cfg::R`](R) reader structure"]
impl crate::Readable for R32EmmcBlockCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_emmc_block_cfg::W`](W) writer structure"]
impl crate::Writable for R32EmmcBlockCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_EMMC_BLOCK_CFG to value 0"]
impl crate::Resettable for R32EmmcBlockCfgSpec {}
