#[doc = "Register `R8_RST_BOOT_STAT` reader"]
pub type R = crate::R<R8RstBootStatSpec>;
#[doc = "Field `RB_RESET_FLAG` reader - recent reset flag"]
pub type RbResetFlagR = crate::FieldReader;
#[doc = "Field `RB_CFG_RESET_EN` reader - manual reset input enable status"]
pub type RbCfgResetEnR = crate::BitReader;
#[doc = "Field `RB_CFG_BOOT_EN` reader - boot-loader enable status"]
pub type RbCfgBootEnR = crate::BitReader;
#[doc = "Field `RB_CFG_DEBUG_EN` reader - debug enable status"]
pub type RbCfgDebugEnR = crate::BitReader;
#[doc = "Field `RB_BOOT_LOADER` reader - indicate boot loader status"]
pub type RbBootLoaderR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - recent reset flag"]
    #[inline(always)]
    pub fn rb_reset_flag(&self) -> RbResetFlagR {
        RbResetFlagR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - manual reset input enable status"]
    #[inline(always)]
    pub fn rb_cfg_reset_en(&self) -> RbCfgResetEnR {
        RbCfgResetEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - boot-loader enable status"]
    #[inline(always)]
    pub fn rb_cfg_boot_en(&self) -> RbCfgBootEnR {
        RbCfgBootEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - debug enable status"]
    #[inline(always)]
    pub fn rb_cfg_debug_en(&self) -> RbCfgDebugEnR {
        RbCfgDebugEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - indicate boot loader status"]
    #[inline(always)]
    pub fn rb_boot_loader(&self) -> RbBootLoaderR {
        RbBootLoaderR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "reset status and boot/debug status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rst_boot_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8RstBootStatSpec;
impl crate::RegisterSpec for R8RstBootStatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_rst_boot_stat::R`](R) reader structure"]
impl crate::Readable for R8RstBootStatSpec {}
#[doc = "`reset()` method sets R8_RST_BOOT_STAT to value 0xc8"]
impl crate::Resettable for R8RstBootStatSpec {
    const RESET_VALUE: u8 = 0xc8;
}
