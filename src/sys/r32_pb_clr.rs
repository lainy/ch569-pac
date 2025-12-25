#[doc = "Register `R32_PB_CLR` writer"]
pub type W = crate::W<R32PbClrSpec>;
#[doc = "Field `R32_PB_CLR` writer - GPIO PB clear output"]
pub type R32PbClrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl W {
    #[doc = "Bits 0:24 - GPIO PB clear output"]
    #[inline(always)]
    pub fn r32_pb_clr(&mut self) -> R32PbClrW<'_, R32PbClrSpec> {
        R32PbClrW::new(self, 0)
    }
}
#[doc = "GPIO PB clear output\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbClrSpec;
impl crate::RegisterSpec for R32PbClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r32_pb_clr::W`](W) writer structure"]
impl crate::Writable for R32PbClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_CLR to value 0"]
impl crate::Resettable for R32PbClrSpec {}
