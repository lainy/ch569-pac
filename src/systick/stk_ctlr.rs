#[doc = "Register `STK_CTLR` reader"]
pub type R = crate::R<StkCtlrSpec>;
#[doc = "Register `STK_CTLR` writer"]
pub type W = crate::W<StkCtlrSpec>;
#[doc = "Field `STE` reader - Systick counter enable"]
pub type SteR = crate::BitReader;
#[doc = "Field `STE` writer - Systick counter enable"]
pub type SteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIE` reader - Systick counter interrupt enable"]
pub type StieR = crate::BitReader;
#[doc = "Field `STIE` writer - Systick counter interrupt enable"]
pub type StieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STCLK` reader - System counter clock Source selection"]
pub type StclkR = crate::BitReader;
#[doc = "Field `STCLK` writer - System counter clock Source selection"]
pub type StclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRELOAD` reader - System counter reload control"]
pub type StreloadR = crate::BitReader;
#[doc = "Field `STRELOAD` writer - System counter reload control"]
pub type StreloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Systick counter enable"]
    #[inline(always)]
    pub fn ste(&self) -> SteR {
        SteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Systick counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> StieR {
        StieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System counter clock Source selection"]
    #[inline(always)]
    pub fn stclk(&self) -> StclkR {
        StclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - System counter reload control"]
    #[inline(always)]
    pub fn streload(&self) -> StreloadR {
        StreloadR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Systick counter enable"]
    #[inline(always)]
    pub fn ste(&mut self) -> SteW<'_, StkCtlrSpec> {
        SteW::new(self, 0)
    }
    #[doc = "Bit 1 - Systick counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> StieW<'_, StkCtlrSpec> {
        StieW::new(self, 1)
    }
    #[doc = "Bit 2 - System counter clock Source selection"]
    #[inline(always)]
    pub fn stclk(&mut self) -> StclkW<'_, StkCtlrSpec> {
        StclkW::new(self, 2)
    }
    #[doc = "Bit 8 - System counter reload control"]
    #[inline(always)]
    pub fn streload(&mut self) -> StreloadW<'_, StkCtlrSpec> {
        StreloadW::new(self, 8)
    }
}
#[doc = "Systick counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StkCtlrSpec;
impl crate::RegisterSpec for StkCtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_ctlr::R`](R) reader structure"]
impl crate::Readable for StkCtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`stk_ctlr::W`](W) writer structure"]
impl crate::Writable for StkCtlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STK_CTLR to value 0"]
impl crate::Resettable for StkCtlrSpec {}
