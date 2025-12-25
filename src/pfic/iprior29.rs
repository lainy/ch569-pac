#[doc = "Register `IPRIOR29` reader"]
pub type R = crate::R<Iprior29Spec>;
#[doc = "Register `IPRIOR29` writer"]
pub type W = crate::W<Iprior29Spec>;
#[doc = "Field `IPRIOR29` reader - IPRIOR29"]
pub type Iprior29R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR29` writer - IPRIOR29"]
pub type Iprior29W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR29"]
    #[inline(always)]
    pub fn iprior29(&self) -> Iprior29R {
        Iprior29R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR29"]
    #[inline(always)]
    pub fn iprior29(&mut self) -> Iprior29W<'_, Iprior29Spec> {
        Iprior29W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior29Spec;
impl crate::RegisterSpec for Iprior29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior29::R`](R) reader structure"]
impl crate::Readable for Iprior29Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior29::W`](W) writer structure"]
impl crate::Writable for Iprior29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR29 to value 0"]
impl crate::Resettable for Iprior29Spec {}
