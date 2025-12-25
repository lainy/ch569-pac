#[doc = "Register `IPRIOR8` reader"]
pub type R = crate::R<Iprior8Spec>;
#[doc = "Register `IPRIOR8` writer"]
pub type W = crate::W<Iprior8Spec>;
#[doc = "Field `IPRIOR8` reader - IPRIOR8"]
pub type Iprior8R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR8` writer - IPRIOR8"]
pub type Iprior8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR8"]
    #[inline(always)]
    pub fn iprior8(&self) -> Iprior8R {
        Iprior8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR8"]
    #[inline(always)]
    pub fn iprior8(&mut self) -> Iprior8W<'_, Iprior8Spec> {
        Iprior8W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior8Spec;
impl crate::RegisterSpec for Iprior8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior8::R`](R) reader structure"]
impl crate::Readable for Iprior8Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior8::W`](W) writer structure"]
impl crate::Writable for Iprior8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR8 to value 0"]
impl crate::Resettable for Iprior8Spec {}
