#[doc = "Register `R32_ECDC_KEY_191T160` reader"]
pub type R = crate::R<R32EcdcKey191t160Spec>;
#[doc = "Register `R32_ECDC_KEY_191T160` writer"]
pub type W = crate::W<R32EcdcKey191t160Spec>;
#[doc = "Field `RB_ECDC_KEY_191T160` reader - User key 160-191 register"]
pub type RbEcdcKey191t160R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_KEY_191T160` writer - User key 160-191 register"]
pub type RbEcdcKey191t160W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User key 160-191 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_191t160(&self) -> RbEcdcKey191t160R {
        RbEcdcKey191t160R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User key 160-191 register"]
    #[inline(always)]
    pub fn rb_ecdc_key_191t160(&mut self) -> RbEcdcKey191t160W<'_, R32EcdcKey191t160Spec> {
        RbEcdcKey191t160W::new(self, 0)
    }
}
#[doc = "User key 160-191 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_key_191t160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_key_191t160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcKey191t160Spec;
impl crate::RegisterSpec for R32EcdcKey191t160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_key_191t160::R`](R) reader structure"]
impl crate::Readable for R32EcdcKey191t160Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_key_191t160::W`](W) writer structure"]
impl crate::Writable for R32EcdcKey191t160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_KEY_191T160 to value 0"]
impl crate::Resettable for R32EcdcKey191t160Spec {}
