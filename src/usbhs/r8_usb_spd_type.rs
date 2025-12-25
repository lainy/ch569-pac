#[doc = "Register `R8_USB_SPD_TYPE` reader"]
pub type R = crate::R<R8UsbSpdTypeSpec>;
#[doc = "Field `RB_USBSPEED_MASK` reader - USB actual speed"]
pub type RbUsbspeedMaskR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - USB actual speed"]
    #[inline(always)]
    pub fn rb_usbspeed_mask(&self) -> RbUsbspeedMaskR {
        RbUsbspeedMaskR::new(self.bits & 3)
    }
}
#[doc = "USB actual speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_spd_type::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbSpdTypeSpec;
impl crate::RegisterSpec for R8UsbSpdTypeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_spd_type::R`](R) reader structure"]
impl crate::Readable for R8UsbSpdTypeSpec {}
#[doc = "`reset()` method sets R8_USB_SPD_TYPE to value 0"]
impl crate::Resettable for R8UsbSpdTypeSpec {}
