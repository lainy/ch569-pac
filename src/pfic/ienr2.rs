#[doc = "Register `IENR2` reader"]
pub type R = crate::R<Ienr2Spec>;
#[doc = "Register `IENR2` writer"]
pub type W = crate::W<Ienr2Spec>;
#[doc = "Field `INTEN` reader - INTEN"]
pub type IntenR = crate::FieldReader<u32>;
#[doc = "Field `INTEN` writer - INTEN"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, Ienr2Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ienr2Spec;
impl crate::RegisterSpec for Ienr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ienr2::R`](R) reader structure"]
impl crate::Readable for Ienr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ienr2::W`](W) writer structure"]
impl crate::Writable for Ienr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IENR2 to value 0"]
impl crate::Resettable for Ienr2Spec {}
