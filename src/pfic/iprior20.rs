#[doc = "Register `IPRIOR20` reader"]
pub type R = crate::R<Iprior20Spec>;
#[doc = "Register `IPRIOR20` writer"]
pub type W = crate::W<Iprior20Spec>;
#[doc = "Field `IPRIOR20` reader - IPRIOR20"]
pub type Iprior20R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR20` writer - IPRIOR20"]
pub type Iprior20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR20"]
    #[inline(always)]
    pub fn iprior20(&self) -> Iprior20R {
        Iprior20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR20"]
    #[inline(always)]
    pub fn iprior20(&mut self) -> Iprior20W<'_, Iprior20Spec> {
        Iprior20W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior20Spec;
impl crate::RegisterSpec for Iprior20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior20::R`](R) reader structure"]
impl crate::Readable for Iprior20Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior20::W`](W) writer structure"]
impl crate::Writable for Iprior20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR20 to value 0"]
impl crate::Resettable for Iprior20Spec {}
