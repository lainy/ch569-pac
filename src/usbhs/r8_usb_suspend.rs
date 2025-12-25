#[doc = "Register `R8_USB_SUSPEND` reader"]
pub type R = crate::R<R8UsbSuspendSpec>;
#[doc = "Register `R8_USB_SUSPEND` writer"]
pub type W = crate::W<R8UsbSuspendSpec>;
#[doc = "Field `RB_DEV_WAKEUP` reader - Remote wake-up control bit"]
pub type RbDevWakeupR = crate::BitReader;
#[doc = "Field `RB_DEV_WAKEUP` writer - Remote wake-up control bit"]
pub type RbDevWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Remote wake-up control bit"]
    #[inline(always)]
    pub fn rb_dev_wakeup(&self) -> RbDevWakeupR {
        RbDevWakeupR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Remote wake-up control bit"]
    #[inline(always)]
    pub fn rb_dev_wakeup(&mut self) -> RbDevWakeupW<'_, R8UsbSuspendSpec> {
        RbDevWakeupW::new(self, 1)
    }
}
#[doc = "USB suspend register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_usb_suspend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_usb_suspend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UsbSuspendSpec;
impl crate::RegisterSpec for R8UsbSuspendSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_suspend::R`](R) reader structure"]
impl crate::Readable for R8UsbSuspendSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_suspend::W`](W) writer structure"]
impl crate::Writable for R8UsbSuspendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_USB_SUSPEND to value 0"]
impl crate::Resettable for R8UsbSuspendSpec {}
