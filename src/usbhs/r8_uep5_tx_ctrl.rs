#[doc = "Register `R8_UEP5_TX_CTRL` reader"]
pub type R = crate::R<R8Uep5TxCtrlSpec>;
#[doc = "Register `R8_UEP5_TX_CTRL` writer"]
pub type W = crate::W<R8Uep5TxCtrlSpec>;
#[doc = "Field `RB_UEP_TRES_MASK` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type RbUepTresMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_TRES_MASK` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type RbUepTresMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_TRES_NO` reader - expected no response"]
pub type RbUepTresNoR = crate::BitReader;
#[doc = "Field `RB_UEP_TRES_NO` writer - expected no response"]
pub type RbUepTresNoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_UEP_T_TOG_MASK` reader - prepared data toggle flag of USB endpoint X transmittal"]
pub type RbUepTTogMaskR = crate::FieldReader;
#[doc = "Field `RB_UEP_T_TOG_MASK` writer - prepared data toggle flag of USB endpoint X transmittal"]
pub type RbUepTTogMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_UEP_T_AUTOTOG` reader - enable automatic toggle after successful transfer completion on endpoint 0"]
pub type RbUepTAutotogR = crate::BitReader;
#[doc = "Field `RB_UEP_T_AUTOTOG` writer - enable automatic toggle after successful transfer completion on endpoint 0"]
pub type RbUepTAutotogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep_tres_mask(&self) -> RbUepTresMaskR {
        RbUepTresMaskR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - expected no response"]
    #[inline(always)]
    pub fn rb_uep_tres_no(&self) -> RbUepTresNoR {
        RbUepTresNoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - prepared data toggle flag of USB endpoint X transmittal"]
    #[inline(always)]
    pub fn rb_uep_t_tog_mask(&self) -> RbUepTTogMaskR {
        RbUepTTogMaskR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint 0"]
    #[inline(always)]
    pub fn rb_uep_t_autotog(&self) -> RbUepTAutotogR {
        RbUepTAutotogR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep_tres_mask(&mut self) -> RbUepTresMaskW<'_, R8Uep5TxCtrlSpec> {
        RbUepTresMaskW::new(self, 0)
    }
    #[doc = "Bit 2 - expected no response"]
    #[inline(always)]
    pub fn rb_uep_tres_no(&mut self) -> RbUepTresNoW<'_, R8Uep5TxCtrlSpec> {
        RbUepTresNoW::new(self, 2)
    }
    #[doc = "Bits 3:4 - prepared data toggle flag of USB endpoint X transmittal"]
    #[inline(always)]
    pub fn rb_uep_t_tog_mask(&mut self) -> RbUepTTogMaskW<'_, R8Uep5TxCtrlSpec> {
        RbUepTTogMaskW::new(self, 3)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint 0"]
    #[inline(always)]
    pub fn rb_uep_t_autotog(&mut self) -> RbUepTAutotogW<'_, R8Uep5TxCtrlSpec> {
        RbUepTAutotogW::new(self, 5)
    }
}
#[doc = "endpoint 5 tx control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uep5_tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uep5_tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uep5TxCtrlSpec;
impl crate::RegisterSpec for R8Uep5TxCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep5_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for R8Uep5TxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uep5_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for R8Uep5TxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UEP5_TX_CTRL to value 0"]
impl crate::Resettable for R8Uep5TxCtrlSpec {}
