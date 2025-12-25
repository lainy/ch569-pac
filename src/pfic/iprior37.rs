#[doc = "Register `IPRIOR37` reader"]
pub type R = crate::R<Iprior37Spec>;
#[doc = "Register `IPRIOR37` writer"]
pub type W = crate::W<Iprior37Spec>;
#[doc = "Field `IPRIOR37` reader - IPRIOR37"]
pub type Iprior37R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR37` writer - IPRIOR37"]
pub type Iprior37W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR37"]
    #[inline(always)]
    pub fn iprior37(&self) -> Iprior37R {
        Iprior37R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR37"]
    #[inline(always)]
    pub fn iprior37(&mut self) -> Iprior37W<'_, Iprior37Spec> {
        Iprior37W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior37Spec;
impl crate::RegisterSpec for Iprior37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior37::R`](R) reader structure"]
impl crate::Readable for Iprior37Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior37::W`](W) writer structure"]
impl crate::Writable for Iprior37Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR37 to value 0"]
impl crate::Resettable for Iprior37Spec {}
