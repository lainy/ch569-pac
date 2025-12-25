#[doc = "Register `R32_TMR1_FIFO` reader"]
pub type R = crate::R<R32Tmr1FifoSpec>;
#[doc = "Register `R32_TMR1_FIFO` writer"]
pub type W = crate::W<R32Tmr1FifoSpec>;
#[doc = "Field `R32_TMR1_FIFO` reader - TMR current count"]
pub type R32Tmr1FifoR = crate::FieldReader<u32>;
#[doc = "Field `R32_TMR1_FIFO` writer - TMR current count"]
pub type R32Tmr1FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TMR current count"]
    #[inline(always)]
    pub fn r32_tmr1_fifo(&self) -> R32Tmr1FifoR {
        R32Tmr1FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TMR current count"]
    #[inline(always)]
    pub fn r32_tmr1_fifo(&mut self) -> R32Tmr1FifoW<'_, R32Tmr1FifoSpec> {
        R32Tmr1FifoW::new(self, 0)
    }
}
#[doc = "TMR1 FIFO only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr1_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr1FifoSpec;
impl crate::RegisterSpec for R32Tmr1FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr1_fifo::R`](R) reader structure"]
impl crate::Readable for R32Tmr1FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr1_fifo::W`](W) writer structure"]
impl crate::Writable for R32Tmr1FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR1_FIFO to value 0"]
impl crate::Resettable for R32Tmr1FifoSpec {}
