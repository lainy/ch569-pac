#[doc = "Register `R16_UEP2_MAX_LEN_R16_UH_MAX_LEN` reader"]
pub type R = crate::R<R16Uep2MaxLenR16UhMaxLenSpec>;
#[doc = "Register `R16_UEP2_MAX_LEN_R16_UH_MAX_LEN` writer"]
pub type W = crate::W<R16Uep2MaxLenR16UhMaxLenSpec>;
#[doc = "Field `UEP2_MAX_LEN_UH_MAX_LEN` reader - endpoint 2 receive max length / USB host receive max packet length register"]
pub type Uep2MaxLenUhMaxLenR = crate::FieldReader<u16>;
#[doc = "Field `UEP2_MAX_LEN_UH_MAX_LEN` writer - endpoint 2 receive max length / USB host receive max packet length register"]
pub type Uep2MaxLenUhMaxLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 2 receive max length / USB host receive max packet length register"]
    #[inline(always)]
    pub fn uep2_max_len_uh_max_len(&self) -> Uep2MaxLenUhMaxLenR {
        Uep2MaxLenUhMaxLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 2 receive max length / USB host receive max packet length register"]
    #[inline(always)]
    pub fn uep2_max_len_uh_max_len(
        &mut self,
    ) -> Uep2MaxLenUhMaxLenW<'_, R16Uep2MaxLenR16UhMaxLenSpec> {
        Uep2MaxLenUhMaxLenW::new(self, 0)
    }
}
#[doc = "endpoint 2 receive max length / USB host receive max packet length register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep2_max_len_r16_uh_max_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep2_max_len_r16_uh_max_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep2MaxLenR16UhMaxLenSpec;
impl crate::RegisterSpec for R16Uep2MaxLenR16UhMaxLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep2_max_len_r16_uh_max_len::R`](R) reader structure"]
impl crate::Readable for R16Uep2MaxLenR16UhMaxLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep2_max_len_r16_uh_max_len::W`](W) writer structure"]
impl crate::Writable for R16Uep2MaxLenR16UhMaxLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP2_MAX_LEN_R16_UH_MAX_LEN to value 0"]
impl crate::Resettable for R16Uep2MaxLenR16UhMaxLenSpec {}
