#[doc = "Register `R8_USB_MIS_ST` reader"]
pub type R = crate::R<R8UsbMisStSpec>;
#[doc = "Field `RB_USB_SPLIT_EN` reader - RO,indicate host allow SPLIT packet"]
pub type RbUsbSplitEnR = crate::BitReader;
#[doc = "Field `RB_USB_ATTACH` reader - RO, indicate device attached status on USB host"]
pub type RbUsbAttachR = crate::BitReader;
#[doc = "Field `RB_USBBUS_SUSPEND` reader - RO, indicate USB suspend status"]
pub type RbUsbbusSuspendR = crate::BitReader;
#[doc = "Field `RB_USBBUS_RESET` reader - RO, indicate USB bus reset status"]
pub type RbUsbbusResetR = crate::BitReader;
#[doc = "Field `RB_USB_FIFO_RDY` reader - RO, indicate USB receiving FIFO ready status (not empty)"]
pub type RbUsbFifoRdyR = crate::BitReader;
#[doc = "Field `RB_USB_SIE_FREE` reader - RO, indicate USB SIE free status"]
pub type RbUsbSieFreeR = crate::BitReader;
#[doc = "Field `RB_USB_SOF_ACT` reader - RO, indicate host SOF timer action status for USB host"]
pub type RbUsbSofActR = crate::BitReader;
#[doc = "Field `RB_USB_SOF_PRES` reader - RO, indicate host SOF timer presage status"]
pub type RbUsbSofPresR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RO,indicate host allow SPLIT packet"]
    #[inline(always)]
    pub fn rb_usb_split_en(&self) -> RbUsbSplitEnR {
        RbUsbSplitEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO, indicate device attached status on USB host"]
    #[inline(always)]
    pub fn rb_usb_attach(&self) -> RbUsbAttachR {
        RbUsbAttachR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RO, indicate USB suspend status"]
    #[inline(always)]
    pub fn rb_usbbus_suspend(&self) -> RbUsbbusSuspendR {
        RbUsbbusSuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RO, indicate USB bus reset status"]
    #[inline(always)]
    pub fn rb_usbbus_reset(&self) -> RbUsbbusResetR {
        RbUsbbusResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RO, indicate USB receiving FIFO ready status (not empty)"]
    #[inline(always)]
    pub fn rb_usb_fifo_rdy(&self) -> RbUsbFifoRdyR {
        RbUsbFifoRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, indicate USB SIE free status"]
    #[inline(always)]
    pub fn rb_usb_sie_free(&self) -> RbUsbSieFreeR {
        RbUsbSieFreeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, indicate host SOF timer action status for USB host"]
    #[inline(always)]
    pub fn rb_usb_sof_act(&self) -> RbUsbSofActR {
        RbUsbSofActR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate host SOF timer presage status"]
    #[inline(always)]
    pub fn rb_usb_sof_pres(&self) -> RbUsbSofPresR {
        RbUsbSofPresR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB miscellaneous status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_mis_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbMisStSpec;
impl crate::RegisterSpec for R8UsbMisStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_mis_st::R`](R) reader structure"]
impl crate::Readable for R8UsbMisStSpec {}
#[doc = "`reset()` method sets R8_USB_MIS_ST to value 0x20"]
impl crate::Resettable for R8UsbMisStSpec {
    const RESET_VALUE: u8 = 0x20;
}
