#[doc = "Register `R8_TMR0_FIFO_COUNT` reader"]
pub type R = crate::R<R8Tmr0FifoCountSpec>;
#[doc = "Field `R8_TMR0_FIFO_COUNT` reader - TMR0 FIFO count status"]
pub type R8Tmr0FifoCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TMR0 FIFO count status"]
    #[inline(always)]
    pub fn r8_tmr0_fifo_count(&self) -> R8Tmr0FifoCountR {
        R8Tmr0FifoCountR::new(self.bits)
    }
}
#[doc = "TMR0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr0_fifo_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr0FifoCountSpec;
impl crate::RegisterSpec for R8Tmr0FifoCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr0_fifo_count::R`](R) reader structure"]
impl crate::Readable for R8Tmr0FifoCountSpec {}
#[doc = "`reset()` method sets R8_TMR0_FIFO_COUNT to value 0"]
impl crate::Resettable for R8Tmr0FifoCountSpec {}
