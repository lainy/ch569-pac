#[doc = "Register `R8_USB_CTRL` reader"]
pub type R = crate::R<R8UsbCtrlSpec>;
#[doc = "Register `R8_USB_CTRL` writer"]
pub type W = crate::W<R8UsbCtrlSpec>;
#[doc = "Field `RB_USB_DMA_EN` reader - DMA enable and DMA interrupt enable for USB"]
pub type RbUsbDmaEnR = crate::BitReader;
#[doc = "Field `RB_USB_DMA_EN` writer - DMA enable and DMA interrupt enable for USB"]
pub type RbUsbDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_CLR_ALL` reader - force clear FIFO and count of USB"]
pub type RbUsbClrAllR = crate::BitReader;
#[doc = "Field `RB_USB_CLR_ALL` writer - force clear FIFO and count of USB"]
pub type RbUsbClrAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_RESET_SIE` reader - force reset USB SIE, need software clear"]
pub type RbUsbResetSieR = crate::BitReader;
#[doc = "Field `RB_USB_RESET_SIE` writer - force reset USB SIE, need software clear"]
pub type RbUsbResetSieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_INT_BUSY` reader - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type RbUsbIntBusyR = crate::BitReader;
#[doc = "Field `RB_USB_INT_BUSY` writer - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type RbUsbIntBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DEV_PU_EN` reader - USB device enable and internal pullup resistance enable"]
pub type RbDevPuEnR = crate::BitReader;
#[doc = "Field `RB_DEV_PU_EN` writer - USB device enable and internal pullup resistance enable"]
pub type RbDevPuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_USB_SPTP_MASK` reader - enable USB low speed"]
pub type RbUsbSptpMaskR = crate::FieldReader;
#[doc = "Field `RB_USB_SPTP_MASK` writer - enable USB low speed"]
pub type RbUsbSptpMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_USB_MODE` reader - enable USB host mode: 0=device mode, 1=host mode"]
pub type RbUsbModeR = crate::BitReader;
#[doc = "Field `RB_USB_MODE` writer - enable USB host mode: 0=device mode, 1=host mode"]
pub type RbUsbModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_usb_dma_en(&self) -> RbUsbDmaEnR {
        RbUsbDmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_usb_clr_all(&self) -> RbUsbClrAllR {
        RbUsbClrAllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_usb_reset_sie(&self) -> RbUsbResetSieR {
        RbUsbResetSieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_usb_int_busy(&self) -> RbUsbIntBusyR {
        RbUsbIntBusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_dev_pu_en(&self) -> RbDevPuEnR {
        RbDevPuEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - enable USB low speed"]
    #[inline(always)]
    pub fn rb_usb_sptp_mask(&self) -> RbUsbSptpMaskR {
        RbUsbSptpMaskR::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_usb_mode(&self) -> RbUsbModeR {
        RbUsbModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_usb_dma_en(&mut self) -> RbUsbDmaEnW<'_, R8UsbCtrlSpec> {
        RbUsbDmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_usb_clr_all(&mut self) -> RbUsbClrAllW<'_, R8UsbCtrlSpec> {
        RbUsbClrAllW::new(self, 1)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_usb_reset_sie(&mut self) -> RbUsbResetSieW<'_, R8UsbCtrlSpec> {
        RbUsbResetSieW::new(self, 2)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_usb_int_busy(&mut self) -> RbUsbIntBusyW<'_, R8UsbCtrlSpec> {
        RbUsbIntBusyW::new(self, 3)
    }
    #[doc = "Bit 4 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_dev_pu_en(&mut self) -> RbDevPuEnW<'_, R8UsbCtrlSpec> {
        RbDevPuEnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - enable USB low speed"]
    #[inline(always)]
    pub fn rb_usb_sptp_mask(&mut self) -> RbUsbSptpMaskW<'_, R8UsbCtrlSpec> {
        RbUsbSptpMaskW::new(self, 5)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_usb_mode(&mut self) -> RbUsbModeW<'_, R8UsbCtrlSpec> {
        RbUsbModeW::new(self, 7)
    }
}
#[doc = "USB base control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbCtrlSpec;
impl crate::RegisterSpec for R8UsbCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_ctrl::R`](R) reader structure"]
impl crate::Readable for R8UsbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_ctrl::W`](W) writer structure"]
impl crate::Writable for R8UsbCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_CTRL to value 0x06"]
impl crate::Resettable for R8UsbCtrlSpec {
    const RESET_VALUE: u8 = 0x06;
}
