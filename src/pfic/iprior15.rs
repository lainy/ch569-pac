#[doc = "Register `IPRIOR15` reader"]
pub type R = crate::R<Iprior15Spec>;
#[doc = "Register `IPRIOR15` writer"]
pub type W = crate::W<Iprior15Spec>;
#[doc = "Field `IPRIOR15` reader - IPRIOR15"]
pub type Iprior15R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR15` writer - IPRIOR15"]
pub type Iprior15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR15"]
    #[inline(always)]
    pub fn iprior15(&self) -> Iprior15R {
        Iprior15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR15"]
    #[inline(always)]
    pub fn iprior15(&mut self) -> Iprior15W<'_, Iprior15Spec> {
        Iprior15W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior15Spec;
impl crate::RegisterSpec for Iprior15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior15::R`](R) reader structure"]
impl crate::Readable for Iprior15Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior15::W`](W) writer structure"]
impl crate::Writable for Iprior15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR15 to value 0"]
impl crate::Resettable for Iprior15Spec {}
