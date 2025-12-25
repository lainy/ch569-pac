#[doc = "Register `R32_EMMC_STATUS` reader"]
pub type R = crate::R<R32EmmcStatusSpec>;
#[doc = "Field `MASK_BLOCK_NUM` reader - the number of blocks successfully transmitted in the current multi-block transmission"]
pub type MaskBlockNumR = crate::FieldReader<u16>;
#[doc = "Field `RB_EMMC_CMDSTA` reader - indicate cmd line is high level now"]
pub type RbEmmcCmdstaR = crate::BitReader;
#[doc = "Field `RB_EMMC_DAT0STA` reader - indicate dat\\[0\\] line is high level now"]
pub type RbEmmcDat0staR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - the number of blocks successfully transmitted in the current multi-block transmission"]
    #[inline(always)]
    pub fn mask_block_num(&self) -> MaskBlockNumR {
        MaskBlockNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - indicate cmd line is high level now"]
    #[inline(always)]
    pub fn rb_emmc_cmdsta(&self) -> RbEmmcCmdstaR {
        RbEmmcCmdstaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - indicate dat\\[0\\] line is high level now"]
    #[inline(always)]
    pub fn rb_emmc_dat0sta(&self) -> RbEmmcDat0staR {
        RbEmmcDat0staR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "SD status\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcStatusSpec;
impl crate::RegisterSpec for R32EmmcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_status::R`](R) reader structure"]
impl crate::Readable for R32EmmcStatusSpec {}
#[doc = "`reset()` method sets R32_EMMC_STATUS to value 0"]
impl crate::Resettable for R32EmmcStatusSpec {}
