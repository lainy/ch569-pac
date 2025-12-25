#[doc = "Register `R32_PA_CLR` writer"]
pub type W = crate::W<R32PaClrSpec>;
#[doc = "Field `R32_PA_CLR` writer - GPIO PA clear output"]
pub type R32PaClrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - GPIO PA clear output"]
    #[inline(always)]
    pub fn r32_pa_clr(&mut self) -> R32PaClrW<'_, R32PaClrSpec> {
        R32PaClrW::new(self, 0)
    }
}
#[doc = "GPIO PA clear output\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaClrSpec;
impl crate::RegisterSpec for R32PaClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r32_pa_clr::W`](W) writer structure"]
impl crate::Writable for R32PaClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_CLR to value 0"]
impl crate::Resettable for R32PaClrSpec {}
