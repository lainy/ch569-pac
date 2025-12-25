#[doc = "Register `IPRIOR32` reader"]
pub type R = crate::R<Iprior32Spec>;
#[doc = "Register `IPRIOR32` writer"]
pub type W = crate::W<Iprior32Spec>;
#[doc = "Field `IPRIOR32` reader - IPRIOR32"]
pub type Iprior32R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR32` writer - IPRIOR32"]
pub type Iprior32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR32"]
    #[inline(always)]
    pub fn iprior32(&self) -> Iprior32R {
        Iprior32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR32"]
    #[inline(always)]
    pub fn iprior32(&mut self) -> Iprior32W<'_, Iprior32Spec> {
        Iprior32W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior32Spec;
impl crate::RegisterSpec for Iprior32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior32::R`](R) reader structure"]
impl crate::Readable for Iprior32Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior32::W`](W) writer structure"]
impl crate::Writable for Iprior32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR32 to value 0"]
impl crate::Resettable for Iprior32Spec {}
