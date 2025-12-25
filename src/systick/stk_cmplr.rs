#[doc = "Register `STK_CMPLR` reader"]
pub type R = crate::R<StkCmplrSpec>;
#[doc = "Register `STK_CMPLR` writer"]
pub type W = crate::W<StkCmplrSpec>;
#[doc = "Field `CMPL` reader - CMPL"]
pub type CmplR = crate::FieldReader<u32>;
#[doc = "Field `CMPL` writer - CMPL"]
pub type CmplW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CMPL"]
    #[inline(always)]
    pub fn cmpl(&self) -> CmplR {
        CmplR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMPL"]
    #[inline(always)]
    pub fn cmpl(&mut self) -> CmplW<'_, StkCmplrSpec> {
        CmplW::new(self, 0)
    }
}
#[doc = "Systick compare low register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cmplr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cmplr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StkCmplrSpec;
impl crate::RegisterSpec for StkCmplrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cmplr::R`](R) reader structure"]
impl crate::Readable for StkCmplrSpec {}
#[doc = "`write(|w| ..)` method takes [`stk_cmplr::W`](W) writer structure"]
impl crate::Writable for StkCmplrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STK_CMPLR to value 0"]
impl crate::Resettable for StkCmplrSpec {}
