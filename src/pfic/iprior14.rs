#[doc = "Register `IPRIOR14` reader"]
pub type R = crate::R<Iprior14Spec>;
#[doc = "Register `IPRIOR14` writer"]
pub type W = crate::W<Iprior14Spec>;
#[doc = "Field `IPRIOR14` reader - IPRIOR14"]
pub type Iprior14R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR14` writer - IPRIOR14"]
pub type Iprior14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR14"]
    #[inline(always)]
    pub fn iprior14(&self) -> Iprior14R {
        Iprior14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR14"]
    #[inline(always)]
    pub fn iprior14(&mut self) -> Iprior14W<'_, Iprior14Spec> {
        Iprior14W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior14Spec;
impl crate::RegisterSpec for Iprior14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior14::R`](R) reader structure"]
impl crate::Readable for Iprior14Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior14::W`](W) writer structure"]
impl crate::Writable for Iprior14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR14 to value 0"]
impl crate::Resettable for Iprior14Spec {}
