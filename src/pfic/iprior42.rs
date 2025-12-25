#[doc = "Register `IPRIOR42` reader"]
pub type R = crate::R<Iprior42Spec>;
#[doc = "Register `IPRIOR42` writer"]
pub type W = crate::W<Iprior42Spec>;
#[doc = "Field `IPRIOR42` reader - IPRIOR42"]
pub type Iprior42R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR42` writer - IPRIOR42"]
pub type Iprior42W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR42"]
    #[inline(always)]
    pub fn iprior42(&self) -> Iprior42R {
        Iprior42R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR42"]
    #[inline(always)]
    pub fn iprior42(&mut self) -> Iprior42W<'_, Iprior42Spec> {
        Iprior42W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior42Spec;
impl crate::RegisterSpec for Iprior42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior42::R`](R) reader structure"]
impl crate::Readable for Iprior42Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior42::W`](W) writer structure"]
impl crate::Writable for Iprior42Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR42 to value 0"]
impl crate::Resettable for Iprior42Spec {}
