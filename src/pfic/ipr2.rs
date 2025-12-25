#[doc = "Register `IPR2` reader"]
pub type R = crate::R<Ipr2Spec>;
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PendstaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PendstaR {
        PendstaR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipr2Spec;
impl crate::RegisterSpec for Ipr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr2::R`](R) reader structure"]
impl crate::Readable for Ipr2Spec {}
#[doc = "`reset()` method sets IPR2 to value 0"]
impl crate::Resettable for Ipr2Spec {}
