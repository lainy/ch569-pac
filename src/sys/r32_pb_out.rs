#[doc = "Register `R32_PB_OUT` reader"]
pub type R = crate::R<R32PbOutSpec>;
#[doc = "Register `R32_PB_OUT` writer"]
pub type W = crate::W<R32PbOutSpec>;
#[doc = "Field `R32_PB_OUT` reader - GPIO PB output"]
pub type R32PbOutR = crate::FieldReader<u32>;
#[doc = "Field `R32_PB_OUT` writer - GPIO PB output"]
pub type R32PbOutW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB output"]
    #[inline(always)]
    pub fn r32_pb_out(&self) -> R32PbOutR {
        R32PbOutR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - GPIO PB output"]
    #[inline(always)]
    pub fn r32_pb_out(&mut self) -> R32PbOutW<'_, R32PbOutSpec> {
        R32PbOutW::new(self, 0)
    }
}
#[doc = "GPIO PB output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbOutSpec;
impl crate::RegisterSpec for R32PbOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_out::R`](R) reader structure"]
impl crate::Readable for R32PbOutSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_out::W`](W) writer structure"]
impl crate::Writable for R32PbOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_OUT to value 0"]
impl crate::Resettable for R32PbOutSpec {}
