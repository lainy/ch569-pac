#[doc = "Register `R8_HSPI_RTX_STATUS` reader"]
pub type R = crate::R<R8HspiRtxStatusSpec>;
#[doc = "Register `R8_HSPI_RTX_STATUS` writer"]
pub type W = crate::W<R8HspiRtxStatusSpec>;
#[doc = "Field `RB_HSPI_CRC_ERR` reader - CRC error occur"]
pub type RbHspiCrcErrR = crate::BitReader;
#[doc = "Field `RB_HSPI_CRC_ERR` writer - CRC error occur"]
pub type RbHspiCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_NUM_MIS` reader - rx and tx sequence number mismatch"]
pub type RbHspiNumMisR = crate::BitReader;
#[doc = "Field `RB_HSPI_NUM_MIS` writer - rx and tx sequence number mismatch"]
pub type RbHspiNumMisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CRC error occur"]
    #[inline(always)]
    pub fn rb_hspi_crc_err(&self) -> RbHspiCrcErrR {
        RbHspiCrcErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx and tx sequence number mismatch"]
    #[inline(always)]
    pub fn rb_hspi_num_mis(&self) -> RbHspiNumMisR {
        RbHspiNumMisR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CRC error occur"]
    #[inline(always)]
    pub fn rb_hspi_crc_err(&mut self) -> RbHspiCrcErrW<'_, R8HspiRtxStatusSpec> {
        RbHspiCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2 - rx and tx sequence number mismatch"]
    #[inline(always)]
    pub fn rb_hspi_num_mis(&mut self) -> RbHspiNumMisW<'_, R8HspiRtxStatusSpec> {
        RbHspiNumMisW::new(self, 2)
    }
}
#[doc = "parallel rtx status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_rtx_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_rtx_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiRtxStatusSpec;
impl crate::RegisterSpec for R8HspiRtxStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_rtx_status::R`](R) reader structure"]
impl crate::Readable for R8HspiRtxStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_rtx_status::W`](W) writer structure"]
impl crate::Writable for R8HspiRtxStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_RTX_STATUS to value 0"]
impl crate::Resettable for R8HspiRtxStatusSpec {}
