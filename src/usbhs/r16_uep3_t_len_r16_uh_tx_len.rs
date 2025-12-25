#[doc = "Register `R16_UEP3_T_LEN_R16_UH_TX_LEN` reader"]
pub type R = crate::R<R16Uep3TLenR16UhTxLenSpec>;
#[doc = "Register `R16_UEP3_T_LEN_R16_UH_TX_LEN` writer"]
pub type W = crate::W<R16Uep3TLenR16UhTxLenSpec>;
#[doc = "Field `UEP3_T_LEN_UH_TX_LEN` reader - endpoint 3 transmittal length / host transmittal endpoint transmittal length"]
pub type Uep3TLenUhTxLenR = crate::FieldReader<u16>;
#[doc = "Field `UEP3_T_LEN_UH_TX_LEN` writer - endpoint 3 transmittal length / host transmittal endpoint transmittal length"]
pub type Uep3TLenUhTxLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 3 transmittal length / host transmittal endpoint transmittal length"]
    #[inline(always)]
    pub fn uep3_t_len_uh_tx_len(&self) -> Uep3TLenUhTxLenR {
        Uep3TLenUhTxLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 3 transmittal length / host transmittal endpoint transmittal length"]
    #[inline(always)]
    pub fn uep3_t_len_uh_tx_len(&mut self) -> Uep3TLenUhTxLenW<'_, R16Uep3TLenR16UhTxLenSpec> {
        Uep3TLenUhTxLenW::new(self, 0)
    }
}
#[doc = "endpoint 3 transmittal length / host transmittal endpoint transmittal length\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep3_t_len_r16_uh_tx_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep3_t_len_r16_uh_tx_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep3TLenR16UhTxLenSpec;
impl crate::RegisterSpec for R16Uep3TLenR16UhTxLenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep3_t_len_r16_uh_tx_len::R`](R) reader structure"]
impl crate::Readable for R16Uep3TLenR16UhTxLenSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep3_t_len_r16_uh_tx_len::W`](W) writer structure"]
impl crate::Writable for R16Uep3TLenR16UhTxLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP3_T_LEN_R16_UH_TX_LEN to value 0"]
impl crate::Resettable for R16Uep3TLenR16UhTxLenSpec {}
