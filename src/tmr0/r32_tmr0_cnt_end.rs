#[doc = "Register `R32_TMR0_CNT_END` reader"]
pub type R = crate::R<R32Tmr0CntEndSpec>;
#[doc = "Register `R32_TMR0_CNT_END` writer"]
pub type W = crate::W<R32Tmr0CntEndSpec>;
#[doc = "Field `R32_TMR0_COUNT` reader - TMR0 current count"]
pub type R32Tmr0CountR = crate::FieldReader<u32>;
#[doc = "Field `R32_TMR0_COUNT` writer - TMR0 current count"]
pub type R32Tmr0CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TMR0 current count"]
    #[inline(always)]
    pub fn r32_tmr0_count(&self) -> R32Tmr0CountR {
        R32Tmr0CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TMR0 current count"]
    #[inline(always)]
    pub fn r32_tmr0_count(&mut self) -> R32Tmr0CountW<'_, R32Tmr0CntEndSpec> {
        R32Tmr0CountW::new(self, 0)
    }
}
#[doc = "TMR0 end count value, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr0_cnt_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr0_cnt_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr0CntEndSpec;
impl crate::RegisterSpec for R32Tmr0CntEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr0_cnt_end::R`](R) reader structure"]
impl crate::Readable for R32Tmr0CntEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr0_cnt_end::W`](W) writer structure"]
impl crate::Writable for R32Tmr0CntEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR0_CNT_END to value 0"]
impl crate::Resettable for R32Tmr0CntEndSpec {}
