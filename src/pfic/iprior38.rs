#[doc = "Register `IPRIOR38` reader"]
pub type R = crate::R<Iprior38Spec>;
#[doc = "Register `IPRIOR38` writer"]
pub type W = crate::W<Iprior38Spec>;
#[doc = "Field `IPRIOR38` reader - IPRIOR38"]
pub type Iprior38R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR38` writer - IPRIOR38"]
pub type Iprior38W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR38"]
    #[inline(always)]
    pub fn iprior38(&self) -> Iprior38R {
        Iprior38R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR38"]
    #[inline(always)]
    pub fn iprior38(&mut self) -> Iprior38W<'_, Iprior38Spec> {
        Iprior38W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior38Spec;
impl crate::RegisterSpec for Iprior38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior38::R`](R) reader structure"]
impl crate::Readable for Iprior38Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior38::W`](W) writer structure"]
impl crate::Writable for Iprior38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR38 to value 0"]
impl crate::Resettable for Iprior38Spec {}
