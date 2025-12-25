#[doc = "Register `IPRIOR60` reader"]
pub type R = crate::R<Iprior60Spec>;
#[doc = "Register `IPRIOR60` writer"]
pub type W = crate::W<Iprior60Spec>;
#[doc = "Field `IPRIOR60` reader - IPRIOR60"]
pub type Iprior60R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR60` writer - IPRIOR60"]
pub type Iprior60W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR60"]
    #[inline(always)]
    pub fn iprior60(&self) -> Iprior60R {
        Iprior60R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR60"]
    #[inline(always)]
    pub fn iprior60(&mut self) -> Iprior60W<'_, Iprior60Spec> {
        Iprior60W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior60Spec;
impl crate::RegisterSpec for Iprior60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior60::R`](R) reader structure"]
impl crate::Readable for Iprior60Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior60::W`](W) writer structure"]
impl crate::Writable for Iprior60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR60 to value 0"]
impl crate::Resettable for Iprior60Spec {}
