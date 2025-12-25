#[doc = "Register `R32_PA_SMT` reader"]
pub type R = crate::R<R32PaSmtSpec>;
#[doc = "Register `R32_PA_SMT` writer"]
pub type W = crate::W<R32PaSmtSpec>;
#[doc = "Field `R32_PA_SMT` reader - GPIO PA output slew rate_input schmitt trigger"]
pub type R32PaSmtR = crate::FieldReader<u32>;
#[doc = "Field `R32_PA_SMT` writer - GPIO PA output slew rate_input schmitt trigger"]
pub type R32PaSmtW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA output slew rate_input schmitt trigger"]
    #[inline(always)]
    pub fn r32_pa_smt(&self) -> R32PaSmtR {
        R32PaSmtR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPIO PA output slew rate_input schmitt trigger"]
    #[inline(always)]
    pub fn r32_pa_smt(&mut self) -> R32PaSmtW<'_, R32PaSmtSpec> {
        R32PaSmtW::new(self, 0)
    }
}
#[doc = "GPIO PA output slew rate_input schmitt trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_smt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_smt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaSmtSpec;
impl crate::RegisterSpec for R32PaSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_smt::R`](R) reader structure"]
impl crate::Readable for R32PaSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_smt::W`](W) writer structure"]
impl crate::Writable for R32PaSmtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_SMT to value 0"]
impl crate::Resettable for R32PaSmtSpec {}
