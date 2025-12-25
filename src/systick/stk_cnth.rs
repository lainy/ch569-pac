#[doc = "Register `STK_CNTH` reader"]
pub type R = crate::R<StkCnthSpec>;
#[doc = "Register `STK_CNTH` writer"]
pub type W = crate::W<StkCnthSpec>;
#[doc = "Field `CNTH` reader - CNTH"]
pub type CnthR = crate::FieldReader<u32>;
#[doc = "Field `CNTH` writer - CNTH"]
pub type CnthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTH"]
    #[inline(always)]
    pub fn cnth(&self) -> CnthR {
        CnthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTH"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CnthW<'_, StkCnthSpec> {
        CnthW::new(self, 0)
    }
}
#[doc = "Systick counter high register\n\nYou can [`read`](crate::Reg::read) this register and get [`stk_cnth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stk_cnth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StkCnthSpec;
impl crate::RegisterSpec for StkCnthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cnth::R`](R) reader structure"]
impl crate::Readable for StkCnthSpec {}
#[doc = "`write(|w| ..)` method takes [`stk_cnth::W`](W) writer structure"]
impl crate::Writable for StkCnthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STK_CNTH to value 0"]
impl crate::Resettable for StkCnthSpec {}
