#[doc = "Register `IPRIOR13` reader"]
pub type R = crate::R<Iprior13Spec>;
#[doc = "Register `IPRIOR13` writer"]
pub type W = crate::W<Iprior13Spec>;
#[doc = "Field `IPRIOR13` reader - IPRIOR13"]
pub type Iprior13R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR13` writer - IPRIOR13"]
pub type Iprior13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR13"]
    #[inline(always)]
    pub fn iprior13(&self) -> Iprior13R {
        Iprior13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR13"]
    #[inline(always)]
    pub fn iprior13(&mut self) -> Iprior13W<'_, Iprior13Spec> {
        Iprior13W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior13Spec;
impl crate::RegisterSpec for Iprior13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior13::R`](R) reader structure"]
impl crate::Readable for Iprior13Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior13::W`](W) writer structure"]
impl crate::Writable for Iprior13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR13 to value 0"]
impl crate::Resettable for Iprior13Spec {}
