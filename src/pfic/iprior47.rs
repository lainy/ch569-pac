#[doc = "Register `IPRIOR47` reader"]
pub type R = crate::R<Iprior47Spec>;
#[doc = "Register `IPRIOR47` writer"]
pub type W = crate::W<Iprior47Spec>;
#[doc = "Field `IPRIOR47` reader - IPRIOR47"]
pub type Iprior47R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR47` writer - IPRIOR47"]
pub type Iprior47W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR47"]
    #[inline(always)]
    pub fn iprior47(&self) -> Iprior47R {
        Iprior47R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR47"]
    #[inline(always)]
    pub fn iprior47(&mut self) -> Iprior47W<'_, Iprior47Spec> {
        Iprior47W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior47Spec;
impl crate::RegisterSpec for Iprior47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior47::R`](R) reader structure"]
impl crate::Readable for Iprior47Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior47::W`](W) writer structure"]
impl crate::Writable for Iprior47Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR47 to value 0"]
impl crate::Resettable for Iprior47Spec {}
