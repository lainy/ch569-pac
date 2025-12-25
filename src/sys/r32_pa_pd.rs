#[doc = "Register `R32_PA_PD` reader"]
pub type R = crate::R<R32PaPdSpec>;
#[doc = "Register `R32_PA_PD` writer"]
pub type W = crate::W<R32PaPdSpec>;
#[doc = "Field `R32_PA_PD` reader - GPIO PA output open-drain_input pulldown resistance enable"]
pub type R32PaPdR = crate::FieldReader<u32>;
#[doc = "Field `R32_PA_PD` writer - GPIO PA output open-drain_input pulldown resistance enable"]
pub type R32PaPdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA output open-drain_input pulldown resistance enable"]
    #[inline(always)]
    pub fn r32_pa_pd(&self) -> R32PaPdR {
        R32PaPdR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPIO PA output open-drain_input pulldown resistance enable"]
    #[inline(always)]
    pub fn r32_pa_pd(&mut self) -> R32PaPdW<'_, R32PaPdSpec> {
        R32PaPdW::new(self, 0)
    }
}
#[doc = "GPIO PA output open-drain_input pulldown resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaPdSpec;
impl crate::RegisterSpec for R32PaPdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_pd::R`](R) reader structure"]
impl crate::Readable for R32PaPdSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_pd::W`](W) writer structure"]
impl crate::Writable for R32PaPdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_PD to value 0"]
impl crate::Resettable for R32PaPdSpec {}
