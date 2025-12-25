#[doc = "Register `IPRIOR19` reader"]
pub type R = crate::R<Iprior19Spec>;
#[doc = "Register `IPRIOR19` writer"]
pub type W = crate::W<Iprior19Spec>;
#[doc = "Field `IPRIOR19` reader - IPRIOR19"]
pub type Iprior19R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR19` writer - IPRIOR19"]
pub type Iprior19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR19"]
    #[inline(always)]
    pub fn iprior19(&self) -> Iprior19R {
        Iprior19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR19"]
    #[inline(always)]
    pub fn iprior19(&mut self) -> Iprior19W<'_, Iprior19Spec> {
        Iprior19W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior19Spec;
impl crate::RegisterSpec for Iprior19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior19::R`](R) reader structure"]
impl crate::Readable for Iprior19Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior19::W`](W) writer structure"]
impl crate::Writable for Iprior19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR19 to value 0"]
impl crate::Resettable for Iprior19Spec {}
