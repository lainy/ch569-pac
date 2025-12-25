#[doc = "Register `R8_SLP_POWER_CTRL` reader"]
pub type R = crate::R<R8SlpPowerCtrlSpec>;
#[doc = "Register `R8_SLP_POWER_CTRL` writer"]
pub type W = crate::W<R8SlpPowerCtrlSpec>;
#[doc = "Field `RB_SLP_USBHS_PWRDN` reader - enable USBHS power down"]
pub type RbSlpUsbhsPwrdnR = crate::BitReader;
#[doc = "Field `RB_SLP_USBHS_PWRDN` writer - enable USBHS power down"]
pub type RbSlpUsbhsPwrdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable USBHS power down"]
    #[inline(always)]
    pub fn rb_slp_usbhs_pwrdn(&self) -> RbSlpUsbhsPwrdnR {
        RbSlpUsbhsPwrdnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USBHS power down"]
    #[inline(always)]
    pub fn rb_slp_usbhs_pwrdn(&mut self) -> RbSlpUsbhsPwrdnW<'_, R8SlpPowerCtrlSpec> {
        RbSlpUsbhsPwrdnW::new(self, 0)
    }
}
#[doc = "power control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpPowerCtrlSpec;
impl crate::RegisterSpec for R8SlpPowerCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_power_ctrl::R`](R) reader structure"]
impl crate::Readable for R8SlpPowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_power_ctrl::W`](W) writer structure"]
impl crate::Writable for R8SlpPowerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_POWER_CTRL to value 0"]
impl crate::Resettable for R8SlpPowerCtrlSpec {}
