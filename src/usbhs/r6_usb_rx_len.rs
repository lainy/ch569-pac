#[doc = "Register `R6_USB_RX_LEN` reader"]
pub type R = crate::R<R6UsbRxLenSpec>;
#[doc = "Field `USB_RX_LEN` reader - length of received bytes"]
pub type UsbRxLenR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - length of received bytes"]
    #[inline(always)]
    pub fn usb_rx_len(&self) -> UsbRxLenR {
        UsbRxLenR::new(self.bits)
    }
}
#[doc = "USB receiving length\n\nYou can [`read`](crate::Reg::read) this register and get [`r6_usb_rx_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R6UsbRxLenSpec;
impl crate::RegisterSpec for R6UsbRxLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r6_usb_rx_len::R`](R) reader structure"]
impl crate::Readable for R6UsbRxLenSpec {}
#[doc = "`reset()` method sets R6_USB_RX_LEN to value 0"]
impl crate::Resettable for R6UsbRxLenSpec {}
