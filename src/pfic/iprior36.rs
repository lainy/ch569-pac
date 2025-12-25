#[doc = "Register `IPRIOR36` reader"]
pub type R = crate::R<Iprior36Spec>;
#[doc = "Register `IPRIOR36` writer"]
pub type W = crate::W<Iprior36Spec>;
#[doc = "Field `IPRIOR36` reader - IPRIOR36"]
pub type Iprior36R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR36` writer - IPRIOR36"]
pub type Iprior36W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR36"]
    #[inline(always)]
    pub fn iprior36(&self) -> Iprior36R {
        Iprior36R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR36"]
    #[inline(always)]
    pub fn iprior36(&mut self) -> Iprior36W<'_, Iprior36Spec> {
        Iprior36W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior36Spec;
impl crate::RegisterSpec for Iprior36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior36::R`](R) reader structure"]
impl crate::Readable for Iprior36Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior36::W`](W) writer structure"]
impl crate::Writable for Iprior36Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR36 to value 0"]
impl crate::Resettable for Iprior36Spec {}
