#[doc = "Register `R8_USB_INT_ST` reader"]
pub type R = crate::R<R8UsbIntStSpec>;
#[doc = "Field `RB_HOST_RES_MASK_RB_DEV_ENDP_MASK` reader - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
pub type RbHostResMaskRbDevEndpMaskR = crate::FieldReader;
#[doc = "Field `RB_DEV_TOKEN_MASK` reader - RO, bit mask of current token PID code received for USB device mode"]
pub type RbDevTokenMaskR = crate::FieldReader;
#[doc = "Field `RB_USB_ST_TOGOK` reader - RO, indicate current USB transfer toggle is OK"]
pub type RbUsbStTogokR = crate::BitReader;
#[doc = "Field `RB_USB_ST_NAK` reader - RO, indicate current USB transfer is NAK received for USB device mode"]
pub type RbUsbStNakR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
    #[inline(always)]
    pub fn rb_host_res_mask_rb_dev_endp_mask(&self) -> RbHostResMaskRbDevEndpMaskR {
        RbHostResMaskRbDevEndpMaskR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - RO, bit mask of current token PID code received for USB device mode"]
    #[inline(always)]
    pub fn rb_dev_token_mask(&self) -> RbDevTokenMaskR {
        RbDevTokenMaskR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_usb_st_togok(&self) -> RbUsbStTogokR {
        RbUsbStTogokR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_st_nak(&self) -> RbUsbStNakR {
        RbUsbStNakR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbIntStSpec;
impl crate::RegisterSpec for R8UsbIntStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_st::R`](R) reader structure"]
impl crate::Readable for R8UsbIntStSpec {}
#[doc = "`reset()` method sets R8_USB_INT_ST to value 0"]
impl crate::Resettable for R8UsbIntStSpec {}
