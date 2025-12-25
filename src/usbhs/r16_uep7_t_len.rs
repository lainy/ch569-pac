#[doc = "Register `R16_UEP7_T_LEN` reader"]
pub type R = crate::R<R16Uep7TLenSpec>;
#[doc = "Register `R16_UEP7_T_LEN` writer"]
pub type W = crate::W<R16Uep7TLenSpec>;
#[doc = "Field `UEP7_T_LEN` reader - endpoint 7 transmittal length"]
pub type Uep7TLenR = crate::FieldReader<u16>;
#[doc = "Field `UEP7_T_LEN` writer - endpoint 7 transmittal length"]
pub type Uep7TLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 7 transmittal length"]
    #[inline(always)]
    pub fn uep7_t_len(&self) -> Uep7TLenR {
        Uep7TLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 7 transmittal length"]
    #[inline(always)]
    pub fn uep7_t_len(&mut self) -> Uep7TLenW<'_, R16Uep7TLenSpec> {
        Uep7TLenW::new(self, 0)
    }
}
#[doc = "endpoint 7 transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep7_t_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep7_t_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep7TLenSpec;
impl crate::RegisterSpec for R16Uep7TLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep7_t_len::R`](R) reader structure"]
impl crate::Readable for R16Uep7TLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep7_t_len::W`](W) writer structure"]
impl crate::Writable for R16Uep7TLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP7_T_LEN to value 0"]
impl crate::Resettable for R16Uep7TLenSpec {}
