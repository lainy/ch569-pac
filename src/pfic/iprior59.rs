#[doc = "Register `IPRIOR59` reader"]
pub type R = crate::R<Iprior59Spec>;
#[doc = "Register `IPRIOR59` writer"]
pub type W = crate::W<Iprior59Spec>;
#[doc = "Field `IPRIOR59` reader - IPRIOR59"]
pub type Iprior59R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR59` writer - IPRIOR59"]
pub type Iprior59W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR59"]
    #[inline(always)]
    pub fn iprior59(&self) -> Iprior59R {
        Iprior59R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR59"]
    #[inline(always)]
    pub fn iprior59(&mut self) -> Iprior59W<'_, Iprior59Spec> {
        Iprior59W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior59Spec;
impl crate::RegisterSpec for Iprior59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior59::R`](R) reader structure"]
impl crate::Readable for Iprior59Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior59::W`](W) writer structure"]
impl crate::Writable for Iprior59Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR59 to value 0"]
impl crate::Resettable for Iprior59Spec {}
