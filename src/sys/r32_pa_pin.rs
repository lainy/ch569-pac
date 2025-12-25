#[doc = "Register `R32_PA_PIN` reader"]
pub type R = crate::R<R32PaPinSpec>;
#[doc = "Field `R32_PA_PIN` reader - GPIO PA input"]
pub type R32PaPinR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA input"]
    #[inline(always)]
    pub fn r32_pa_pin(&self) -> R32PaPinR {
        R32PaPinR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "GPIO PA input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaPinSpec;
impl crate::RegisterSpec for R32PaPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_pin::R`](R) reader structure"]
impl crate::Readable for R32PaPinSpec {}
#[doc = "`reset()` method sets R32_PA_PIN to value 0"]
impl crate::Resettable for R32PaPinSpec {}
