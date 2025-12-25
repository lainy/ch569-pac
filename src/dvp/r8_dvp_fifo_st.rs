#[doc = "Register `R8_DVP_FIFO_ST` reader"]
pub type R = crate::R<R8DvpFifoStSpec>;
#[doc = "Field `RB_DVP_FIFO_RDY` reader - DVP receive fifo ready"]
pub type RbDvpFifoRdyR = crate::BitReader;
#[doc = "Field `RB_DVP_FIFO_FULL` reader - DVP receive fifo full"]
pub type RbDvpFifoFullR = crate::BitReader;
#[doc = "Field `RB_DVP_FIFO_OV` reader - DVP receive fifo overflow"]
pub type RbDvpFifoOvR = crate::BitReader;
#[doc = "Field `RB_DVP_MSK_FIFO_CNT` reader - DVP receive fifo count"]
pub type RbDvpMskFifoCntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DVP receive fifo ready"]
    #[inline(always)]
    pub fn rb_dvp_fifo_rdy(&self) -> RbDvpFifoRdyR {
        RbDvpFifoRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP receive fifo full"]
    #[inline(always)]
    pub fn rb_dvp_fifo_full(&self) -> RbDvpFifoFullR {
        RbDvpFifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive fifo overflow"]
    #[inline(always)]
    pub fn rb_dvp_fifo_ov(&self) -> RbDvpFifoOvR {
        RbDvpFifoOvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - DVP receive fifo count"]
    #[inline(always)]
    pub fn rb_dvp_msk_fifo_cnt(&self) -> RbDvpMskFifoCntR {
        RbDvpMskFifoCntR::new((self.bits >> 4) & 7)
    }
}
#[doc = "DVP receive fifo status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_fifo_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8DvpFifoStSpec;
impl crate::RegisterSpec for R8DvpFifoStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_dvp_fifo_st::R`](R) reader structure"]
impl crate::Readable for R8DvpFifoStSpec {}
#[doc = "`reset()` method sets R8_DVP_FIFO_ST to value 0"]
impl crate::Resettable for R8DvpFifoStSpec {}
