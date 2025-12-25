#[doc = "Register `R8_USB_INT_FG` reader"]
pub type R = crate::R<R8UsbIntFgSpec>;
#[doc = "Register `R8_USB_INT_FG` writer"]
pub type W = crate::W<R8UsbIntFgSpec>;
#[doc = "Field `RB_USB_IF_BUSRST_RB_USB_IF_DETECT` reader - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RbUsbIfBusrstRbUsbIfDetectR = crate::BitReader;
#[doc = "Field `RB_USB_IF_BUSRST_RB_USB_IF_DETECT` writer - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RbUsbIfBusrstRbUsbIfDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IF_TRANSFER` reader - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUsbIfTransferR = crate::BitReader;
#[doc = "Field `RB_USB_IF_TRANSFER` writer - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUsbIfTransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IF_SUSPEND` reader - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUsbIfSuspendR = crate::BitReader;
#[doc = "Field `RB_USB_IF_SUSPEND` writer - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RbUsbIfSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IF_HST_SOF` reader - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RbUsbIfHstSofR = crate::BitReader;
#[doc = "Field `RB_USB_IF_HST_SOF` writer - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RbUsbIfHstSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IF_FIFOOV` reader - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RbUsbIfFifoovR = crate::BitReader;
#[doc = "Field `RB_USB_IF_FIFOOV` writer - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RbUsbIfFifoovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IF_SETUOACT` reader - RO, Setup transaction end interrupt flag"]
pub type RbUsbIfSetuoactR = crate::BitReader;
#[doc = "Field `RB_USB_IF_SETUOACT` writer - RO, Setup transaction end interrupt flag"]
pub type RbUsbIfSetuoactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_IF_ISOACT` reader - RO, Synchronous transmission received control token packet interrupt flag"]
pub type RbUsbIfIsoactR = crate::BitReader;
#[doc = "Field `RB_USB_IF_ISOACT` writer - RO, Synchronous transmission received control token packet interrupt flag"]
pub type RbUsbIfIsoactW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_busrst_rb_usb_if_detect(&self) -> RbUsbIfBusrstRbUsbIfDetectR {
        RbUsbIfBusrstRbUsbIfDetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_transfer(&self) -> RbUsbIfTransferR {
        RbUsbIfTransferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_suspend(&self) -> RbUsbIfSuspendR {
        RbUsbIfSuspendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_hst_sof(&self) -> RbUsbIfHstSofR {
        RbUsbIfHstSofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_fifoov(&self) -> RbUsbIfFifoovR {
        RbUsbIfFifoovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, Setup transaction end interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_setuoact(&self) -> RbUsbIfSetuoactR {
        RbUsbIfSetuoactR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, Synchronous transmission received control token packet interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_isoact(&self) -> RbUsbIfIsoactR {
        RbUsbIfIsoactR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_busrst_rb_usb_if_detect(
        &mut self,
    ) -> RbUsbIfBusrstRbUsbIfDetectW<'_, R8UsbIntFgSpec> {
        RbUsbIfBusrstRbUsbIfDetectW::new(self, 0)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_transfer(&mut self) -> RbUsbIfTransferW<'_, R8UsbIntFgSpec> {
        RbUsbIfTransferW::new(self, 1)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_suspend(&mut self) -> RbUsbIfSuspendW<'_, R8UsbIntFgSpec> {
        RbUsbIfSuspendW::new(self, 2)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_hst_sof(&mut self) -> RbUsbIfHstSofW<'_, R8UsbIntFgSpec> {
        RbUsbIfHstSofW::new(self, 3)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_usb_if_fifoov(&mut self) -> RbUsbIfFifoovW<'_, R8UsbIntFgSpec> {
        RbUsbIfFifoovW::new(self, 4)
    }
    #[doc = "Bit 5 - RO, Setup transaction end interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_setuoact(&mut self) -> RbUsbIfSetuoactW<'_, R8UsbIntFgSpec> {
        RbUsbIfSetuoactW::new(self, 5)
    }
    #[doc = "Bit 6 - RO, Synchronous transmission received control token packet interrupt flag"]
    #[inline(always)]
    pub fn rb_usb_if_isoact(&mut self) -> RbUsbIfIsoactW<'_, R8UsbIntFgSpec> {
        RbUsbIfIsoactW::new(self, 6)
    }
}
#[doc = "USB interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_int_fg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_int_fg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbIntFgSpec;
impl crate::RegisterSpec for R8UsbIntFgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_fg::R`](R) reader structure"]
impl crate::Readable for R8UsbIntFgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_int_fg::W`](W) writer structure"]
impl crate::Writable for R8UsbIntFgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_INT_FG to value 0"]
impl crate::Resettable for R8UsbIntFgSpec {}
