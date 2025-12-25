#[doc = "Register `R8_UEP3_TX_CTRL_R8_UH_TX_CTRL` reader"]
pub type R = crate::R<R8Uep3TxCtrlR8UhTxCtrlSpec>;
#[doc = "Register `R8_UEP3_TX_CTRL_R8_UH_TX_CTRL` writer"]
pub type W = crate::W<R8Uep3TxCtrlR8UhTxCtrlSpec>;
#[doc = "Field `RB_UEP_TRES_MASK_RB_UH_TRES_MASK` reader - bit mask of handshake response type for USB endpoint X transmittal (IN) / expected handshake response type for host transmittal (SETUP/OUT)"]
pub type RbUepTresMaskRbUhTresMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_TRES_MASK_RB_UH_TRES_MASK` writer - bit mask of handshake response type for USB endpoint X transmittal (IN) / expected handshake response type for host transmittal (SETUP/OUT)"]
pub type RbUepTresMaskRbUhTresMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_TRES_NO_RB_UH_TRES_NO` reader - expected no response / expected no response, 1=enable, 0=disable, for non-zero endpoint isochronous transactions"]
pub type RbUepTresNoRbUhTresNoR = crate::BitReader;
#[doc = "Field `RB_UEP_TRES_NO_RB_UH_TRES_NO` writer - expected no response / expected no response, 1=enable, 0=disable, for non-zero endpoint isochronous transactions"]
pub type RbUepTresNoRbUhTresNoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_T_TOG_MASK_RB_UH_T_TOG_MASK` reader - prepared data toggle flag of USB endpoint X transmittal / prepared data toggle flag of host transmittal (SETUP/OUT)"]
pub type RbUepTTogMaskRbUhTTogMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_T_TOG_MASK_RB_UH_T_TOG_MASK` writer - prepared data toggle flag of USB endpoint X transmittal / prepared data toggle flag of host transmittal (SETUP/OUT)"]
pub type RbUepTTogMaskRbUhTTogMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_T_AUTOTOG_RB_UH_T_AUTOTOG` reader - enable automatic toggle after successful transfer completion on endpoint 0 / enable automatic toggle after successful transfer completion"]
pub type RbUepTAutotogRbUhTAutotogR = crate::BitReader;
#[doc = "Field `RB_UEP_T_AUTOTOG_RB_UH_T_AUTOTOG` writer - enable automatic toggle after successful transfer completion on endpoint 0 / enable automatic toggle after successful transfer completion"]
pub type RbUepTAutotogRbUhTAutotogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UH_TDATA_NO` reader - prepared no data packet, for high speed hub in host mode"]
pub type RbUhTdataNoR = crate::BitReader;
#[doc = "Field `RB_UH_TDATA_NO` writer - prepared no data packet, for high speed hub in host mode"]
pub type RbUhTdataNoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN) / expected handshake response type for host transmittal (SETUP/OUT)"]
    #[inline(always)]
    pub fn rb_uep_tres_mask_rb_uh_tres_mask(&self) -> RbUepTresMaskRbUhTresMaskR {
        RbUepTresMaskRbUhTresMaskR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - expected no response / expected no response, 1=enable, 0=disable, for non-zero endpoint isochronous transactions"]
    #[inline(always)]
    pub fn rb_uep_tres_no_rb_uh_tres_no(&self) -> RbUepTresNoRbUhTresNoR {
        RbUepTresNoRbUhTresNoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - prepared data toggle flag of USB endpoint X transmittal / prepared data toggle flag of host transmittal (SETUP/OUT)"]
    #[inline(always)]
    pub fn rb_uep_t_tog_mask_rb_uh_t_tog_mask(&self) -> RbUepTTogMaskRbUhTTogMaskR {
        RbUepTTogMaskRbUhTTogMaskR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint 0 / enable automatic toggle after successful transfer completion"]
    #[inline(always)]
    pub fn rb_uep_t_autotog_rb_uh_t_autotog(&self) -> RbUepTAutotogRbUhTAutotogR {
        RbUepTAutotogRbUhTAutotogR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - prepared no data packet, for high speed hub in host mode"]
    #[inline(always)]
    pub fn rb_uh_tdata_no(&self) -> RbUhTdataNoR {
        RbUhTdataNoR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN) / expected handshake response type for host transmittal (SETUP/OUT)"]
    #[inline(always)]
    pub fn rb_uep_tres_mask_rb_uh_tres_mask(
        &mut self,
    ) -> RbUepTresMaskRbUhTresMaskW<'_, R8Uep3TxCtrlR8UhTxCtrlSpec> {
        RbUepTresMaskRbUhTresMaskW::new(self, 0)
    }
    #[doc = "Bit 2 - expected no response / expected no response, 1=enable, 0=disable, for non-zero endpoint isochronous transactions"]
    #[inline(always)]
    pub fn rb_uep_tres_no_rb_uh_tres_no(
        &mut self,
    ) -> RbUepTresNoRbUhTresNoW<'_, R8Uep3TxCtrlR8UhTxCtrlSpec> {
        RbUepTresNoRbUhTresNoW::new(self, 2)
    }
    #[doc = "Bits 3:4 - prepared data toggle flag of USB endpoint X transmittal / prepared data toggle flag of host transmittal (SETUP/OUT)"]
    #[inline(always)]
    pub fn rb_uep_t_tog_mask_rb_uh_t_tog_mask(
        &mut self,
    ) -> RbUepTTogMaskRbUhTTogMaskW<'_, R8Uep3TxCtrlR8UhTxCtrlSpec> {
        RbUepTTogMaskRbUhTTogMaskW::new(self, 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint 0 / enable automatic toggle after successful transfer completion"]
    #[inline(always)]
    pub fn rb_uep_t_autotog_rb_uh_t_autotog(
        &mut self,
    ) -> RbUepTAutotogRbUhTAutotogW<'_, R8Uep3TxCtrlR8UhTxCtrlSpec> {
        RbUepTAutotogRbUhTAutotogW::new(self, 5)
    }
    #[doc = "Bit 6 - prepared no data packet, for high speed hub in host mode"]
    #[inline(always)]
    pub fn rb_uh_tdata_no(&mut self) -> RbUhTdataNoW<'_, R8Uep3TxCtrlR8UhTxCtrlSpec> {
        RbUhTdataNoW::new(self, 6)
    }
}
#[doc = "endpoint 3 tx control / host transmittal endpoint control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep3_tx_ctrl_r8_uh_tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep3_tx_ctrl_r8_uh_tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep3TxCtrlR8UhTxCtrlSpec;
impl crate::RegisterSpec for R8Uep3TxCtrlR8UhTxCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep3_tx_ctrl_r8_uh_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for R8Uep3TxCtrlR8UhTxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep3_tx_ctrl_r8_uh_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for R8Uep3TxCtrlR8UhTxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP3_TX_CTRL_R8_UH_TX_CTRL to value 0"]
impl crate::Resettable for R8Uep3TxCtrlR8UhTxCtrlSpec {}
