#[doc = "Register `R8_EMMC_CONTROL` reader"]
pub type R = crate::R<R8EmmcControlSpec>;
#[doc = "Register `R8_EMMC_CONTROL` writer"]
pub type W = crate::W<R8EmmcControlSpec>;
#[doc = "Field `RB_EMMC_LW_MASK` reader - effctive data width for sending or receiving data"]
pub type RbEmmcLwMaskR = crate::FieldReader;
#[doc = "Field `RB_EMMC_LW_MASK` writer - effctive data width for sending or receiving data"]
pub type RbEmmcLwMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_EMMC_ALL_CLR` reader - reset all the inner logic, default is valid"]
pub type RbEmmcAllClrR = crate::BitReader;
#[doc = "Field `RB_EMMC_ALL_CLR` writer - reset all the inner logic, default is valid"]
pub type RbEmmcAllClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_DMAEN` reader - enable the dma"]
pub type RbEmmcDmaenR = crate::BitReader;
#[doc = "Field `RB_EMMC_DMAEN` writer - enable the dma"]
pub type RbEmmcDmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_RST_LGC` reader - reset the data tran/recv logic"]
pub type RbEmmcRstLgcR = crate::BitReader;
#[doc = "Field `RB_EMMC_RST_LGC` writer - reset the data tran/recv logic"]
pub type RbEmmcRstLgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EMMC_NEGSMP` reader - controller use nagedge sample cmd"]
pub type RbEmmcNegsmpR = crate::BitReader;
#[doc = "Field `RB_EMMC_NEGSMP` writer - controller use nagedge sample cmd"]
pub type RbEmmcNegsmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - effctive data width for sending or receiving data"]
    #[inline(always)]
    pub fn rb_emmc_lw_mask(&self) -> RbEmmcLwMaskR {
        RbEmmcLwMaskR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - reset all the inner logic, default is valid"]
    #[inline(always)]
    pub fn rb_emmc_all_clr(&self) -> RbEmmcAllClrR {
        RbEmmcAllClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable the dma"]
    #[inline(always)]
    pub fn rb_emmc_dmaen(&self) -> RbEmmcDmaenR {
        RbEmmcDmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reset the data tran/recv logic"]
    #[inline(always)]
    pub fn rb_emmc_rst_lgc(&self) -> RbEmmcRstLgcR {
        RbEmmcRstLgcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - controller use nagedge sample cmd"]
    #[inline(always)]
    pub fn rb_emmc_negsmp(&self) -> RbEmmcNegsmpR {
        RbEmmcNegsmpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - effctive data width for sending or receiving data"]
    #[inline(always)]
    pub fn rb_emmc_lw_mask(&mut self) -> RbEmmcLwMaskW<'_, R8EmmcControlSpec> {
        RbEmmcLwMaskW::new(self, 0)
    }
    #[doc = "Bit 2 - reset all the inner logic, default is valid"]
    #[inline(always)]
    pub fn rb_emmc_all_clr(&mut self) -> RbEmmcAllClrW<'_, R8EmmcControlSpec> {
        RbEmmcAllClrW::new(self, 2)
    }
    #[doc = "Bit 3 - enable the dma"]
    #[inline(always)]
    pub fn rb_emmc_dmaen(&mut self) -> RbEmmcDmaenW<'_, R8EmmcControlSpec> {
        RbEmmcDmaenW::new(self, 3)
    }
    #[doc = "Bit 4 - reset the data tran/recv logic"]
    #[inline(always)]
    pub fn rb_emmc_rst_lgc(&mut self) -> RbEmmcRstLgcW<'_, R8EmmcControlSpec> {
        RbEmmcRstLgcW::new(self, 4)
    }
    #[doc = "Bit 5 - controller use nagedge sample cmd"]
    #[inline(always)]
    pub fn rb_emmc_negsmp(&mut self) -> RbEmmcNegsmpW<'_, R8EmmcControlSpec> {
        RbEmmcNegsmpW::new(self, 5)
    }
}
#[doc = "SD 8bits control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_emmc_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_emmc_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8EmmcControlSpec;
impl crate::RegisterSpec for R8EmmcControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_emmc_control::R`](R) reader structure"]
impl crate::Readable for R8EmmcControlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_emmc_control::W`](W) writer structure"]
impl crate::Writable for R8EmmcControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_EMMC_CONTROL to value 0x15"]
impl crate::Resettable for R8EmmcControlSpec {
    const RESET_VALUE: u8 = 0x15;
}
