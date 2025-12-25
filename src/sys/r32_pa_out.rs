#[doc = "Register `R32_PA_OUT` reader"]
pub type R = crate::R<R32PaOutSpec>;
#[doc = "Register `R32_PA_OUT` writer"]
pub type W = crate::W<R32PaOutSpec>;
#[doc = "Field `R32_PA_OUT` reader - GPIO PA output"]
pub type R32PaOutR = crate::FieldReader<u32>;
#[doc = "Field `R32_PA_OUT` writer - GPIO PA output"]
pub type R32PaOutW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA output"]
    #[inline(always)]
    pub fn r32_pa_out(&self) -> R32PaOutR {
        R32PaOutR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPIO PA output"]
    #[inline(always)]
    pub fn r32_pa_out(&mut self) -> R32PaOutW<'_, R32PaOutSpec> {
        R32PaOutW::new(self, 0)
    }
}
#[doc = "GPIO PA output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaOutSpec;
impl crate::RegisterSpec for R32PaOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_out::R`](R) reader structure"]
impl crate::Readable for R32PaOutSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_out::W`](W) writer structure"]
impl crate::Writable for R32PaOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_OUT to value 0"]
impl crate::Resettable for R32PaOutSpec {}
