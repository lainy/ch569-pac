#[doc = "Register `R16_UEP1_MAX_LEN` reader"]
pub type R = crate::R<R16Uep1MaxLenSpec>;
#[doc = "Register `R16_UEP1_MAX_LEN` writer"]
pub type W = crate::W<R16Uep1MaxLenSpec>;
#[doc = "Field `UEP1_MAX_LEN` reader - endpoint 1 receive max length"]
pub type Uep1MaxLenR = crate::FieldReader<u16>;
#[doc = "Field `UEP1_MAX_LEN` writer - endpoint 1 receive max length"]
pub type Uep1MaxLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 1 receive max length"]
    #[inline(always)]
    pub fn uep1_max_len(&self) -> Uep1MaxLenR {
        Uep1MaxLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 1 receive max length"]
    #[inline(always)]
    pub fn uep1_max_len(&mut self) -> Uep1MaxLenW<'_, R16Uep1MaxLenSpec> {
        Uep1MaxLenW::new(self, 0)
    }
}
#[doc = "endpoint 1 receive max length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep1_max_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep1_max_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep1MaxLenSpec;
impl crate::RegisterSpec for R16Uep1MaxLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep1_max_len::R`](R) reader structure"]
impl crate::Readable for R16Uep1MaxLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep1_max_len::W`](W) writer structure"]
impl crate::Writable for R16Uep1MaxLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP1_MAX_LEN to value 0"]
impl crate::Resettable for R16Uep1MaxLenSpec {}
