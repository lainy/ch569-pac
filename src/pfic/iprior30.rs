#[doc = "Register `IPRIOR30` reader"]
pub type R = crate::R<Iprior30Spec>;
#[doc = "Register `IPRIOR30` writer"]
pub type W = crate::W<Iprior30Spec>;
#[doc = "Field `IPRIOR30` reader - IPRIOR30"]
pub type Iprior30R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR30` writer - IPRIOR30"]
pub type Iprior30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR30"]
    #[inline(always)]
    pub fn iprior30(&self) -> Iprior30R {
        Iprior30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR30"]
    #[inline(always)]
    pub fn iprior30(&mut self) -> Iprior30W<'_, Iprior30Spec> {
        Iprior30W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior30Spec;
impl crate::RegisterSpec for Iprior30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior30::R`](R) reader structure"]
impl crate::Readable for Iprior30Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior30::W`](W) writer structure"]
impl crate::Writable for Iprior30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR30 to value 0"]
impl crate::Resettable for Iprior30Spec {}
