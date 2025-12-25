#[doc = "Register `R8_UART0_TFC` reader"]
pub type R = crate::R<R8Uart0TfcSpec>;
#[doc = "Field `R8_UART0_TFC` reader - UART transmitter FIFO count"]
pub type R8Uart0TfcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART transmitter FIFO count"]
    #[inline(always)]
    pub fn r8_uart0_tfc(&self) -> R8Uart0TfcR {
        R8Uart0TfcR::new(self.bits)
    }
}
#[doc = "UART0 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_tfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0TfcSpec;
impl crate::RegisterSpec for R8Uart0TfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_tfc::R`](R) reader structure"]
impl crate::Readable for R8Uart0TfcSpec {}
#[doc = "`reset()` method sets R8_UART0_TFC to value 0"]
impl crate::Resettable for R8Uart0TfcSpec {}
