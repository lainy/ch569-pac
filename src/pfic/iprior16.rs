#[doc = "Register `IPRIOR16` reader"]
pub type R = crate::R<Iprior16Spec>;
#[doc = "Register `IPRIOR16` writer"]
pub type W = crate::W<Iprior16Spec>;
#[doc = "Field `IPRIOR16` reader - IPRIOR16"]
pub type Iprior16R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR16` writer - IPRIOR16"]
pub type Iprior16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR16"]
    #[inline(always)]
    pub fn iprior16(&self) -> Iprior16R {
        Iprior16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR16"]
    #[inline(always)]
    pub fn iprior16(&mut self) -> Iprior16W<'_, Iprior16Spec> {
        Iprior16W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior16Spec;
impl crate::RegisterSpec for Iprior16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior16::R`](R) reader structure"]
impl crate::Readable for Iprior16Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior16::W`](W) writer structure"]
impl crate::Writable for Iprior16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR16 to value 0"]
impl crate::Resettable for Iprior16Spec {}
