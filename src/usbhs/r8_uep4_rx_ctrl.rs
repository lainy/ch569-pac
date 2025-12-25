#[doc = "Register `R8_UEP4_RX_CTRL` reader"]
pub type R = crate::R<R8Uep4RxCtrlSpec>;
#[doc = "Register `R8_UEP4_RX_CTRL` writer"]
pub type W = crate::W<R8Uep4RxCtrlSpec>;
#[doc = "Field `RB_UEP_RRES_MASK` reader - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type RbUepRresMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_RRES_MASK` writer - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
pub type RbUepRresMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_RRES_NO` reader - prepared no response"]
pub type RbUepRresNoR = crate::BitReader;
#[doc = "Field `RB_UEP_RRES_NO` writer - prepared no response"]
pub type RbUepRresNoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_R_TOG_MASK` reader - expected data toggle flag of USB endpoint X receiving"]
pub type RbUepRTogMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_R_TOG_MASK` writer - expected data toggle flag of USB endpoint X receiving"]
pub type RbUepRTogMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_R_AUTOTOG` reader - enable automatic toggle after successful transfer completion on endpoint"]
pub type RbUepRAutotogR = crate::BitReader;
#[doc = "Field `RB_UEP_R_AUTOTOG` writer - enable automatic toggle after successful transfer completion on endpoint"]
pub type RbUepRAutotogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep_rres_mask(&self) -> RbUepRresMaskR {
        RbUepRresMaskR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - prepared no response"]
    #[inline(always)]
    pub fn rb_uep_rres_no(&self) -> RbUepRresNoR {
        RbUepRresNoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask(&self) -> RbUepRTogMaskR {
        RbUepRTogMaskR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint"]
    #[inline(always)]
    pub fn rb_uep_r_autotog(&self) -> RbUepRAutotogR {
        RbUepRAutotogR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep_rres_mask(&mut self) -> RbUepRresMaskW<'_, R8Uep4RxCtrlSpec> {
        RbUepRresMaskW::new(self, 0)
    }
    #[doc = "Bit 2 - prepared no response"]
    #[inline(always)]
    pub fn rb_uep_rres_no(&mut self) -> RbUepRresNoW<'_, R8Uep4RxCtrlSpec> {
        RbUepRresNoW::new(self, 2)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask(&mut self) -> RbUepRTogMaskW<'_, R8Uep4RxCtrlSpec> {
        RbUepRTogMaskW::new(self, 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint"]
    #[inline(always)]
    pub fn rb_uep_r_autotog(&mut self) -> RbUepRAutotogW<'_, R8Uep4RxCtrlSpec> {
        RbUepRAutotogW::new(self, 5)
    }
}
#[doc = "endpoint 4 rx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep4_rx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep4_rx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep4RxCtrlSpec;
impl crate::RegisterSpec for R8Uep4RxCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep4_rx_ctrl::R`](R) reader structure"]
impl crate::Readable for R8Uep4RxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep4_rx_ctrl::W`](W) writer structure"]
impl crate::Writable for R8Uep4RxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP4_RX_CTRL to value 0"]
impl crate::Resettable for R8Uep4RxCtrlSpec {}
