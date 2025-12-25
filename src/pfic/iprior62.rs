#[doc = "Register `IPRIOR62` reader"]
pub type R = crate::R<Iprior62Spec>;
#[doc = "Register `IPRIOR62` writer"]
pub type W = crate::W<Iprior62Spec>;
#[doc = "Field `IPRIOR62` reader - IPRIOR62"]
pub type Iprior62R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR62` writer - IPRIOR62"]
pub type Iprior62W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR62"]
    #[inline(always)]
    pub fn iprior62(&self) -> Iprior62R {
        Iprior62R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR62"]
    #[inline(always)]
    pub fn iprior62(&mut self) -> Iprior62W<'_, Iprior62Spec> {
        Iprior62W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior62::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior62::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior62Spec;
impl crate::RegisterSpec for Iprior62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior62::R`](R) reader structure"]
impl crate::Readable for Iprior62Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior62::W`](W) writer structure"]
impl crate::Writable for Iprior62Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR62 to value 0"]
impl crate::Resettable for Iprior62Spec {}
