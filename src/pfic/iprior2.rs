#[doc = "Register `IPRIOR2` reader"]
pub type R = crate::R<Iprior2Spec>;
#[doc = "Register `IPRIOR2` writer"]
pub type W = crate::W<Iprior2Spec>;
#[doc = "Field `IPRIOR2` reader - IPRIOR2"]
pub type Iprior2R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR2` writer - IPRIOR2"]
pub type Iprior2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR2"]
    #[inline(always)]
    pub fn iprior2(&self) -> Iprior2R {
        Iprior2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR2"]
    #[inline(always)]
    pub fn iprior2(&mut self) -> Iprior2W<'_, Iprior2Spec> {
        Iprior2W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior2Spec;
impl crate::RegisterSpec for Iprior2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior2::R`](R) reader structure"]
impl crate::Readable for Iprior2Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior2::W`](W) writer structure"]
impl crate::Writable for Iprior2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR2 to value 0"]
impl crate::Resettable for Iprior2Spec {}
