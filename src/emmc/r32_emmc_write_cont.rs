#[doc = "Register `R32_EMMC_WRITE_CONT` writer"]
pub type W = crate::W<R32EmmcWriteContSpec>;
#[doc = "Field `R32_EMMC_WRITE_CONT` writer - response parameter register"]
pub type R32EmmcWriteContW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn r32_emmc_write_cont(&mut self) -> R32EmmcWriteContW<'_, R32EmmcWriteContSpec> {
        R32EmmcWriteContW::new(self, 0)
    }
}
#[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\] 32bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_write_cont::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcWriteContSpec;
impl crate::RegisterSpec for R32EmmcWriteContSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r32_emmc_write_cont::W`](W) writer structure"]
impl crate::Writable for R32EmmcWriteContSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_EMMC_WRITE_CONT to value 0"]
impl crate::Resettable for R32EmmcWriteContSpec {}
