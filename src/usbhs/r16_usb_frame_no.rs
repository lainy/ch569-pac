#[doc = "Register `R16_USB_FRAME_NO` reader"]
pub type R = crate::R<R16UsbFrameNoSpec>;
#[doc = "Field `USB_FRAME_NO` reader - USB frame number"]
pub type UsbFrameNoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - USB frame number"]
    #[inline(always)]
    pub fn usb_frame_no(&self) -> UsbFrameNoR {
        UsbFrameNoR::new(self.bits)
    }
}
#[doc = "USB frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_usb_frame_no::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16UsbFrameNoSpec;
impl crate::RegisterSpec for R16UsbFrameNoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_usb_frame_no::R`](R) reader structure"]
impl crate::Readable for R16UsbFrameNoSpec {}
#[doc = "`reset()` method sets R16_USB_FRAME_NO to value 0"]
impl crate::Resettable for R16UsbFrameNoSpec {}
