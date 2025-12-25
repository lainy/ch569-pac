#[doc = "Register `R32_PB_PD` reader"]
pub type R = crate::R<R32PbPdSpec>;
#[doc = "Register `R32_PB_PD` writer"]
pub type W = crate::W<R32PbPdSpec>;
#[doc = "Field `R32_PB_PD` reader - GPIO PB output open-drain_input pulldown resistance enable"]
pub type R32PbPdR = crate::FieldReader<u32>;
#[doc = "Field `R32_PB_PD` writer - GPIO PB output open-drain_input pulldown resistance enable"]
pub type R32PbPdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB output open-drain_input pulldown resistance enable"]
    #[inline(always)]
    pub fn r32_pb_pd(&self) -> R32PbPdR {
        R32PbPdR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - GPIO PB output open-drain_input pulldown resistance enable"]
    #[inline(always)]
    pub fn r32_pb_pd(&mut self) -> R32PbPdW<'_, R32PbPdSpec> {
        R32PbPdW::new(self, 0)
    }
}
#[doc = "GPIO PB output open-drain_input pulldown resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbPdSpec;
impl crate::RegisterSpec for R32PbPdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_pd::R`](R) reader structure"]
impl crate::Readable for R32PbPdSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_pd::W`](W) writer structure"]
impl crate::Writable for R32PbPdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_PD to value 0"]
impl crate::Resettable for R32PbPdSpec {}
