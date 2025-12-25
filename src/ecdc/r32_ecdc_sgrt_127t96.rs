#[doc = "Register `R32_ECDC_SGRT_127T96` reader"]
pub type R = crate::R<R32EcdcSgrt127t96Spec>;
#[doc = "Register `R32_ECDC_SGRT_127T96` writer"]
pub type W = crate::W<R32EcdcSgrt127t96Spec>;
#[doc = "Field `RB_ECDC_SGRT_127T96` reader - Single encryption and decryption result 96-127 register"]
pub type RbEcdcSgrt127t96R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_SGRT_127T96` writer - Single encryption and decryption result 96-127 register"]
pub type RbEcdcSgrt127t96W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single encryption and decryption result 96-127 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_127t96(&self) -> RbEcdcSgrt127t96R {
        RbEcdcSgrt127t96R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single encryption and decryption result 96-127 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_127t96(&mut self) -> RbEcdcSgrt127t96W<'_, R32EcdcSgrt127t96Spec> {
        RbEcdcSgrt127t96W::new(self, 0)
    }
}
#[doc = "Single encryption and decryption result 96-127 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgrt_127t96::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgrt_127t96::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcSgrt127t96Spec;
impl crate::RegisterSpec for R32EcdcSgrt127t96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_sgrt_127t96::R`](R) reader structure"]
impl crate::Readable for R32EcdcSgrt127t96Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_sgrt_127t96::W`](W) writer structure"]
impl crate::Writable for R32EcdcSgrt127t96Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_SGRT_127T96 to value 0"]
impl crate::Resettable for R32EcdcSgrt127t96Spec {}
