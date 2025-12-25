#[doc = "Register `R8_DVP_INT_EN` reader"]
pub type R = crate::R<R8DvpIntEnSpec>;
#[doc = "Register `R8_DVP_INT_EN` writer"]
pub type W = crate::W<R8DvpIntEnSpec>;
#[doc = "Field `RB_DVP_IE_STR_FRM` reader - DVP frame start interrupt enable"]
pub type RbDvpIeStrFrmR = crate::BitReader;
#[doc = "Field `RB_DVP_IE_STR_FRM` writer - DVP frame start interrupt enable"]
pub type RbDvpIeStrFrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IE_ROW_DONE` reader - DVP row received done interrupt enable"]
pub type RbDvpIeRowDoneR = crate::BitReader;
#[doc = "Field `RB_DVP_IE_ROW_DONE` writer - DVP row received done interrupt enable"]
pub type RbDvpIeRowDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IE_FRM_DONE` reader - DVP frame received done interrupt enable"]
pub type RbDvpIeFrmDoneR = crate::BitReader;
#[doc = "Field `RB_DVP_IE_FRM_DONE` writer - DVP frame received done interrupt enable"]
pub type RbDvpIeFrmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IE_FIFO_OV` reader - DVP receive fifo overflow interrupt enable"]
pub type RbDvpIeFifoOvR = crate::BitReader;
#[doc = "Field `RB_DVP_IE_FIFO_OV` writer - DVP receive fifo overflow interrupt enable"]
pub type RbDvpIeFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IE_STP_FRM` reader - DVP frame stop interrupt enable"]
pub type RbDvpIeStpFrmR = crate::BitReader;
#[doc = "Field `RB_DVP_IE_STP_FRM` writer - DVP frame stop interrupt enable"]
pub type RbDvpIeStpFrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&self) -> RbDvpIeStrFrmR {
        RbDvpIeStrFrmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&self) -> RbDvpIeRowDoneR {
        RbDvpIeRowDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&self) -> RbDvpIeFrmDoneR {
        RbDvpIeFrmDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&self) -> RbDvpIeFifoOvR {
        RbDvpIeFifoOvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&self) -> RbDvpIeStpFrmR {
        RbDvpIeStpFrmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&mut self) -> RbDvpIeStrFrmW<'_, R8DvpIntEnSpec> {
        RbDvpIeStrFrmW::new(self, 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&mut self) -> RbDvpIeRowDoneW<'_, R8DvpIntEnSpec> {
        RbDvpIeRowDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&mut self) -> RbDvpIeFrmDoneW<'_, R8DvpIntEnSpec> {
        RbDvpIeFrmDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&mut self) -> RbDvpIeFifoOvW<'_, R8DvpIntEnSpec> {
        RbDvpIeFifoOvW::new(self, 3)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&mut self) -> RbDvpIeStpFrmW<'_, R8DvpIntEnSpec> {
        RbDvpIeStpFrmW::new(self, 4)
    }
}
#[doc = "DVP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8DvpIntEnSpec;
impl crate::RegisterSpec for R8DvpIntEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_dvp_int_en::R`](R) reader structure"]
impl crate::Readable for R8DvpIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_dvp_int_en::W`](W) writer structure"]
impl crate::Writable for R8DvpIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_DVP_INT_EN to value 0"]
impl crate::Resettable for R8DvpIntEnSpec {}
