#[doc = "Register `R8_DVP_INT_FLAG` reader"]
pub type R = crate::R<R8DvpIntFlagSpec>;
#[doc = "Register `R8_DVP_INT_FLAG` writer"]
pub type W = crate::W<R8DvpIntFlagSpec>;
#[doc = "Field `RB_DVP_IF_STR_FRM` reader - interrupt flag for DVP frame start"]
pub type RbDvpIfStrFrmR = crate::BitReader;
#[doc = "Field `RB_DVP_IF_STR_FRM` writer - interrupt flag for DVP frame start"]
pub type RbDvpIfStrFrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IF_ROW_DONE` reader - interrupt flag for DVP row receive done"]
pub type RbDvpIfRowDoneR = crate::BitReader;
#[doc = "Field `RB_DVP_IF_ROW_DONE` writer - interrupt flag for DVP row receive done"]
pub type RbDvpIfRowDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IF_FRM_DONE` reader - interrupt flag for DVP frame receive done"]
pub type RbDvpIfFrmDoneR = crate::BitReader;
#[doc = "Field `RB_DVP_IF_FRM_DONE` writer - interrupt flag for DVP frame receive done"]
pub type RbDvpIfFrmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IF_FIFO_OV` reader - interrupt flag for DVP receive fifo overflow"]
pub type RbDvpIfFifoOvR = crate::BitReader;
#[doc = "Field `RB_DVP_IF_FIFO_OV` writer - interrupt flag for DVP receive fifo overflow"]
pub type RbDvpIfFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_IF_STP_FRM` reader - interrupt flag for DVP frame stop"]
pub type RbDvpIfStpFrmR = crate::BitReader;
#[doc = "Field `RB_DVP_IF_STP_FRM` writer - interrupt flag for DVP frame stop"]
pub type RbDvpIfStpFrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt flag for DVP frame start"]
    #[inline(always)]
    pub fn rb_dvp_if_str_frm(&self) -> RbDvpIfStrFrmR {
        RbDvpIfStrFrmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for DVP row receive done"]
    #[inline(always)]
    pub fn rb_dvp_if_row_done(&self) -> RbDvpIfRowDoneR {
        RbDvpIfRowDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for DVP frame receive done"]
    #[inline(always)]
    pub fn rb_dvp_if_frm_done(&self) -> RbDvpIfFrmDoneR {
        RbDvpIfFrmDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for DVP receive fifo overflow"]
    #[inline(always)]
    pub fn rb_dvp_if_fifo_ov(&self) -> RbDvpIfFifoOvR {
        RbDvpIfFifoOvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt flag for DVP frame stop"]
    #[inline(always)]
    pub fn rb_dvp_if_stp_frm(&self) -> RbDvpIfStpFrmR {
        RbDvpIfStpFrmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for DVP frame start"]
    #[inline(always)]
    pub fn rb_dvp_if_str_frm(&mut self) -> RbDvpIfStrFrmW<'_, R8DvpIntFlagSpec> {
        RbDvpIfStrFrmW::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt flag for DVP row receive done"]
    #[inline(always)]
    pub fn rb_dvp_if_row_done(&mut self) -> RbDvpIfRowDoneW<'_, R8DvpIntFlagSpec> {
        RbDvpIfRowDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt flag for DVP frame receive done"]
    #[inline(always)]
    pub fn rb_dvp_if_frm_done(&mut self) -> RbDvpIfFrmDoneW<'_, R8DvpIntFlagSpec> {
        RbDvpIfFrmDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt flag for DVP receive fifo overflow"]
    #[inline(always)]
    pub fn rb_dvp_if_fifo_ov(&mut self) -> RbDvpIfFifoOvW<'_, R8DvpIntFlagSpec> {
        RbDvpIfFifoOvW::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt flag for DVP frame stop"]
    #[inline(always)]
    pub fn rb_dvp_if_stp_frm(&mut self) -> RbDvpIfStpFrmW<'_, R8DvpIntFlagSpec> {
        RbDvpIfStpFrmW::new(self, 4)
    }
}
#[doc = "DVP interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_int_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_int_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8DvpIntFlagSpec;
impl crate::RegisterSpec for R8DvpIntFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r8_dvp_int_flag::R`](R) reader structure"]
impl crate::Readable for R8DvpIntFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_dvp_int_flag::W`](W) writer structure"]
impl crate::Writable for R8DvpIntFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_DVP_INT_FLAG to value 0"]
impl crate::Resettable for R8DvpIntFlagSpec {}
