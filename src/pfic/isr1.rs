#[doc = "Register `ISR1` reader"]
pub type R = crate::R<Isr1Spec>;
#[doc = "Field `INTSTA` reader - Interrupt ID Status"]
pub type IntstaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 12:31 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intsta(&self) -> IntstaR {
        IntstaR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isr1Spec;
impl crate::RegisterSpec for Isr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr1::R`](R) reader structure"]
impl crate::Readable for Isr1Spec {}
#[doc = "`reset()` method sets ISR1 to value 0"]
impl crate::Resettable for Isr1Spec {}
