#[doc = "Register `IPRIOR39` reader"]
pub type R = crate::R<Iprior39Spec>;
#[doc = "Register `IPRIOR39` writer"]
pub type W = crate::W<Iprior39Spec>;
#[doc = "Field `IPRIOR39` reader - IPRIOR39"]
pub type Iprior39R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR39` writer - IPRIOR39"]
pub type Iprior39W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR39"]
    #[inline(always)]
    pub fn iprior39(&self) -> Iprior39R {
        Iprior39R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR39"]
    #[inline(always)]
    pub fn iprior39(&mut self) -> Iprior39W<'_, Iprior39Spec> {
        Iprior39W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior39Spec;
impl crate::RegisterSpec for Iprior39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior39::R`](R) reader structure"]
impl crate::Readable for Iprior39Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior39::W`](W) writer structure"]
impl crate::Writable for Iprior39Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR39 to value 0"]
impl crate::Resettable for Iprior39Spec {}
