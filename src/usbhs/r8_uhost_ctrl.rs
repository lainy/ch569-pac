#[doc = "Register `R8_UHOST_CTRL` reader"]
pub type R = crate::R<R8UhostCtrlSpec>;
#[doc = "Register `R8_UHOST_CTRL` writer"]
pub type W = crate::W<R8UhostCtrlSpec>;
#[doc = "Field `RB_UH_BUS_RESET` reader - USB host send bus reset signal"]
pub type RbUhBusResetR = crate::BitReader;
#[doc = "Field `RB_UH_BUS_RESET` writer - USB host send bus reset signal"]
pub type RbUhBusResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UH_BUS_SUSPEND` reader - USB host send bus suspend signal"]
pub type RbUhBusSuspendR = crate::BitReader;
#[doc = "Field `RB_UH_BUS_SUSPEND` writer - USB host send bus suspend signal"]
pub type RbUhBusSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UH_BUS_RESUME` reader - USB host suspend state and wake up device"]
pub type RbUhBusResumeR = crate::BitReader;
#[doc = "Field `RB_UH_BUS_RESUME` writer - USB host suspend state and wake up device"]
pub type RbUhBusResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UH_AUTOSOF_EN` reader - Automatically generate sof packet enable control"]
pub type RbUhAutosofEnR = crate::BitReader;
#[doc = "Field `RB_UH_AUTOSOF_EN` writer - Automatically generate sof packet enable control"]
pub type RbUhAutosofEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB host send bus reset signal"]
    #[inline(always)]
    pub fn rb_uh_bus_reset(&self) -> RbUhBusResetR {
        RbUhBusResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB host send bus suspend signal"]
    #[inline(always)]
    pub fn rb_uh_bus_suspend(&self) -> RbUhBusSuspendR {
        RbUhBusSuspendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB host suspend state and wake up device"]
    #[inline(always)]
    pub fn rb_uh_bus_resume(&self) -> RbUhBusResumeR {
        RbUhBusResumeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Automatically generate sof packet enable control"]
    #[inline(always)]
    pub fn rb_uh_autosof_en(&self) -> RbUhAutosofEnR {
        RbUhAutosofEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB host send bus reset signal"]
    #[inline(always)]
    pub fn rb_uh_bus_reset(&mut self) -> RbUhBusResetW<'_, R8UhostCtrlSpec> {
        RbUhBusResetW::new(self, 0)
    }
    #[doc = "Bit 1 - USB host send bus suspend signal"]
    #[inline(always)]
    pub fn rb_uh_bus_suspend(&mut self) -> RbUhBusSuspendW<'_, R8UhostCtrlSpec> {
        RbUhBusSuspendW::new(self, 1)
    }
    #[doc = "Bit 2 - USB host suspend state and wake up device"]
    #[inline(always)]
    pub fn rb_uh_bus_resume(&mut self) -> RbUhBusResumeW<'_, R8UhostCtrlSpec> {
        RbUhBusResumeW::new(self, 2)
    }
    #[doc = "Bit 7 - Automatically generate sof packet enable control"]
    #[inline(always)]
    pub fn rb_uh_autosof_en(&mut self) -> RbUhAutosofEnW<'_, R8UhostCtrlSpec> {
        RbUhAutosofEnW::new(self, 7)
    }
}
#[doc = "USB host control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uhost_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uhost_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8UhostCtrlSpec;
impl crate::RegisterSpec for R8UhostCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uhost_ctrl::R`](R) reader structure"]
impl crate::Readable for R8UhostCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uhost_ctrl::W`](W) writer structure"]
impl crate::Writable for R8UhostCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UHOST_CTRL to value 0"]
impl crate::Resettable for R8UhostCtrlSpec {}
