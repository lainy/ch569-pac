#[doc = "Register `R8_UART2_TFC` reader"]
pub type R = crate::R<R8Uart2TfcSpec>;
#[doc = "Field `R8_UART2_TFC` reader - UART transmitter FIFO count"]
pub type R8Uart2TfcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UART transmitter FIFO count"]
    #[inline(always)]
    pub fn r8_uart2_tfc(&self) -> R8Uart2TfcR {
        R8Uart2TfcR::new(self.bits)
    }
}
#[doc = "UART2 transmitter FIFO count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_tfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart2TfcSpec;
impl crate::RegisterSpec for R8Uart2TfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart2_tfc::R`](R) reader structure"]
impl crate::Readable for R8Uart2TfcSpec {}
#[doc = "`reset()` method sets R8_UART2_TFC to value 0"]
impl crate::Resettable for R8Uart2TfcSpec {}
