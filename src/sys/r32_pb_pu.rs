#[doc = "Register `R32_PB_PU` reader"]
pub type R = crate::R<R32PbPuSpec>;
#[doc = "Register `R32_PB_PU` writer"]
pub type W = crate::W<R32PbPuSpec>;
#[doc = "Field `R32_PB_PU` reader - GPIO PB pullup resistance enable"]
pub type R32PbPuR = crate::FieldReader<u32>;
#[doc = "Field `R32_PB_PU` writer - GPIO PB pullup resistance enable"]
pub type R32PbPuW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn r32_pb_pu(&self) -> R32PbPuR {
        R32PbPuR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn r32_pb_pu(&mut self) -> R32PbPuW<'_, R32PbPuSpec> {
        R32PbPuW::new(self, 0)
    }
}
#[doc = "GPIO PB pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_pu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_pu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbPuSpec;
impl crate::RegisterSpec for R32PbPuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_pu::R`](R) reader structure"]
impl crate::Readable for R32PbPuSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_pu::W`](W) writer structure"]
impl crate::Writable for R32PbPuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_PU to value 0"]
impl crate::Resettable for R32PbPuSpec {}
