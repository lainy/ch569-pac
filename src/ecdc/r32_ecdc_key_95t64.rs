#[doc = "Register `R32_ECDC_KEY_95T64` reader"]
pub type R = crate::R<R32EcdcKey95t64Spec>;
#[doc = "Register `R32_ECDC_KEY_95T64` writer"]
pub type W = crate::W<R32EcdcKey95t64Spec>;
#[doc = "Field `RB_ECDC_KEY_95T64` reader - User key 64-95 register"]
pub type RbEcdcKey95t64R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_KEY_95T64` writer - User key 64-95 register"]
pub type RbEcdcKey95t64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User key 64-95 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_95t64(&self) -> RbEcdcKey95t64R {
        RbEcdcKey95t64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User key 64-95 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_95t64(&mut self) -> RbEcdcKey95t64W<'_, R32EcdcKey95t64Spec> {
        RbEcdcKey95t64W::new(self, 0)
    }
}
#[doc = "User key 64-95 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_95t64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_95t64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcKey95t64Spec;
impl crate::RegisterSpec for R32EcdcKey95t64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_key_95t64::R`](R) reader structure"]
impl crate::Readable for R32EcdcKey95t64Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_key_95t64::W`](W) writer structure"]
impl crate::Writable for R32EcdcKey95t64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_KEY_95T64 to value 0"]
impl crate::Resettable for R32EcdcKey95t64Spec {}
