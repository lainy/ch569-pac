#[doc = "Register `IPRIOR51` reader"]
pub type R = crate::R<Iprior51Spec>;
#[doc = "Register `IPRIOR51` writer"]
pub type W = crate::W<Iprior51Spec>;
#[doc = "Field `IPRIOR51` reader - IPRIOR51"]
pub type Iprior51R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR51` writer - IPRIOR51"]
pub type Iprior51W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR51"]
    #[inline(always)]
    pub fn iprior51(&self) -> Iprior51R {
        Iprior51R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR51"]
    #[inline(always)]
    pub fn iprior51(&mut self) -> Iprior51W<'_, Iprior51Spec> {
        Iprior51W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior51Spec;
impl crate::RegisterSpec for Iprior51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior51::R`](R) reader structure"]
impl crate::Readable for Iprior51Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior51::W`](W) writer structure"]
impl crate::Writable for Iprior51Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR51 to value 0"]
impl crate::Resettable for Iprior51Spec {}
