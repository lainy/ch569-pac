#[doc = "Register `IPRIOR55` reader"]
pub type R = crate::R<Iprior55Spec>;
#[doc = "Register `IPRIOR55` writer"]
pub type W = crate::W<Iprior55Spec>;
#[doc = "Field `IPRIOR55` reader - IPRIOR55"]
pub type Iprior55R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR55` writer - IPRIOR55"]
pub type Iprior55W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR55"]
    #[inline(always)]
    pub fn iprior55(&self) -> Iprior55R {
        Iprior55R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR55"]
    #[inline(always)]
    pub fn iprior55(&mut self) -> Iprior55W<'_, Iprior55Spec> {
        Iprior55W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior55Spec;
impl crate::RegisterSpec for Iprior55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior55::R`](R) reader structure"]
impl crate::Readable for Iprior55Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior55::W`](W) writer structure"]
impl crate::Writable for Iprior55Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR55 to value 0"]
impl crate::Resettable for Iprior55Spec {}
