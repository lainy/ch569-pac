#[doc = "Register `R8_UART0_IIR` reader"]
pub type R = crate::R<R8Uart0IirSpec>;
#[doc = "Field `RB_IIR_NO_INT` reader - UART no interrupt flag"]
pub type RbIirNoIntR = crate::BitReader;
#[doc = "Field `RB_IIR_INT_MASK` reader - UART interrupt flag bit mask"]
pub type RbIirIntMaskR = crate::FieldReader;
#[doc = "Field `RB_IIR_FIFO_ID` reader - UART FIFO enabled flag"]
pub type RbIirFifoIdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - UART no interrupt flag"]
    #[inline(always)]
    pub fn rb_iir_no_int(&self) -> RbIirNoIntR {
        RbIirNoIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - UART interrupt flag bit mask"]
    #[inline(always)]
    pub fn rb_iir_int_mask(&self) -> RbIirIntMaskR {
        RbIirIntMaskR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 6:7 - UART FIFO enabled flag"]
    #[inline(always)]
    pub fn rb_iir_fifo_id(&self) -> RbIirFifoIdR {
        RbIirFifoIdR::new((self.bits >> 6) & 3)
    }
}
#[doc = "UART0 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0IirSpec;
impl crate::RegisterSpec for R8Uart0IirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_iir::R`](R) reader structure"]
impl crate::Readable for R8Uart0IirSpec {}
#[doc = "`reset()` method sets R8_UART0_IIR to value 0x01"]
impl crate::Resettable for R8Uart0IirSpec {
    const RESET_VALUE: u8 = 0x01;
}
