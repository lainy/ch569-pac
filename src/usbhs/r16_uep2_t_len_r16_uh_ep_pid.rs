#[doc = "Register `R16_UEP2_T_LEN_R16_UH_EP_PID` reader"]
pub type R = crate::R<R16Uep2TLenR16UhEpPidSpec>;
#[doc = "Register `R16_UEP2_T_LEN_R16_UH_EP_PID` writer"]
pub type W = crate::W<R16Uep2TLenR16UhEpPidSpec>;
#[doc = "Field `RB_UH_EPNUM_MASK_UEP2_T_LEN_0_3` reader - The endpoint number of the target of this operation"]
pub type RbUhEpnumMaskUep2TLen0_3R = crate::FieldReader;
#[doc = "Field `RB_UH_EPNUM_MASK_UEP2_T_LEN_0_3` writer - The endpoint number of the target of this operation"]
pub type RbUhEpnumMaskUep2TLen0_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RB_UH_TOKEN_MASK_UEP2_T_LEN_4_7` reader - The token PID packet identification of this USB transfer transaction"]
pub type RbUhTokenMaskUep2TLen4_7R = crate::FieldReader;
#[doc = "Field `RB_UH_TOKEN_MASK_UEP2_T_LEN_4_7` writer - The token PID packet identification of this USB transfer transaction"]
pub type RbUhTokenMaskUep2TLen4_7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UEP2_T_LEN_8_15` reader - endpoint 2 transmittal length"]
pub type Uep2TLen8_15R = crate::FieldReader;
#[doc = "Field `UEP2_T_LEN_8_15` writer - endpoint 2 transmittal length"]
pub type Uep2TLen8_15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - The endpoint number of the target of this operation"]
    #[inline(always)]
    pub fn rb_uh_epnum_mask_uep2_t_len_0_3(&self) -> RbUhEpnumMaskUep2TLen0_3R {
        RbUhEpnumMaskUep2TLen0_3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The token PID packet identification of this USB transfer transaction"]
    #[inline(always)]
    pub fn rb_uh_token_mask_uep2_t_len_4_7(&self) -> RbUhTokenMaskUep2TLen4_7R {
        RbUhTokenMaskUep2TLen4_7R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - endpoint 2 transmittal length"]
    #[inline(always)]
    pub fn uep2_t_len_8_15(&self) -> Uep2TLen8_15R {
        Uep2TLen8_15R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The endpoint number of the target of this operation"]
    #[inline(always)]
    pub fn rb_uh_epnum_mask_uep2_t_len_0_3(
        &mut self,
    ) -> RbUhEpnumMaskUep2TLen0_3W<'_, R16Uep2TLenR16UhEpPidSpec> {
        RbUhEpnumMaskUep2TLen0_3W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The token PID packet identification of this USB transfer transaction"]
    #[inline(always)]
    pub fn rb_uh_token_mask_uep2_t_len_4_7(
        &mut self,
    ) -> RbUhTokenMaskUep2TLen4_7W<'_, R16Uep2TLenR16UhEpPidSpec> {
        RbUhTokenMaskUep2TLen4_7W::new(self, 4)
    }
    #[doc = "Bits 8:15 - endpoint 2 transmittal length"]
    #[inline(always)]
    pub fn uep2_t_len_8_15(&mut self) -> Uep2TLen8_15W<'_, R16Uep2TLenR16UhEpPidSpec> {
        Uep2TLen8_15W::new(self, 8)
    }
}
#[doc = "endpoint 2 transmittal length / Set usb host token register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep2_t_len_r16_uh_ep_pid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep2_t_len_r16_uh_ep_pid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep2TLenR16UhEpPidSpec;
impl crate::RegisterSpec for R16Uep2TLenR16UhEpPidSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep2_t_len_r16_uh_ep_pid::R`](R) reader structure"]
impl crate::Readable for R16Uep2TLenR16UhEpPidSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep2_t_len_r16_uh_ep_pid::W`](W) writer structure"]
impl crate::Writable for R16Uep2TLenR16UhEpPidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP2_T_LEN_R16_UH_EP_PID to value 0"]
impl crate::Resettable for R16Uep2TLenR16UhEpPidSpec {}
