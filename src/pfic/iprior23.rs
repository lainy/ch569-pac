#[doc = "Register `IPRIOR23` reader"]
pub type R = crate::R<Iprior23Spec>;
#[doc = "Register `IPRIOR23` writer"]
pub type W = crate::W<Iprior23Spec>;
#[doc = "Field `IPRIOR23` reader - IPRIOR23"]
pub type Iprior23R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR23` writer - IPRIOR23"]
pub type Iprior23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR23"]
    #[inline(always)]
    pub fn iprior23(&self) -> Iprior23R {
        Iprior23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR23"]
    #[inline(always)]
    pub fn iprior23(&mut self) -> Iprior23W<'_, Iprior23Spec> {
        Iprior23W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior23Spec;
impl crate::RegisterSpec for Iprior23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior23::R`](R) reader structure"]
impl crate::Readable for Iprior23Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior23::W`](W) writer structure"]
impl crate::Writable for Iprior23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR23 to value 0"]
impl crate::Resettable for Iprior23Spec {}
