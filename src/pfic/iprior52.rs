#[doc = "Register `IPRIOR52` reader"]
pub type R = crate::R<Iprior52Spec>;
#[doc = "Register `IPRIOR52` writer"]
pub type W = crate::W<Iprior52Spec>;
#[doc = "Field `IPRIOR52` reader - IPRIOR52"]
pub type Iprior52R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR52` writer - IPRIOR52"]
pub type Iprior52W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR52"]
    #[inline(always)]
    pub fn iprior52(&self) -> Iprior52R {
        Iprior52R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR52"]
    #[inline(always)]
    pub fn iprior52(&mut self) -> Iprior52W<'_, Iprior52Spec> {
        Iprior52W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior52Spec;
impl crate::RegisterSpec for Iprior52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior52::R`](R) reader structure"]
impl crate::Readable for Iprior52Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior52::W`](W) writer structure"]
impl crate::Writable for Iprior52Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR52 to value 0"]
impl crate::Resettable for Iprior52Spec {}
