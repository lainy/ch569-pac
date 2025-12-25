#[doc = "Register `IACTR2` reader"]
pub type R = crate::R<Iactr2Spec>;
#[doc = "Register `IACTR2` writer"]
pub type W = crate::W<Iactr2Spec>;
#[doc = "Field `IACTS` reader - IACTS"]
pub type IactsR = crate::FieldReader<u32>;
#[doc = "Field `IACTS` writer - IACTS"]
pub type IactsW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - IACTS"]
    #[inline(always)]
    pub fn iacts(&self) -> IactsR {
        IactsR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - IACTS"]
    #[inline(always)]
    pub fn iacts(&mut self) -> IactsW<'_, Iactr2Spec> {
        IactsW::new(self, 0)
    }
}
#[doc = "Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iactr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iactr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iactr2Spec;
impl crate::RegisterSpec for Iactr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iactr2::R`](R) reader structure"]
impl crate::Readable for Iactr2Spec {}
#[doc = "`write(|w| ..)` method takes [`iactr2::W`](W) writer structure"]
impl crate::Writable for Iactr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IACTR2 to value 0"]
impl crate::Resettable for Iactr2Spec {}
