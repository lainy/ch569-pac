#[doc = "Register `R32_ECDC_IV_95T64` reader"]
pub type R = crate::R<R32EcdcIv95t64Spec>;
#[doc = "Register `R32_ECDC_IV_95T64` writer"]
pub type W = crate::W<R32EcdcIv95t64Spec>;
#[doc = "Field `RB_ECDC_IV_95T64` reader - CTR mode count 64-95 register"]
pub type RbEcdcIv95t64R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_IV_95T64` writer - CTR mode count 64-95 register"]
pub type RbEcdcIv95t64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CTR mode count 64-95 register"]
    #[inline(always)]
    pub fn rb_ecdc_iv_95t64(&self) -> RbEcdcIv95t64R {
        RbEcdcIv95t64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CTR mode count 64-95 register"]
    #[inline(always)]
    pub fn rb_ecdc_iv_95t64(&mut self) -> RbEcdcIv95t64W<'_, R32EcdcIv95t64Spec> {
        RbEcdcIv95t64W::new(self, 0)
    }
}
#[doc = "CTR mode count 64-95 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_iv_95t64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_iv_95t64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcIv95t64Spec;
impl crate::RegisterSpec for R32EcdcIv95t64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_iv_95t64::R`](R) reader structure"]
impl crate::Readable for R32EcdcIv95t64Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_iv_95t64::W`](W) writer structure"]
impl crate::Writable for R32EcdcIv95t64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_IV_95T64 to value 0"]
impl crate::Resettable for R32EcdcIv95t64Spec {}
