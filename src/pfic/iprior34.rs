#[doc = "Register `IPRIOR34` reader"]
pub type R = crate::R<Iprior34Spec>;
#[doc = "Register `IPRIOR34` writer"]
pub type W = crate::W<Iprior34Spec>;
#[doc = "Field `IPRIOR34` reader - IPRIOR34"]
pub type Iprior34R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR34` writer - IPRIOR34"]
pub type Iprior34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR34"]
    #[inline(always)]
    pub fn iprior34(&self) -> Iprior34R {
        Iprior34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR34"]
    #[inline(always)]
    pub fn iprior34(&mut self) -> Iprior34W<'_, Iprior34Spec> {
        Iprior34W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior34Spec;
impl crate::RegisterSpec for Iprior34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior34::R`](R) reader structure"]
impl crate::Readable for Iprior34Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior34::W`](W) writer structure"]
impl crate::Writable for Iprior34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR34 to value 0"]
impl crate::Resettable for Iprior34Spec {}
