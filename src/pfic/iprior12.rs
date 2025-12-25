#[doc = "Register `IPRIOR12` reader"]
pub type R = crate::R<Iprior12Spec>;
#[doc = "Register `IPRIOR12` writer"]
pub type W = crate::W<Iprior12Spec>;
#[doc = "Field `IPRIOR12` reader - IPRIOR12"]
pub type Iprior12R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR12` writer - IPRIOR12"]
pub type Iprior12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR12"]
    #[inline(always)]
    pub fn iprior12(&self) -> Iprior12R {
        Iprior12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR12"]
    #[inline(always)]
    pub fn iprior12(&mut self) -> Iprior12W<'_, Iprior12Spec> {
        Iprior12W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior12Spec;
impl crate::RegisterSpec for Iprior12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior12::R`](R) reader structure"]
impl crate::Readable for Iprior12Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior12::W`](W) writer structure"]
impl crate::Writable for Iprior12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR12 to value 0"]
impl crate::Resettable for Iprior12Spec {}
