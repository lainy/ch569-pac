#[doc = "Register `R8_SAFE_ACCESS_ID` reader"]
pub type R = crate::R<R8SafeAccessIdSpec>;
#[doc = "Field `R8_SAFE_ACCESS_ID` reader - safe accessing ID"]
pub type R8SafeAccessIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - safe accessing ID"]
    #[inline(always)]
    pub fn r8_safe_access_id(&self) -> R8SafeAccessIdR {
        R8SafeAccessIdR::new(self.bits)
    }
}
#[doc = "safe accessing ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SafeAccessIdSpec;
impl crate::RegisterSpec for R8SafeAccessIdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_safe_access_id::R`](R) reader structure"]
impl crate::Readable for R8SafeAccessIdSpec {}
#[doc = "`reset()` method sets R8_SAFE_ACCESS_ID to value 0x02"]
impl crate::Resettable for R8SafeAccessIdSpec {
    const RESET_VALUE: u8 = 0x02;
}
