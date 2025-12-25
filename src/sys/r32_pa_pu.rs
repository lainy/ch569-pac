#[doc = "Register `R32_PA_PU` reader"]
pub type R = crate::R<R32PaPuSpec>;
#[doc = "Register `R32_PA_PU` writer"]
pub type W = crate::W<R32PaPuSpec>;
#[doc = "Field `R32_PA_PU` reader - GPIO PA pullup resistance enable"]
pub type R32PaPuR = crate::FieldReader<u32>;
#[doc = "Field `R32_PA_PU` writer - GPIO PA pullup resistance enable"]
pub type R32PaPuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA pullup resistance enable"]
    #[inline(always)]
    pub fn r32_pa_pu(&self) -> R32PaPuR {
        R32PaPuR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPIO PA pullup resistance enable"]
    #[inline(always)]
    pub fn r32_pa_pu(&mut self) -> R32PaPuW<'_, R32PaPuSpec> {
        R32PaPuW::new(self, 0)
    }
}
#[doc = "GPIO PA pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaPuSpec;
impl crate::RegisterSpec for R32PaPuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_pu::R`](R) reader structure"]
impl crate::Readable for R32PaPuSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_pu::W`](W) writer structure"]
impl crate::Writable for R32PaPuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_PU to value 0"]
impl crate::Resettable for R32PaPuSpec {}
