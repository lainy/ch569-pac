#[doc = "Register `IPRIOR6` reader"]
pub type R = crate::R<Iprior6Spec>;
#[doc = "Register `IPRIOR6` writer"]
pub type W = crate::W<Iprior6Spec>;
#[doc = "Field `IPRIOR6` reader - IPRIOR6"]
pub type Iprior6R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR6` writer - IPRIOR6"]
pub type Iprior6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR6"]
    #[inline(always)]
    pub fn iprior6(&self) -> Iprior6R {
        Iprior6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR6"]
    #[inline(always)]
    pub fn iprior6(&mut self) -> Iprior6W<'_, Iprior6Spec> {
        Iprior6W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior6Spec;
impl crate::RegisterSpec for Iprior6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior6::R`](R) reader structure"]
impl crate::Readable for Iprior6Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior6::W`](W) writer structure"]
impl crate::Writable for Iprior6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR6 to value 0"]
impl crate::Resettable for Iprior6Spec {}
