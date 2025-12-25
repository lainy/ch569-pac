#[doc = "Register `IPRIOR49` reader"]
pub type R = crate::R<Iprior49Spec>;
#[doc = "Register `IPRIOR49` writer"]
pub type W = crate::W<Iprior49Spec>;
#[doc = "Field `IPRIOR49` reader - IPRIOR49"]
pub type Iprior49R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR49` writer - IPRIOR49"]
pub type Iprior49W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR49"]
    #[inline(always)]
    pub fn iprior49(&self) -> Iprior49R {
        Iprior49R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR49"]
    #[inline(always)]
    pub fn iprior49(&mut self) -> Iprior49W<'_, Iprior49Spec> {
        Iprior49W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior49Spec;
impl crate::RegisterSpec for Iprior49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior49::R`](R) reader structure"]
impl crate::Readable for Iprior49Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior49::W`](W) writer structure"]
impl crate::Writable for Iprior49Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR49 to value 0"]
impl crate::Resettable for Iprior49Spec {}
