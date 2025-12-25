#[doc = "Register `STK_CNTL` reader"]
pub type R = crate::R<StkCntlSpec>;
#[doc = "Register `STK_CNTL` writer"]
pub type W = crate::W<StkCntlSpec>;
#[doc = "Field `CNTL` reader - CNTL"]
pub type CntlR = crate::FieldReader<u32>;
#[doc = "Field `CNTL` writer - CNTL"]
pub type CntlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTL"]
    #[inline(always)]
    pub fn cntl(&self) -> CntlR {
        CntlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTL"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CntlW<'_, StkCntlSpec> {
        CntlW::new(self, 0)
    }
}
#[doc = "Systick counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StkCntlSpec;
impl crate::RegisterSpec for StkCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cntl::R`](R) reader structure"]
impl crate::Readable for StkCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`stk_cntl::W`](W) writer structure"]
impl crate::Writable for StkCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STK_CNTL to value 0"]
impl crate::Resettable for StkCntlSpec {}
