#[doc = "Register `IPRIOR28` reader"]
pub type R = crate::R<Iprior28Spec>;
#[doc = "Register `IPRIOR28` writer"]
pub type W = crate::W<Iprior28Spec>;
#[doc = "Field `IPRIOR28` reader - IPRIOR28"]
pub type Iprior28R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR28` writer - IPRIOR28"]
pub type Iprior28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR28"]
    #[inline(always)]
    pub fn iprior28(&self) -> Iprior28R {
        Iprior28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR28"]
    #[inline(always)]
    pub fn iprior28(&mut self) -> Iprior28W<'_, Iprior28Spec> {
        Iprior28W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior28Spec;
impl crate::RegisterSpec for Iprior28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior28::R`](R) reader structure"]
impl crate::Readable for Iprior28Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior28::W`](W) writer structure"]
impl crate::Writable for Iprior28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR28 to value 0"]
impl crate::Resettable for Iprior28Spec {}
