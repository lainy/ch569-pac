#[doc = "Register `IRER1` reader"]
pub type R = crate::R<Irer1Spec>;
#[doc = "Register `IRER1` writer"]
pub type W = crate::W<Irer1Spec>;
#[doc = "Field `INTRSET` reader - INTRSET"]
pub type IntrsetR = crate::FieldReader<u32>;
#[doc = "Field `INTRSET` writer - INTRSET"]
pub type IntrsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - INTRSET"]
    #[inline(always)]
    pub fn intrset(&self) -> IntrsetR {
        IntrsetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - INTRSET"]
    #[inline(always)]
    pub fn intrset(&mut self) -> IntrsetW<'_, Irer1Spec> {
        IntrsetW::new(self, 12)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irer1Spec;
impl crate::RegisterSpec for Irer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irer1::R`](R) reader structure"]
impl crate::Readable for Irer1Spec {}
#[doc = "`write(|w| ..)` method takes [`irer1::W`](W) writer structure"]
impl crate::Writable for Irer1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRER1 to value 0"]
impl crate::Resettable for Irer1Spec {}
