#[doc = "Register `IPRIOR56` reader"]
pub type R = crate::R<Iprior56Spec>;
#[doc = "Register `IPRIOR56` writer"]
pub type W = crate::W<Iprior56Spec>;
#[doc = "Field `IPRIOR56` reader - IPRIOR56"]
pub type Iprior56R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR56` writer - IPRIOR56"]
pub type Iprior56W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPRIOR56"]
    #[inline(always)]
    pub fn iprior56(&self) -> Iprior56R {
        Iprior56R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR56"]
    #[inline(always)]
    pub fn iprior56(&mut self) -> Iprior56W<'_, Iprior56Spec> {
        Iprior56W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprior56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprior56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprior56Spec;
impl crate::RegisterSpec for Iprior56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprior56::R`](R) reader structure"]
impl crate::Readable for Iprior56Spec {}
#[doc = "`write(|w| ..)` method takes [`iprior56::W`](W) writer structure"]
impl crate::Writable for Iprior56Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRIOR56 to value 0"]
impl crate::Resettable for Iprior56Spec {}
