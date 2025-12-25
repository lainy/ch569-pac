#[doc = "Register `IPRIOR54` reader"]
pub type R = crate::R<Iprior54Spec>;
#[doc = "Register `IPRIOR54` writer"]
pub type W = crate::W<Iprior54Spec>;
#[doc = "Field `IPRIOR54` reader - IPRIOR54"]
pub type Iprior54R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR54` writer - IPRIOR54"]
pub type Iprior54W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR54"]
    #[inline(always)]
    pub fn iprior54(&self) -> Iprior54R {
        Iprior54R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR54"]
    #[inline(always)]
    pub fn iprior54(&mut self) -> Iprior54W<'_, Iprior54Spec> {
        Iprior54W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior54Spec;
impl crate::RegisterSpec for Iprior54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior54::R`](R) reader structure"]
impl crate::Readable for Iprior54Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior54::W`](W) writer structure"]
impl crate::Writable for Iprior54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR54 to value 0"]
impl crate::Resettable for Iprior54Spec {}
