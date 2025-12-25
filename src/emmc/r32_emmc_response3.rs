#[doc = "Register `R32_EMMC_RESPONSE3` reader"]
pub type R = crate::R<R32EmmcResponse3Spec>;
#[doc = "Field `R32_EMMC_RESPONSE3` reader - response parameter register"]
pub type R32EmmcResponse3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn r32_emmc_response3(&self) -> R32EmmcResponse3R {
        R32EmmcResponse3R::new(self.bits)
    }
}
#[doc = "SD 128bits response register, \\[127:96\\] 32bits\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_response3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcResponse3Spec;
impl crate::RegisterSpec for R32EmmcResponse3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_response3::R`](R) reader structure"]
impl crate::Readable for R32EmmcResponse3Spec {}
#[doc = "`reset()` method sets R32_EMMC_RESPONSE3 to value 0"]
impl crate::Resettable for R32EmmcResponse3Spec {}
