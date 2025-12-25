#[doc = "Register `R32_PB_SMT` reader"]
pub type R = crate::R<R32PbSmtSpec>;
#[doc = "Register `R32_PB_SMT` writer"]
pub type W = crate::W<R32PbSmtSpec>;
#[doc = "Field `R32_PB_SMT` reader - GPIO PB output slew rate_input schmitt trigger"]
pub type R32PbSmtR = crate::FieldReader<u32>;
#[doc = "Field `R32_PB_SMT` writer - GPIO PB output slew rate_input schmitt trigger"]
pub type R32PbSmtW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB output slew rate_input schmitt trigger"]
    #[inline(always)]
    pub fn r32_pb_smt(&self) -> R32PbSmtR {
        R32PbSmtR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - GPIO PB output slew rate_input schmitt trigger"]
    #[inline(always)]
    pub fn r32_pb_smt(&mut self) -> R32PbSmtW<'_, R32PbSmtSpec> {
        R32PbSmtW::new(self, 0)
    }
}
#[doc = "GPIO PB output slew rate_input schmitt trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_smt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_smt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbSmtSpec;
impl crate::RegisterSpec for R32PbSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_smt::R`](R) reader structure"]
impl crate::Readable for R32PbSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_smt::W`](W) writer structure"]
impl crate::Writable for R32PbSmtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_SMT to value 0"]
impl crate::Resettable for R32PbSmtSpec {}
