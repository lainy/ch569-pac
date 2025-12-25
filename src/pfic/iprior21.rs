#[doc = "Register `IPRIOR21` reader"]
pub type R = crate::R<Iprior21Spec>;
#[doc = "Register `IPRIOR21` writer"]
pub type W = crate::W<Iprior21Spec>;
#[doc = "Field `IPRIOR21` reader - IPRIOR21"]
pub type Iprior21R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR21` writer - IPRIOR21"]
pub type Iprior21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR21"]
    #[inline(always)]
    pub fn iprior21(&self) -> Iprior21R {
        Iprior21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR21"]
    #[inline(always)]
    pub fn iprior21(&mut self) -> Iprior21W<'_, Iprior21Spec> {
        Iprior21W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior21Spec;
impl crate::RegisterSpec for Iprior21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior21::R`](R) reader structure"]
impl crate::Readable for Iprior21Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior21::W`](W) writer structure"]
impl crate::Writable for Iprior21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR21 to value 0"]
impl crate::Resettable for Iprior21Spec {}
