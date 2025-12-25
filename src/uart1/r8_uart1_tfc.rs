#[doc = "Register `R8_UART1_TFC` reader"]
pub type R = crate::R<R8Uart1TfcSpec>;
#[doc = "Field `R8_UART1_TFC` reader - UART transmitter FIFO count"]
pub type R8Uart1TfcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART transmitter FIFO count"]
    #[inline(always)]
    pub fn r8_uart1_tfc(&self) -> R8Uart1TfcR {
        R8Uart1TfcR::new(self.bits)
    }
}
#[doc = "UART1 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_tfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1TfcSpec;
impl crate::RegisterSpec for R8Uart1TfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_tfc::R`](R) reader structure"]
impl crate::Readable for R8Uart1TfcSpec {}
#[doc = "`reset()` method sets R8_UART1_TFC to value 0"]
impl crate::Resettable for R8Uart1TfcSpec {}
