#[doc = "Register `IPRIOR61` reader"]
pub type R = crate::R<Iprior61Spec>;
#[doc = "Register `IPRIOR61` writer"]
pub type W = crate::W<Iprior61Spec>;
#[doc = "Field `IPRIOR61` reader - IPRIOR61"]
pub type Iprior61R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR61` writer - IPRIOR61"]
pub type Iprior61W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR61"]
    #[inline(always)]
    pub fn iprior61(&self) -> Iprior61R {
        Iprior61R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR61"]
    #[inline(always)]
    pub fn iprior61(&mut self) -> Iprior61W<'_, Iprior61Spec> {
        Iprior61W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior61Spec;
impl crate::RegisterSpec for Iprior61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior61::R`](R) reader structure"]
impl crate::Readable for Iprior61Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior61::W`](W) writer structure"]
impl crate::Writable for Iprior61Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR61 to value 0"]
impl crate::Resettable for Iprior61Spec {}
