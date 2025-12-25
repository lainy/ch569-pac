#[doc = "Register `IPR1` reader"]
pub type R = crate::R<Ipr1Spec>;
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PendstaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 12:31 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PendstaR {
        PendstaR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipr1Spec;
impl crate::RegisterSpec for Ipr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr1::R`](R) reader structure"]
impl crate::Readable for Ipr1Spec {}
#[doc = "`reset()` method sets IPR1 to value 0"]
impl crate::Resettable for Ipr1Spec {}
