#[doc = "Register `IPRIOR50` reader"]
pub type R = crate::R<Iprior50Spec>;
#[doc = "Register `IPRIOR50` writer"]
pub type W = crate::W<Iprior50Spec>;
#[doc = "Field `IPRIOR50` reader - IPRIOR50"]
pub type Iprior50R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR50` writer - IPRIOR50"]
pub type Iprior50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR50"]
    #[inline(always)]
    pub fn iprior50(&self) -> Iprior50R {
        Iprior50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR50"]
    #[inline(always)]
    pub fn iprior50(&mut self) -> Iprior50W<'_, Iprior50Spec> {
        Iprior50W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior50Spec;
impl crate::RegisterSpec for Iprior50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior50::R`](R) reader structure"]
impl crate::Readable for Iprior50Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior50::W`](W) writer structure"]
impl crate::Writable for Iprior50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR50 to value 0"]
impl crate::Resettable for Iprior50Spec {}
