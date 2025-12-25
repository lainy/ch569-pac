#[doc = "Register `R16_DVP_ROW_CNT` reader"]
pub type R = crate::R<R16DvpRowCntSpec>;
#[doc = "Field `RB_DVP_ROW_CNT` reader - DVP receive fifo full"]
pub type RbDvpRowCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DVP receive fifo full"]
    #[inline(always)]
    pub fn rb_dvp_row_cnt(&self) -> RbDvpRowCntR {
        RbDvpRowCntR::new(self.bits)
    }
}
#[doc = "DVP row count value\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_dvp_row_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16DvpRowCntSpec;
impl crate::RegisterSpec for R16DvpRowCntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_dvp_row_cnt::R`](R) reader structure"]
impl crate::Readable for R16DvpRowCntSpec {}
#[doc = "`reset()` method sets R16_DVP_ROW_CNT to value 0"]
impl crate::Resettable for R16DvpRowCntSpec {}
