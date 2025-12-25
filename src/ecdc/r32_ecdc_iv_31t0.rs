#[doc = "Register `R32_ECDC_IV_31T0` reader"]
pub type R = crate::R<R32EcdcIv31t0Spec>;
#[doc = "Register `R32_ECDC_IV_31T0` writer"]
pub type W = crate::W<R32EcdcIv31t0Spec>;
#[doc = "Field `RB_ECDC_IV_31T0` reader - CTR mode count 0-31 register"]
pub type RbEcdcIv31t0R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_IV_31T0` writer - CTR mode count 0-31 register"]
pub type RbEcdcIv31t0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CTR mode count 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_iv_31t0(&self) -> RbEcdcIv31t0R {
        RbEcdcIv31t0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CTR mode count 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_iv_31t0(&mut self) -> RbEcdcIv31t0W<'_, R32EcdcIv31t0Spec> {
        RbEcdcIv31t0W::new(self, 0)
    }
}
#[doc = "CTR mode count 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_iv_31t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_iv_31t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcIv31t0Spec;
impl crate::RegisterSpec for R32EcdcIv31t0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_iv_31t0::R`](R) reader structure"]
impl crate::Readable for R32EcdcIv31t0Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_iv_31t0::W`](W) writer structure"]
impl crate::Writable for R32EcdcIv31t0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_IV_31T0 to value 0"]
impl crate::Resettable for R32EcdcIv31t0Spec {}
