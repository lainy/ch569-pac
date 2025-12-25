#[doc = "Register `IPRIOR3` reader"]
pub type R = crate::R<Iprior3Spec>;
#[doc = "Register `IPRIOR3` writer"]
pub type W = crate::W<Iprior3Spec>;
#[doc = "Field `IPRIOR3` reader - IPRIOR3"]
pub type Iprior3R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR3` writer - IPRIOR3"]
pub type Iprior3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR3"]
    #[inline(always)]
    pub fn iprior3(&self) -> Iprior3R {
        Iprior3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR3"]
    #[inline(always)]
    pub fn iprior3(&mut self) -> Iprior3W<'_, Iprior3Spec> {
        Iprior3W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior3Spec;
impl crate::RegisterSpec for Iprior3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior3::R`](R) reader structure"]
impl crate::Readable for Iprior3Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior3::W`](W) writer structure"]
impl crate::Writable for Iprior3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR3 to value 0"]
impl crate::Resettable for Iprior3Spec {}
