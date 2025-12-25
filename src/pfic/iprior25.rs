#[doc = "Register `IPRIOR25` reader"]
pub type R = crate::R<Iprior25Spec>;
#[doc = "Register `IPRIOR25` writer"]
pub type W = crate::W<Iprior25Spec>;
#[doc = "Field `IPRIOR25` reader - IPRIOR25"]
pub type Iprior25R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR25` writer - IPRIOR25"]
pub type Iprior25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR25"]
    #[inline(always)]
    pub fn iprior25(&self) -> Iprior25R {
        Iprior25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR25"]
    #[inline(always)]
    pub fn iprior25(&mut self) -> Iprior25W<'_, Iprior25Spec> {
        Iprior25W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior25Spec;
impl crate::RegisterSpec for Iprior25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior25::R`](R) reader structure"]
impl crate::Readable for Iprior25Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior25::W`](W) writer structure"]
impl crate::Writable for Iprior25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR25 to value 0"]
impl crate::Resettable for Iprior25Spec {}
