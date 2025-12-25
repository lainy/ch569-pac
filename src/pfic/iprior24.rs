#[doc = "Register `IPRIOR24` reader"]
pub type R = crate::R<Iprior24Spec>;
#[doc = "Register `IPRIOR24` writer"]
pub type W = crate::W<Iprior24Spec>;
#[doc = "Field `IPRIOR24` reader - IPRIOR24"]
pub type Iprior24R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR24` writer - IPRIOR24"]
pub type Iprior24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR24"]
    #[inline(always)]
    pub fn iprior24(&self) -> Iprior24R {
        Iprior24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR24"]
    #[inline(always)]
    pub fn iprior24(&mut self) -> Iprior24W<'_, Iprior24Spec> {
        Iprior24W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior24Spec;
impl crate::RegisterSpec for Iprior24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior24::R`](R) reader structure"]
impl crate::Readable for Iprior24Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior24::W`](W) writer structure"]
impl crate::Writable for Iprior24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR24 to value 0"]
impl crate::Resettable for Iprior24Spec {}
