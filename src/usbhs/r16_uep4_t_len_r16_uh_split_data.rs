#[doc = "Register `R16_UEP4_T_LEN_R16_UH_SPLIT_DATA` reader"]
pub type R = crate::R<R16Uep4TLenR16UhSplitDataSpec>;
#[doc = "Register `R16_UEP4_T_LEN_R16_UH_SPLIT_DATA` writer"]
pub type W = crate::W<R16Uep4TLenR16UhSplitDataSpec>;
#[doc = "Field `UEP4_T_LEN_UH_SPLIT_DATA` reader - endpoint 4 transmittal length / USB host Tx SPLIT packet data"]
pub type Uep4TLenUhSplitDataR = crate::FieldReader<u16>;
#[doc = "Field `UEP4_T_LEN_UH_SPLIT_DATA` writer - endpoint 4 transmittal length / USB host Tx SPLIT packet data"]
pub type Uep4TLenUhSplitDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 4 transmittal length / USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub fn uep4_t_len_uh_split_data(&self) -> Uep4TLenUhSplitDataR {
        Uep4TLenUhSplitDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 4 transmittal length / USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub fn uep4_t_len_uh_split_data(
        &mut self,
    ) -> Uep4TLenUhSplitDataW<'_, R16Uep4TLenR16UhSplitDataSpec> {
        Uep4TLenUhSplitDataW::new(self, 0)
    }
}
#[doc = "endpoint 4 transmittal length / USB host Tx SPLIT packet data\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep4_t_len_r16_uh_split_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep4_t_len_r16_uh_split_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep4TLenR16UhSplitDataSpec;
impl crate::RegisterSpec for R16Uep4TLenR16UhSplitDataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep4_t_len_r16_uh_split_data::R`](R) reader structure"]
impl crate::Readable for R16Uep4TLenR16UhSplitDataSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep4_t_len_r16_uh_split_data::W`](W) writer structure"]
impl crate::Writable for R16Uep4TLenR16UhSplitDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP4_T_LEN_R16_UH_SPLIT_DATA to value 0"]
impl crate::Resettable for R16Uep4TLenR16UhSplitDataSpec {}
