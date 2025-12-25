#[doc = "Register `SCTLR` reader"]
pub type R = crate::R<SctlrSpec>;
#[doc = "Register `SCTLR` writer"]
pub type W = crate::W<SctlrSpec>;
#[doc = "Field `SLEEPONEXIT` reader - SLEEPONEXIT"]
pub type SleeponexitR = crate::BitReader;
#[doc = "Field `SLEEPONEXIT` writer - SLEEPONEXIT"]
pub type SleeponexitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPDEEP` reader - SLEEPDEEP"]
pub type SleepdeepR = crate::BitReader;
#[doc = "Field `SLEEPDEEP` writer - SLEEPDEEP"]
pub type SleepdeepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFITOWFE` reader - WFITOWFE"]
pub type WfitowfeR = crate::BitReader;
#[doc = "Field `WFITOWFE` writer - WFITOWFE"]
pub type WfitowfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEVONPEND` reader - SEVONPEND"]
pub type SevonpendR = crate::BitReader;
#[doc = "Field `SEVONPEND` writer - SEVONPEND"]
pub type SevonpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETEVENT` reader - SETEVENT"]
pub type SeteventR = crate::BitReader;
#[doc = "Field `SETEVENT` writer - SETEVENT"]
pub type SeteventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SleeponexitR {
        SleeponexitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SleepdeepR {
        SleepdeepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&self) -> WfitowfeR {
        WfitowfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SevonpendR {
        SevonpendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&self) -> SeteventR {
        SeteventR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SleeponexitW<'_, SctlrSpec> {
        SleeponexitW::new(self, 1)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SleepdeepW<'_, SctlrSpec> {
        SleepdeepW::new(self, 2)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&mut self) -> WfitowfeW<'_, SctlrSpec> {
        WfitowfeW::new(self, 3)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SevonpendW<'_, SctlrSpec> {
        SevonpendW::new(self, 4)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&mut self) -> SeteventW<'_, SctlrSpec> {
        SeteventW::new(self, 5)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SctlrSpec;
impl crate::RegisterSpec for SctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctlr::R`](R) reader structure"]
impl crate::Readable for SctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`sctlr::W`](W) writer structure"]
impl crate::Writable for SctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCTLR to value 0"]
impl crate::Resettable for SctlrSpec {}
