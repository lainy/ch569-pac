#[doc = "Register `IPRIOR41` reader"]
pub type R = crate::R<Iprior41Spec>;
#[doc = "Register `IPRIOR41` writer"]
pub type W = crate::W<Iprior41Spec>;
#[doc = "Field `IPRIOR41` reader - IPRIOR41"]
pub type Iprior41R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR41` writer - IPRIOR41"]
pub type Iprior41W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR41"]
    #[inline(always)]
    pub fn iprior41(&self) -> Iprior41R {
        Iprior41R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR41"]
    #[inline(always)]
    pub fn iprior41(&mut self) -> Iprior41W<'_, Iprior41Spec> {
        Iprior41W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior41Spec;
impl crate::RegisterSpec for Iprior41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior41::R`](R) reader structure"]
impl crate::Readable for Iprior41Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior41::W`](W) writer structure"]
impl crate::Writable for Iprior41Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR41 to value 0"]
impl crate::Resettable for Iprior41Spec {}
