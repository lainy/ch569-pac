#[doc = "Register `R16_DVP_COL_CNT` reader"]
pub type R = crate::R<R16DvpColCntSpec>;
#[doc = "Field `RB_DVP_COL_CNT` reader - DVP receive fifo ready"]
pub type RbDvpColCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DVP receive fifo ready"]
    #[inline(always)]
    pub fn rb_dvp_col_cnt(&self) -> RbDvpColCntR {
        RbDvpColCntR::new(self.bits)
    }
}
#[doc = "DVP col count value\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_col_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16DvpColCntSpec;
impl crate::RegisterSpec for R16DvpColCntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_dvp_col_cnt::R`](R) reader structure"]
impl crate::Readable for R16DvpColCntSpec {}
#[doc = "`reset()` method sets R16_DVP_COL_CNT to value 0"]
impl crate::Resettable for R16DvpColCntSpec {}
