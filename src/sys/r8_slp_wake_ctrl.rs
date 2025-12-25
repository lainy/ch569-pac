#[doc = "Register `R8_SLP_WAKE_CTRL` reader"]
pub type R = crate::R<R8SlpWakeCtrlSpec>;
#[doc = "Register `R8_SLP_WAKE_CTRL` writer"]
pub type W = crate::W<R8SlpWakeCtrlSpec>;
#[doc = "Field `RB_SLP_USBHS_WAKE` reader - enable USBHS waking"]
pub type RbSlpUsbhsWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_USBHS_WAKE` writer - enable USBHS waking"]
pub type RbSlpUsbhsWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_USBSS_WAKE` reader - enable USBSS waking"]
pub type RbSlpUsbssWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_USBSS_WAKE` writer - enable USBSS waking"]
pub type RbSlpUsbssWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_ETH` reader - sleep ETH clock"]
pub type RbSlpClkEthR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_ETH` writer - sleep ETH clock"]
pub type RbSlpClkEthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_ECDC` reader - sleep ECDC clock"]
pub type RbSlpClkEcdcR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_ECDC` writer - sleep ECDC clock"]
pub type RbSlpClkEcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_GPIO_WAKE` reader - enable GPIO waking"]
pub type RbSlpGpioWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_GPIO_WAKE` writer - enable GPIO waking"]
pub type RbSlpGpioWakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_ETH_WAKE` reader - enable Eth waking"]
pub type RbSlpEthWakeR = crate::BitReader;
#[doc = "Field `RB_SLP_ETH_WAKE` writer - enable Eth waking"]
pub type RbSlpEthWakeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable USBHS waking"]
    #[inline(always)]
    pub fn rb_slp_usbhs_wake(&self) -> RbSlpUsbhsWakeR {
        RbSlpUsbhsWakeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable USBSS waking"]
    #[inline(always)]
    pub fn rb_slp_usbss_wake(&self) -> RbSlpUsbssWakeR {
        RbSlpUsbssWakeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep ETH clock"]
    #[inline(always)]
    pub fn rb_slp_clk_eth(&self) -> RbSlpClkEthR {
        RbSlpClkEthR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep ECDC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ecdc(&self) -> RbSlpClkEcdcR {
        RbSlpClkEcdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&self) -> RbSlpGpioWakeR {
        RbSlpGpioWakeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable Eth waking"]
    #[inline(always)]
    pub fn rb_slp_eth_wake(&self) -> RbSlpEthWakeR {
        RbSlpEthWakeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USBHS waking"]
    #[inline(always)]
    pub fn rb_slp_usbhs_wake(&mut self) -> RbSlpUsbhsWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpUsbhsWakeW::new(self, 0)
    }
    #[doc = "Bit 1 - enable USBSS waking"]
    #[inline(always)]
    pub fn rb_slp_usbss_wake(&mut self) -> RbSlpUsbssWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpUsbssWakeW::new(self, 1)
    }
    #[doc = "Bit 2 - sleep ETH clock"]
    #[inline(always)]
    pub fn rb_slp_clk_eth(&mut self) -> RbSlpClkEthW<'_, R8SlpWakeCtrlSpec> {
        RbSlpClkEthW::new(self, 2)
    }
    #[doc = "Bit 3 - sleep ECDC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ecdc(&mut self) -> RbSlpClkEcdcW<'_, R8SlpWakeCtrlSpec> {
        RbSlpClkEcdcW::new(self, 3)
    }
    #[doc = "Bit 4 - enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&mut self) -> RbSlpGpioWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpGpioWakeW::new(self, 4)
    }
    #[doc = "Bit 5 - enable Eth waking"]
    #[inline(always)]
    pub fn rb_slp_eth_wake(&mut self) -> RbSlpEthWakeW<'_, R8SlpWakeCtrlSpec> {
        RbSlpEthWakeW::new(self, 5)
    }
}
#[doc = "wake control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_wake_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_wake_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpWakeCtrlSpec;
impl crate::RegisterSpec for R8SlpWakeCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_wake_ctrl::R`](R) reader structure"]
impl crate::Readable for R8SlpWakeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_wake_ctrl::W`](W) writer structure"]
impl crate::Writable for R8SlpWakeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_WAKE_CTRL to value 0"]
impl crate::Resettable for R8SlpWakeCtrlSpec {}
