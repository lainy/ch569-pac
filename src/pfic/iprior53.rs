#[doc = "Register `IPRIOR53` reader"]
pub type R = crate::R<Iprior53Spec>;
#[doc = "Register `IPRIOR53` writer"]
pub type W = crate::W<Iprior53Spec>;
#[doc = "Field `IPRIOR53` reader - IPRIOR53"]
pub type Iprior53R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR53` writer - IPRIOR53"]
pub type Iprior53W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR53"]
    #[inline(always)]
    pub fn iprior53(&self) -> Iprior53R {
        Iprior53R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR53"]
    #[inline(always)]
    pub fn iprior53(&mut self) -> Iprior53W<'_, Iprior53Spec> {
        Iprior53W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior53Spec;
impl crate::RegisterSpec for Iprior53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior53::R`](R) reader structure"]
impl crate::Readable for Iprior53Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior53::W`](W) writer structure"]
impl crate::Writable for Iprior53Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR53 to value 0"]
impl crate::Resettable for Iprior53Spec {}
