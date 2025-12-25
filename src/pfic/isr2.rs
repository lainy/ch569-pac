#[doc = "Register `ISR2` reader"]
pub type R = crate::R<Isr2Spec>;
#[doc = "Field `INTENSTA` reader - Interrupt ID Status"]
pub type IntenstaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&self) -> IntenstaR {
        IntenstaR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isr2Spec;
impl crate::RegisterSpec for Isr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr2::R`](R) reader structure"]
impl crate::Readable for Isr2Spec {}
#[doc = "`reset()` method sets ISR2 to value 0"]
impl crate::Resettable for Isr2Spec {}
