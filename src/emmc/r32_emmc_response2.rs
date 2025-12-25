#[doc = "Register `R32_EMMC_RESPONSE2` reader"]
pub type R = crate::R<R32EmmcResponse2Spec>;
#[doc = "Field `R32_EMMC_RESPONSE2` reader - response parameter register"]
pub type R32EmmcResponse2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn r32_emmc_response2(&self) -> R32EmmcResponse2R {
        R32EmmcResponse2R::new(self.bits)
    }
}
#[doc = "SD 128bits response register, \\[95:64\\] 32bits\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_response2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcResponse2Spec;
impl crate::RegisterSpec for R32EmmcResponse2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_response2::R`](R) reader structure"]
impl crate::Readable for R32EmmcResponse2Spec {}
#[doc = "`reset()` method sets R32_EMMC_RESPONSE2 to value 0"]
impl crate::Resettable for R32EmmcResponse2Spec {}
