#[doc = "Register `R8_UART1_RFC` reader"]
pub type R = crate::R<R8Uart1RfcSpec>;
#[doc = "Field `R8_UART1_RFC` reader - UART receiver FIFO count"]
pub type R8Uart1RfcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART receiver FIFO count"]
    #[inline(always)]
    pub fn r8_uart1_rfc(&self) -> R8Uart1RfcR {
        R8Uart1RfcR::new(self.bits)
    }
}
#[doc = "UART1 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_rfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1RfcSpec;
impl crate::RegisterSpec for R8Uart1RfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_rfc::R`](R) reader structure"]
impl crate::Readable for R8Uart1RfcSpec {}
#[doc = "`reset()` method sets R8_UART1_RFC to value 0"]
impl crate::Resettable for R8Uart1RfcSpec {}
