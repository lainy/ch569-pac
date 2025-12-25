#[doc = "Register `IPRIOR5` reader"]
pub type R = crate::R<Iprior5Spec>;
#[doc = "Register `IPRIOR5` writer"]
pub type W = crate::W<Iprior5Spec>;
#[doc = "Field `IPRIOR5` reader - IPRIOR5"]
pub type Iprior5R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR5` writer - IPRIOR5"]
pub type Iprior5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR5"]
    #[inline(always)]
    pub fn iprior5(&self) -> Iprior5R {
        Iprior5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR5"]
    #[inline(always)]
    pub fn iprior5(&mut self) -> Iprior5W<'_, Iprior5Spec> {
        Iprior5W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior5Spec;
impl crate::RegisterSpec for Iprior5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior5::R`](R) reader structure"]
impl crate::Readable for Iprior5Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior5::W`](W) writer structure"]
impl crate::Writable for Iprior5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR5 to value 0"]
impl crate::Resettable for Iprior5Spec {}
