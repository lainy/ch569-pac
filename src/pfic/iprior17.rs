#[doc = "Register `IPRIOR17` reader"]
pub type R = crate::R<Iprior17Spec>;
#[doc = "Register `IPRIOR17` writer"]
pub type W = crate::W<Iprior17Spec>;
#[doc = "Field `IPRIOR17` reader - IPRIOR17"]
pub type Iprior17R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR17` writer - IPRIOR17"]
pub type Iprior17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR17"]
    #[inline(always)]
    pub fn iprior17(&self) -> Iprior17R {
        Iprior17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR17"]
    #[inline(always)]
    pub fn iprior17(&mut self) -> Iprior17W<'_, Iprior17Spec> {
        Iprior17W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior17Spec;
impl crate::RegisterSpec for Iprior17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior17::R`](R) reader structure"]
impl crate::Readable for Iprior17Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior17::W`](W) writer structure"]
impl crate::Writable for Iprior17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR17 to value 0"]
impl crate::Resettable for Iprior17Spec {}
