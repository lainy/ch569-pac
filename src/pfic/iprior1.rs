#[doc = "Register `IPRIOR1` reader"]
pub type R = crate::R<Iprior1Spec>;
#[doc = "Register `IPRIOR1` writer"]
pub type W = crate::W<Iprior1Spec>;
#[doc = "Field `IPRIOR1` reader - IPRIOR1"]
pub type Iprior1R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR1` writer - IPRIOR1"]
pub type Iprior1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR1"]
    #[inline(always)]
    pub fn iprior1(&self) -> Iprior1R {
        Iprior1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR1"]
    #[inline(always)]
    pub fn iprior1(&mut self) -> Iprior1W<'_, Iprior1Spec> {
        Iprior1W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior1Spec;
impl crate::RegisterSpec for Iprior1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior1::R`](R) reader structure"]
impl crate::Readable for Iprior1Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior1::W`](W) writer structure"]
impl crate::Writable for Iprior1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR1 to value 0"]
impl crate::Resettable for Iprior1Spec {}
