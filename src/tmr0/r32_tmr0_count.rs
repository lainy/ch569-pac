#[doc = "Register `R32_TMR0_COUNT` reader"]
pub type R = crate::R<R32Tmr0CountSpec>;
#[doc = "Field `R32_TMR0_COUNT` reader - TMR0 current count"]
pub type R32Tmr0CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TMR0 current count"]
    #[inline(always)]
    pub fn r32_tmr0_count(&self) -> R32Tmr0CountR {
        R32Tmr0CountR::new(self.bits)
    }
}
#[doc = "TMR0 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr0_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr0CountSpec;
impl crate::RegisterSpec for R32Tmr0CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr0_count::R`](R) reader structure"]
impl crate::Readable for R32Tmr0CountSpec {}
#[doc = "`reset()` method sets R32_TMR0_COUNT to value 0"]
impl crate::Resettable for R32Tmr0CountSpec {}
