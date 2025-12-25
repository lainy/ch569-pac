#[doc = "Register `IPRIOR33` reader"]
pub type R = crate::R<Iprior33Spec>;
#[doc = "Register `IPRIOR33` writer"]
pub type W = crate::W<Iprior33Spec>;
#[doc = "Field `IPRIOR33` reader - IPRIOR33"]
pub type Iprior33R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR33` writer - IPRIOR33"]
pub type Iprior33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR33"]
    #[inline(always)]
    pub fn iprior33(&self) -> Iprior33R {
        Iprior33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR33"]
    #[inline(always)]
    pub fn iprior33(&mut self) -> Iprior33W<'_, Iprior33Spec> {
        Iprior33W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior33Spec;
impl crate::RegisterSpec for Iprior33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior33::R`](R) reader structure"]
impl crate::Readable for Iprior33Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior33::W`](W) writer structure"]
impl crate::Writable for Iprior33Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR33 to value 0"]
impl crate::Resettable for Iprior33Spec {}
