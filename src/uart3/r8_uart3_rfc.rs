#[doc = "Register `R8_UART3_RFC` reader"]
pub type R = crate::R<R8Uart3RfcSpec>;
#[doc = "Field `R8_UART3_RFC` reader - UART receiver FIFO count"]
pub type R8Uart3RfcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART receiver FIFO count"]
    #[inline(always)]
    pub fn r8_uart3_rfc(&self) -> R8Uart3RfcR {
        R8Uart3RfcR::new(self.bits)
    }
}
#[doc = "UART3 receiver FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_rfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart3RfcSpec;
impl crate::RegisterSpec for R8Uart3RfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart3_rfc::R`](R) reader structure"]
impl crate::Readable for R8Uart3RfcSpec {}
#[doc = "`reset()` method sets R8_UART3_RFC to value 0"]
impl crate::Resettable for R8Uart3RfcSpec {}
