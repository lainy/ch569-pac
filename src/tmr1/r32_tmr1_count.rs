#[doc = "Register `R32_TMR1_COUNT` reader"]
pub type R = crate::R<R32Tmr1CountSpec>;
#[doc = "Field `R32_TMR1_COUNT` reader - TMR current count"]
pub type R32Tmr1CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TMR current count"]
    #[inline(always)]
    pub fn r32_tmr1_count(&self) -> R32Tmr1CountR {
        R32Tmr1CountR::new(self.bits)
    }
}
#[doc = "TMR1 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr1CountSpec;
impl crate::RegisterSpec for R32Tmr1CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr1_count::R`](R) reader structure"]
impl crate::Readable for R32Tmr1CountSpec {}
#[doc = "`reset()` method sets R32_TMR1_COUNT to value 0"]
impl crate::Resettable for R32Tmr1CountSpec {}
