#[doc = "Register `R32_ECDC_KEY_159T128` reader"]
pub type R = crate::R<R32EcdcKey159t128Spec>;
#[doc = "Register `R32_ECDC_KEY_159T128` writer"]
pub type W = crate::W<R32EcdcKey159t128Spec>;
#[doc = "Field `RB_ECDC_KEY_159T128` reader - User key 128-159 register"]
pub type RbEcdcKey159t128R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_KEY_159T128` writer - User key 128-159 register"]
pub type RbEcdcKey159t128W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User key 128-159 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_159t128(&self) -> RbEcdcKey159t128R {
        RbEcdcKey159t128R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User key 128-159 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_159t128(&mut self) -> RbEcdcKey159t128W<'_, R32EcdcKey159t128Spec> {
        RbEcdcKey159t128W::new(self, 0)
    }
}
#[doc = "User key 128-159 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_159t128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_159t128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcKey159t128Spec;
impl crate::RegisterSpec for R32EcdcKey159t128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_key_159t128::R`](R) reader structure"]
impl crate::Readable for R32EcdcKey159t128Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_key_159t128::W`](W) writer structure"]
impl crate::Writable for R32EcdcKey159t128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_KEY_159T128 to value 0"]
impl crate::Resettable for R32EcdcKey159t128Spec {}
