#[doc = "Register `R8_SAFE_ACCESS_SIG` reader"]
pub type R = crate::R<R8SafeAccessSigSpec>;
#[doc = "Field `RB_SAFE_ACC_MODE` reader - current safe accessing mode"]
pub type RbSafeAccModeR = crate::FieldReader;
#[doc = "Field `RB_SAFE_ACC_TIMER` reader - safe accessing timer bit mask"]
pub type RbSafeAccTimerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - current safe accessing mode"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&self) -> RbSafeAccModeR {
        RbSafeAccModeR::new(self.bits & 3)
    }
    #[doc = "Bits 4:6 - safe accessing timer bit mask"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&self) -> RbSafeAccTimerR {
        RbSafeAccTimerR::new((self.bits >> 4) & 7)
    }
}
#[doc = "safe accessing sign register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_sig::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SafeAccessSigSpec;
impl crate::RegisterSpec for R8SafeAccessSigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_safe_access_sig::R`](R) reader structure"]
impl crate::Readable for R8SafeAccessSigSpec {}
#[doc = "`reset()` method sets R8_SAFE_ACCESS_SIG to value 0"]
impl crate::Resettable for R8SafeAccessSigSpec {}
