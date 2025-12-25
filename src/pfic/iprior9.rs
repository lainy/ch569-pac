#[doc = "Register `IPRIOR9` reader"]
pub type R = crate::R<Iprior9Spec>;
#[doc = "Register `IPRIOR9` writer"]
pub type W = crate::W<Iprior9Spec>;
#[doc = "Field `IPRIOR9` reader - IPRIOR9"]
pub type Iprior9R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR9` writer - IPRIOR9"]
pub type Iprior9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR9"]
    #[inline(always)]
    pub fn iprior9(&self) -> Iprior9R {
        Iprior9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR9"]
    #[inline(always)]
    pub fn iprior9(&mut self) -> Iprior9W<'_, Iprior9Spec> {
        Iprior9W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior9Spec;
impl crate::RegisterSpec for Iprior9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior9::R`](R) reader structure"]
impl crate::Readable for Iprior9Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior9::W`](W) writer structure"]
impl crate::Writable for Iprior9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR9 to value 0"]
impl crate::Resettable for Iprior9Spec {}
