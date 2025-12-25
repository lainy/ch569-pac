#[doc = "Register `RB_ECDC_SGRT_31T0` reader"]
pub type R = crate::R<RbEcdcSgrt31t0Spec>;
#[doc = "Register `RB_ECDC_SGRT_31T0` writer"]
pub type W = crate::W<RbEcdcSgrt31t0Spec>;
#[doc = "Field `RB_ECDC_SGRT_31T0` reader - Single encryption and decryption result 0-31 register"]
pub type RbEcdcSgrt31t0R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_SGRT_31T0` writer - Single encryption and decryption result 0-31 register"]
pub type RbEcdcSgrt31t0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_31t0(&self) -> RbEcdcSgrt31t0R {
        RbEcdcSgrt31t0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_31t0(&mut self) -> RbEcdcSgrt31t0W<'_, RbEcdcSgrt31t0Spec> {
        RbEcdcSgrt31t0W::new(self, 0)
    }
}
#[doc = "Single encryption and decryption result 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rb_ecdc_sgrt_31t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rb_ecdc_sgrt_31t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbEcdcSgrt31t0Spec;
impl crate::RegisterSpec for RbEcdcSgrt31t0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rb_ecdc_sgrt_31t0::R`](R) reader structure"]
impl crate::Readable for RbEcdcSgrt31t0Spec {}
#[doc = "`write(|w| ..)` method takes [`rb_ecdc_sgrt_31t0::W`](W) writer structure"]
impl crate::Writable for RbEcdcSgrt31t0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RB_ECDC_SGRT_31T0 to value 0"]
impl crate::Resettable for RbEcdcSgrt31t0Spec {}
