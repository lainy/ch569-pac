#[doc = "Register `R32_ECDC_KEY_223T192` reader"]
pub type R = crate::R<R32EcdcKey223t192Spec>;
#[doc = "Register `R32_ECDC_KEY_223T192` writer"]
pub type W = crate::W<R32EcdcKey223t192Spec>;
#[doc = "Field `RB_ECDC_KEY_223T192` reader - User key 192-223 register"]
pub type RbEcdcKey223t192R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_KEY_223T192` writer - User key 192-223 register"]
pub type RbEcdcKey223t192W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User key 192-223 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_223t192(&self) -> RbEcdcKey223t192R {
        RbEcdcKey223t192R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User key 192-223 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_223t192(&mut self) -> RbEcdcKey223t192W<'_, R32EcdcKey223t192Spec> {
        RbEcdcKey223t192W::new(self, 0)
    }
}
#[doc = "User key 192-223 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_223t192::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_223t192::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcKey223t192Spec;
impl crate::RegisterSpec for R32EcdcKey223t192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_key_223t192::R`](R) reader structure"]
impl crate::Readable for R32EcdcKey223t192Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_key_223t192::W`](W) writer structure"]
impl crate::Writable for R32EcdcKey223t192Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_KEY_223T192 to value 0"]
impl crate::Resettable for R32EcdcKey223t192Spec {}
