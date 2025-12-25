#[doc = "Register `R32_PB_PIN` reader"]
pub type R = crate::R<R32PbPinSpec>;
#[doc = "Field `R32_PB_PIN` reader - GPIO PB input"]
pub type R32PbPinR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB input"]
    #[inline(always)]
    pub fn r32_pb_pin(&self) -> R32PbPinR {
        R32PbPinR::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "GPIO PB input\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbPinSpec;
impl crate::RegisterSpec for R32PbPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_pin::R`](R) reader structure"]
impl crate::Readable for R32PbPinSpec {}
#[doc = "`reset()` method sets R32_PB_PIN to value 0"]
impl crate::Resettable for R32PbPinSpec {}
