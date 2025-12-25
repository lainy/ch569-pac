#[doc = "Register `R32_ECDC_SGRT_63T32` reader"]
pub type R = crate::R<R32EcdcSgrt63t32Spec>;
#[doc = "Register `R32_ECDC_SGRT_63T32` writer"]
pub type W = crate::W<R32EcdcSgrt63t32Spec>;
#[doc = "Field `RB_ECDC_SGRT_63T32` reader - Single encryption and decryption result 0-31 register"]
pub type RbEcdcSgrt63t32R = crate::FieldReader<u32>;
#[doc = "Field `RB_ECDC_SGRT_63T32` writer - Single encryption and decryption result 0-31 register"]
pub type RbEcdcSgrt63t32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_63t32(&self) -> RbEcdcSgrt63t32R {
        RbEcdcSgrt63t32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single encryption and decryption result 0-31 register"]
    #[inline(always)]
    pub fn rb_ecdc_sgrt_63t32(&mut self) -> RbEcdcSgrt63t32W<'_, R32EcdcSgrt63t32Spec> {
        RbEcdcSgrt63t32W::new(self, 0)
    }
}
#[doc = "Single encryption and decryption result 0-31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_ecdc_sgrt_63t32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_ecdc_sgrt_63t32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EcdcSgrt63t32Spec;
impl crate::RegisterSpec for R32EcdcSgrt63t32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_ecdc_sgrt_63t32::R`](R) reader structure"]
impl crate::Readable for R32EcdcSgrt63t32Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_ecdc_sgrt_63t32::W`](W) writer structure"]
impl crate::Writable for R32EcdcSgrt63t32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ECDC_SGRT_63T32 to value 0"]
impl crate::Resettable for R32EcdcSgrt63t32Spec {}
