#[doc = "Register `IPRIOR58` reader"]
pub type R = crate::R<Iprior58Spec>;
#[doc = "Register `IPRIOR58` writer"]
pub type W = crate::W<Iprior58Spec>;
#[doc = "Field `IPRIOR58` reader - IPRIOR58"]
pub type Iprior58R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR58` writer - IPRIOR58"]
pub type Iprior58W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR58"]
    #[inline(always)]
    pub fn iprior58(&self) -> Iprior58R {
        Iprior58R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR58"]
    #[inline(always)]
    pub fn iprior58(&mut self) -> Iprior58W<'_, Iprior58Spec> {
        Iprior58W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior58Spec;
impl crate::RegisterSpec for Iprior58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior58::R`](R) reader structure"]
impl crate::Readable for Iprior58Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior58::W`](W) writer structure"]
impl crate::Writable for Iprior58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR58 to value 0"]
impl crate::Resettable for Iprior58Spec {}
