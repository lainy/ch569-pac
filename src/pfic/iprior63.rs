#[doc = "Register `IPRIOR63` reader"]
pub type R = crate::R<Iprior63Spec>;
#[doc = "Register `IPRIOR63` writer"]
pub type W = crate::W<Iprior63Spec>;
#[doc = "Field `IPRIOR63` reader - IPRIOR63"]
pub type Iprior63R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR63` writer - IPRIOR63"]
pub type Iprior63W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR63"]
    #[inline(always)]
    pub fn iprior63(&self) -> Iprior63R {
        Iprior63R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR63"]
    #[inline(always)]
    pub fn iprior63(&mut self) -> Iprior63W<'_, Iprior63Spec> {
        Iprior63W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior63::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior63::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior63Spec;
impl crate::RegisterSpec for Iprior63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior63::R`](R) reader structure"]
impl crate::Readable for Iprior63Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior63::W`](W) writer structure"]
impl crate::Writable for Iprior63Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR63 to value 0"]
impl crate::Resettable for Iprior63Spec {}
