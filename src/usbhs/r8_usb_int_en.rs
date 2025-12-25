#[doc = "Register `R8_USB_INT_EN` reader"]
pub type R = crate::R<R8UsbIntEnSpec>;
#[doc = "Register `R8_USB_INT_EN` writer"]
pub type W = crate::W<R8UsbIntEnSpec>;
#[doc = "Field `RB_USB_IE_BUSRST_RB_USB_IE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode / enable interrupt for USB device detected event for USB host mode"]
pub type RbUsbIeBusrstRbUsbIeDetectR = crate::BitReader;
#[doc = "Field `RB_USB_IE_BUSRST_RB_USB_IE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode / enable interrupt for USB device detected event for USB host mode"]
pub type RbUsbIeBusrstRbUsbIeDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_TRANS` reader - enable interrupt for USB transfer completion"]
pub type RbUsbIeTransR = crate::BitReader;
#[doc = "Field `RB_USB_IE_TRANS` writer - enable interrupt for USB transfer completion"]
pub type RbUsbIeTransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub type RbUsbIeSuspendR = crate::BitReader;
#[doc = "Field `RB_USB_IE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub type RbUsbIeSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_SOF` reader - enable interrupt for host SOF timer action for USB host mode"]
pub type RbUsbIeSofR = crate::BitReader;
#[doc = "Field `RB_USB_IE_SOF` writer - enable interrupt for host SOF timer action for USB host mode"]
pub type RbUsbIeSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_FIFOOV` reader - enable interrupt for FIFO overflow"]
pub type RbUsbIeFifoovR = crate::BitReader;
#[doc = "Field `RB_USB_IE_FIFOOV` writer - enable interrupt for FIFO overflow"]
pub type RbUsbIeFifoovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_SETUPACT` reader - Setup packet end interrupt"]
pub type RbUsbIeSetupactR = crate::BitReader;
#[doc = "Field `RB_USB_IE_SETUPACT` writer - Setup packet end interrupt"]
pub type RbUsbIeSetupactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_ISOACT` reader - Synchronous transmission received control token packet interrupt"]
pub type RbUsbIeIsoactR = crate::BitReader;
#[doc = "Field `RB_USB_IE_ISOACT` writer - Synchronous transmission received control token packet interrupt"]
pub type RbUsbIeIsoactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub type RbUsbIeDevNakR = crate::BitReader;
#[doc = "Field `RB_USB_IE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub type RbUsbIeDevNakW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode / enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_busrst_rb_usb_ie_detect(&self) -> RbUsbIeBusrstRbUsbIeDetectR {
        RbUsbIeBusrstRbUsbIeDetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_usb_ie_trans(&self) -> RbUsbIeTransR {
        RbUsbIeTransR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_usb_ie_suspend(&self) -> RbUsbIeSuspendR {
        RbUsbIeSuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_sof(&self) -> RbUsbIeSofR {
        RbUsbIeSofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_usb_ie_fifoov(&self) -> RbUsbIeFifoovR {
        RbUsbIeFifoovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup packet end interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_setupact(&self) -> RbUsbIeSetupactR {
        RbUsbIeSetupactR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronous transmission received control token packet interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_isoact(&self) -> RbUsbIeIsoactR {
        RbUsbIeIsoactR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_ie_dev_nak(&self) -> RbUsbIeDevNakR {
        RbUsbIeDevNakR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode / enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_busrst_rb_usb_ie_detect(
        &mut self,
    ) -> RbUsbIeBusrstRbUsbIeDetectW<'_, R8UsbIntEnSpec> {
        RbUsbIeBusrstRbUsbIeDetectW::new(self, 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_usb_ie_trans(&mut self) -> RbUsbIeTransW<'_, R8UsbIntEnSpec> {
        RbUsbIeTransW::new(self, 1)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_usb_ie_suspend(&mut self) -> RbUsbIeSuspendW<'_, R8UsbIntEnSpec> {
        RbUsbIeSuspendW::new(self, 2)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_sof(&mut self) -> RbUsbIeSofW<'_, R8UsbIntEnSpec> {
        RbUsbIeSofW::new(self, 3)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_usb_ie_fifoov(&mut self) -> RbUsbIeFifoovW<'_, R8UsbIntEnSpec> {
        RbUsbIeFifoovW::new(self, 4)
    }
    #[doc = "Bit 5 - Setup packet end interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_setupact(&mut self) -> RbUsbIeSetupactW<'_, R8UsbIntEnSpec> {
        RbUsbIeSetupactW::new(self, 5)
    }
    #[doc = "Bit 6 - Synchronous transmission received control token packet interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_isoact(&mut self) -> RbUsbIeIsoactW<'_, R8UsbIntEnSpec> {
        RbUsbIeIsoactW::new(self, 6)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_ie_dev_nak(&mut self) -> RbUsbIeDevNakW<'_, R8UsbIntEnSpec> {
        RbUsbIeDevNakW::new(self, 7)
    }
}
#[doc = "USB interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbIntEnSpec;
impl crate::RegisterSpec for R8UsbIntEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_en::R`](R) reader structure"]
impl crate::Readable for R8UsbIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_int_en::W`](W) writer structure"]
impl crate::Writable for R8UsbIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_INT_EN to value 0"]
impl crate::Resettable for R8UsbIntEnSpec {}
