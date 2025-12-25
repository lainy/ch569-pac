#[doc = "Register `IRER2` reader"]
pub type R = crate::R<Irer2Spec>;
#[doc = "Register `IRER2` writer"]
pub type W = crate::W<Irer2Spec>;
#[doc = "Field `INTRSET` reader - INTRSET"]
pub type IntrsetR = crate::FieldReader<u32>;
#[doc = "Field `INTRSET` writer - INTRSET"]
pub type IntrsetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - INTRSET"]
    #[inline(always)]
    pub fn intrset(&self) -> IntrsetR {
        IntrsetR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - INTRSET"]
    #[inline(always)]
    pub fn intrset(&mut self) -> IntrsetW<'_, Irer2Spec> {
        IntrsetW::new(self, 0)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irer2Spec;
impl crate::RegisterSpec for Irer2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irer2::R`](R) reader structure"]
impl crate::Readable for Irer2Spec {}
#[doc = "`write(|w| ..)` method takes [`irer2::W`](W) writer structure"]
impl crate::Writable for Irer2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRER2 to value 0"]
impl crate::Resettable for Irer2Spec {}
