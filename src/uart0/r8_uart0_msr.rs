#[doc = "Register `R8_UART0_MSR` reader"]
pub type R = crate::R<R8Uart0MsrSpec>;
#[doc = "Field `RB_MSR_CTS_CHG` reader - UART0 CTS changed status, high action"]
pub type RbMsrCtsChgR = crate::BitReader;
#[doc = "Field `RB_MSR_DSR_CHG` reader - UART0 DSR changed status, high action"]
pub type RbMsrDsrChgR = crate::BitReader;
#[doc = "Field `RB_MSR_RI_CHG` reader - UART0 RI changed status, high action"]
pub type RbMsrRiChgR = crate::BitReader;
#[doc = "Field `RB_MSR_DCD_CHG` reader - UART0 DCD changed status, high action"]
pub type RbMsrDcdChgR = crate::BitReader;
#[doc = "Field `RB_MSR_CTS` reader - UART0 CTS action status"]
pub type RbMsrCtsR = crate::BitReader;
#[doc = "Field `RB_MSR_DSR` reader - UART0 DSR action status"]
pub type RbMsrDsrR = crate::BitReader;
#[doc = "Field `RB_MSR_RI` reader - UART0 RI action status"]
pub type RbMsrRiR = crate::BitReader;
#[doc = "Field `RB_MSR_DCD` reader - UART0 DCD action status"]
pub type RbMsrDcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART0 CTS changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_cts_chg(&self) -> RbMsrCtsChgR {
        RbMsrCtsChgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 DSR changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_dsr_chg(&self) -> RbMsrDsrChgR {
        RbMsrDsrChgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 RI changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_ri_chg(&self) -> RbMsrRiChgR {
        RbMsrRiChgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART0 DCD changed status, high action"]
    #[inline(always)]
    pub fn rb_msr_dcd_chg(&self) -> RbMsrDcdChgR {
        RbMsrDcdChgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 CTS action status"]
    #[inline(always)]
    pub fn rb_msr_cts(&self) -> RbMsrCtsR {
        RbMsrCtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 DSR action status"]
    #[inline(always)]
    pub fn rb_msr_dsr(&self) -> RbMsrDsrR {
        RbMsrDsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 RI action status"]
    #[inline(always)]
    pub fn rb_msr_ri(&self) -> RbMsrRiR {
        RbMsrRiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART0 DCD action status"]
    #[inline(always)]
    pub fn rb_msr_dcd(&self) -> RbMsrDcdR {
        RbMsrDcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART0 modem status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0MsrSpec;
impl crate::RegisterSpec for R8Uart0MsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_msr::R`](R) reader structure"]
impl crate::Readable for R8Uart0MsrSpec {}
#[doc = "`reset()` method sets R8_UART0_MSR to value 0"]
impl crate::Resettable for R8Uart0MsrSpec {}
