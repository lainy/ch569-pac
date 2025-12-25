#[doc = "Register `STK_CNTFG` reader"]
pub type R = crate::R<StkCntfgSpec>;
#[doc = "Register `STK_CNTFG` writer"]
pub type W = crate::W<StkCntfgSpec>;
#[doc = "Field `SWIE` reader - System soft interrupt enable"]
pub type SwieR = crate::BitReader;
#[doc = "Field `SWIE` writer - System soft interrupt enable"]
pub type SwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTIF` reader - Systick counter clear zero flag"]
pub type CntifR = crate::BitReader;
#[doc = "Field `CNTIF` writer - Systick counter clear zero flag"]
pub type CntifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&self) -> SwieR {
        SwieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&self) -> CntifR {
        CntifR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&mut self) -> SwieW<'_, StkCntfgSpec> {
        SwieW::new(self, 0)
    }
    #[doc = "Bit 1 - Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&mut self) -> CntifW<'_, StkCntfgSpec> {
        CntifW::new(self, 1)
    }
}
#[doc = "Systick counter flag\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cntfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cntfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StkCntfgSpec;
impl crate::RegisterSpec for StkCntfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cntfg::R`](R) reader structure"]
impl crate::Readable for StkCntfgSpec {}
#[doc = "`write(|w| ..)` method takes [`stk_cntfg::W`](W) writer structure"]
impl crate::Writable for StkCntfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STK_CNTFG to value 0"]
impl crate::Resettable for StkCntfgSpec {}
