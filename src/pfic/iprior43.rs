#[doc = "Register `IPRIOR43` reader"]
pub type R = crate::R<Iprior43Spec>;
#[doc = "Register `IPRIOR43` writer"]
pub type W = crate::W<Iprior43Spec>;
#[doc = "Field `IPRIOR43` reader - IPRIOR43"]
pub type Iprior43R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR43` writer - IPRIOR43"]
pub type Iprior43W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR43"]
    #[inline(always)]
    pub fn iprior43(&self) -> Iprior43R {
        Iprior43R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR43"]
    #[inline(always)]
    pub fn iprior43(&mut self) -> Iprior43W<'_, Iprior43Spec> {
        Iprior43W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior43Spec;
impl crate::RegisterSpec for Iprior43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior43::R`](R) reader structure"]
impl crate::Readable for Iprior43Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior43::W`](W) writer structure"]
impl crate::Writable for Iprior43Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR43 to value 0"]
impl crate::Resettable for Iprior43Spec {}
