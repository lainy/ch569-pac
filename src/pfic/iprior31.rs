#[doc = "Register `IPRIOR31` reader"]
pub type R = crate::R<Iprior31Spec>;
#[doc = "Register `IPRIOR31` writer"]
pub type W = crate::W<Iprior31Spec>;
#[doc = "Field `IPRIOR31` reader - IPRIOR31"]
pub type Iprior31R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR31` writer - IPRIOR31"]
pub type Iprior31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR31"]
    #[inline(always)]
    pub fn iprior31(&self) -> Iprior31R {
        Iprior31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR31"]
    #[inline(always)]
    pub fn iprior31(&mut self) -> Iprior31W<'_, Iprior31Spec> {
        Iprior31W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior31Spec;
impl crate::RegisterSpec for Iprior31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior31::R`](R) reader structure"]
impl crate::Readable for Iprior31Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior31::W`](W) writer structure"]
impl crate::Writable for Iprior31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR31 to value 0"]
impl crate::Resettable for Iprior31Spec {}
