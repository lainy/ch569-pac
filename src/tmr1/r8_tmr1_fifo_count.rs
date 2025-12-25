#[doc = "Register `R8_TMR1_FIFO_COUNT` reader"]
pub type R = crate::R<R8Tmr1FifoCountSpec>;
#[doc = "Field `R8_TMR1_FIFO_COUNT` reader - TMR FIFO count status"]
pub type R8Tmr1FifoCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TMR FIFO count status"]
    #[inline(always)]
    pub fn r8_tmr1_fifo_count(&self) -> R8Tmr1FifoCountR {
        R8Tmr1FifoCountR::new(self.bits)
    }
}
#[doc = "TMR1 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr1_fifo_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr1FifoCountSpec;
impl crate::RegisterSpec for R8Tmr1FifoCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr1_fifo_count::R`](R) reader structure"]
impl crate::Readable for R8Tmr1FifoCountSpec {}
#[doc = "`reset()` method sets R8_TMR1_FIFO_COUNT to value 0"]
impl crate::Resettable for R8Tmr1FifoCountSpec {}
