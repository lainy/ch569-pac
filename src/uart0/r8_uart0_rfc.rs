#[doc = "Register `R8_UART0_RFC` reader"]
pub type R = crate::R<R8Uart0RfcSpec>;
#[doc = "Field `R8_UART_RFC` reader - UART receiver FIFO count"]
pub type R8UartRfcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART receiver FIFO count"]
    #[inline(always)]
    pub fn r8_uart_rfc(&self) -> R8UartRfcR {
        R8UartRfcR::new(self.bits)
    }
}
#[doc = "UART0 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_rfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0RfcSpec;
impl crate::RegisterSpec for R8Uart0RfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_rfc::R`](R) reader structure"]
impl crate::Readable for R8Uart0RfcSpec {}
#[doc = "`reset()` method sets R8_UART0_RFC to value 0"]
impl crate::Resettable for R8Uart0RfcSpec {}
