#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `HWSTKCTRL` reader - HWSTKCTRL"]
pub type HwstkctrlR = crate::BitReader;
#[doc = "Field `HWSTKCTRL` writer - HWSTKCTRL"]
pub type HwstkctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NESTCTRL` reader - NESTCTRL"]
pub type NestctrlR = crate::BitReader;
#[doc = "Field `NESTCTRL` writer - NESTCTRL"]
pub type NestctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMISET` writer - NMISET"]
pub type NmisetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIRESET` writer - NMIRESET"]
pub type NmiresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCSET` writer - EXCSET"]
pub type ExcsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCRESET` writer - EXCRESET"]
pub type ExcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFICRSET` writer - PFICRSET"]
pub type PficrsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESET` writer - SYSRESET"]
pub type SysresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCODE` writer - KEYCODE"]
pub type KeycodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&self) -> HwstkctrlR {
        HwstkctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&self) -> NestctrlR {
        NestctrlR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&mut self) -> HwstkctrlW<'_, CfgrSpec> {
        HwstkctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&mut self) -> NestctrlW<'_, CfgrSpec> {
        NestctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - NMISET"]
    #[inline(always)]
    pub fn nmiset(&mut self) -> NmisetW<'_, CfgrSpec> {
        NmisetW::new(self, 2)
    }
    #[doc = "Bit 3 - NMIRESET"]
    #[inline(always)]
    pub fn nmireset(&mut self) -> NmiresetW<'_, CfgrSpec> {
        NmiresetW::new(self, 3)
    }
    #[doc = "Bit 4 - EXCSET"]
    #[inline(always)]
    pub fn excset(&mut self) -> ExcsetW<'_, CfgrSpec> {
        ExcsetW::new(self, 4)
    }
    #[doc = "Bit 5 - EXCRESET"]
    #[inline(always)]
    pub fn excreset(&mut self) -> ExcresetW<'_, CfgrSpec> {
        ExcresetW::new(self, 5)
    }
    #[doc = "Bit 6 - PFICRSET"]
    #[inline(always)]
    pub fn pficrset(&mut self) -> PficrsetW<'_, CfgrSpec> {
        PficrsetW::new(self, 6)
    }
    #[doc = "Bit 7 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SysresetW<'_, CfgrSpec> {
        SysresetW::new(self, 7)
    }
    #[doc = "Bits 16:31 - KEYCODE"]
    #[inline(always)]
    pub fn keycode(&mut self) -> KeycodeW<'_, CfgrSpec> {
        KeycodeW::new(self, 16)
    }
}
#[doc = "Interrupt Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
