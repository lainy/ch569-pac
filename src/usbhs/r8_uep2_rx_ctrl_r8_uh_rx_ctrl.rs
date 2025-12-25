#[doc = "Register `R8_UEP2_RX_CTRL_R8_UH_RX_CTRL` reader"]
pub type R = crate::R<R8Uep2RxCtrlR8UhRxCtrlSpec>;
#[doc = "Register `R8_UEP2_RX_CTRL_R8_UH_RX_CTRL` writer"]
pub type W = crate::W<R8Uep2RxCtrlR8UhRxCtrlSpec>;
#[doc = "Field `RB_UEP_RRES_MASK_RB_UH_RRES_MASK` reader - bit mask of handshake response type for USB endpoint X receiving (OUT) / Host reeiver response control bit"]
pub type RbUepRresMaskRbUhRresMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_RRES_MASK_RB_UH_RRES_MASK` writer - bit mask of handshake response type for USB endpoint X receiving (OUT) / Host reeiver response control bit"]
pub type RbUepRresMaskRbUhRresMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_RRES_NO_RB_UH_RRES_NO` reader - prepared no response / Response control bit of host receiver"]
pub type RbUepRresNoRbUhRresNoR = crate::BitReader;
#[doc = "Field `RB_UEP_RRES_NO_RB_UH_RRES_NO` writer - prepared no response / Response control bit of host receiver"]
pub type RbUepRresNoRbUhRresNoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK` reader - expected data toggle flag of USB endpoint X receiving / expected data toggle flag of host receiving (IN)"]
pub type RbUepRTogMaskRbUhRTogMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK` writer - expected data toggle flag of USB endpoint X receiving / expected data toggle flag of host receiving (IN)"]
pub type RbUepRTogMaskRbUhRTogMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG` reader - enable automatic toggle after successful transfer completion on endpoint / enable automatic toggle after successful receiver completion"]
pub type RbUepRAutotogRbUhRAutotogR = crate::BitReader;
#[doc = "Field `RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG` writer - enable automatic toggle after successful transfer completion on endpoint / enable automatic toggle after successful receiver completion"]
pub type RbUepRAutotogRbUhRAutotogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UH_RDATA_NO` reader - expect no data packet, for high speed hub in host mode"]
pub type RbUhRdataNoR = crate::BitReader;
#[doc = "Field `RB_UH_RDATA_NO` writer - expect no data packet, for high speed hub in host mode"]
pub type RbUhRdataNoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT) / Host reeiver response control bit"]
    #[inline(always)]
    pub fn rb_uep_rres_mask_rb_uh_rres_mask(&self) -> RbUepRresMaskRbUhRresMaskR {
        RbUepRresMaskRbUhRresMaskR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - prepared no response / Response control bit of host receiver"]
    #[inline(always)]
    pub fn rb_uep_rres_no_rb_uh_rres_no(&self) -> RbUepRresNoRbUhRresNoR {
        RbUepRresNoRbUhRresNoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving / expected data toggle flag of host receiving (IN)"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask_rb_uh_r_tog_mask(&self) -> RbUepRTogMaskRbUhRTogMaskR {
        RbUepRTogMaskRbUhRTogMaskR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint / enable automatic toggle after successful receiver completion"]
    #[inline(always)]
    pub fn rb_uep_r_autotog_rb_uh_r_autotog(&self) -> RbUepRAutotogRbUhRAutotogR {
        RbUepRAutotogRbUhRAutotogR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - expect no data packet, for high speed hub in host mode"]
    #[inline(always)]
    pub fn rb_uh_rdata_no(&self) -> RbUhRdataNoR {
        RbUhRdataNoR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT) / Host reeiver response control bit"]
    #[inline(always)]
    pub fn rb_uep_rres_mask_rb_uh_rres_mask(
        &mut self,
    ) -> RbUepRresMaskRbUhRresMaskW<'_, R8Uep2RxCtrlR8UhRxCtrlSpec> {
        RbUepRresMaskRbUhRresMaskW::new(self, 0)
    }
    #[doc = "Bit 2 - prepared no response / Response control bit of host receiver"]
    #[inline(always)]
    pub fn rb_uep_rres_no_rb_uh_rres_no(
        &mut self,
    ) -> RbUepRresNoRbUhRresNoW<'_, R8Uep2RxCtrlR8UhRxCtrlSpec> {
        RbUepRresNoRbUhRresNoW::new(self, 2)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving / expected data toggle flag of host receiving (IN)"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask_rb_uh_r_tog_mask(
        &mut self,
    ) -> RbUepRTogMaskRbUhRTogMaskW<'_, R8Uep2RxCtrlR8UhRxCtrlSpec> {
        RbUepRTogMaskRbUhRTogMaskW::new(self, 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint / enable automatic toggle after successful receiver completion"]
    #[inline(always)]
    pub fn rb_uep_r_autotog_rb_uh_r_autotog(
        &mut self,
    ) -> RbUepRAutotogRbUhRAutotogW<'_, R8Uep2RxCtrlR8UhRxCtrlSpec> {
        RbUepRAutotogRbUhRAutotogW::new(self, 5)
    }
    #[doc = "Bit 6 - expect no data packet, for high speed hub in host mode"]
    #[inline(always)]
    pub fn rb_uh_rdata_no(&mut self) -> RbUhRdataNoW<'_, R8Uep2RxCtrlR8UhRxCtrlSpec> {
        RbUhRdataNoW::new(self, 6)
    }
}
#[doc = "endpoint 2 rx control / USb host receive endpoint control register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep2_rx_ctrl_r8_uh_rx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep2_rx_ctrl_r8_uh_rx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep2RxCtrlR8UhRxCtrlSpec;
impl crate::RegisterSpec for R8Uep2RxCtrlR8UhRxCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep2_rx_ctrl_r8_uh_rx_ctrl::R`](R) reader structure"]
impl crate::Readable for R8Uep2RxCtrlR8UhRxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep2_rx_ctrl_r8_uh_rx_ctrl::W`](W) writer structure"]
impl crate::Writable for R8Uep2RxCtrlR8UhRxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP2_RX_CTRL_R8_UH_RX_CTRL to value 0"]
impl crate::Resettable for R8Uep2RxCtrlR8UhRxCtrlSpec {}
