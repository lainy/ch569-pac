#[doc = "Register `R8_HSPI_BURST_CNT` reader"]
pub type R = crate::R<R8HspiBurstCntSpec>;
#[doc = "Register `R8_HSPI_BURST_CNT` writer"]
pub type W = crate::W<R8HspiBurstCntSpec>;
#[doc = "Field `RB_HSPI_BURST_CNT` reader - parallel if tx burst count"]
pub type RbHspiBurstCntR = crate::FieldReader;
#[doc = "Field `RB_HSPI_BURST_CNT` writer - parallel if tx burst count"]
pub type RbHspiBurstCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - parallel if tx burst count"]
    #[inline(always)]
    pub fn rb_hspi_burst_cnt(&self) -> RbHspiBurstCntR {
        RbHspiBurstCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - parallel if tx burst count"]
    #[inline(always)]
    pub fn rb_hspi_burst_cnt(&mut self) -> RbHspiBurstCntW<'_, R8HspiBurstCntSpec> {
        RbHspiBurstCntW::new(self, 0)
    }
}
#[doc = "parallel if tx burst count\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_burst_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_burst_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiBurstCntSpec;
impl crate::RegisterSpec for R8HspiBurstCntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_burst_cnt::R`](R) reader structure"]
impl crate::Readable for R8HspiBurstCntSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_burst_cnt::W`](W) writer structure"]
impl crate::Writable for R8HspiBurstCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_BURST_CNT to value 0"]
impl crate::Resettable for R8HspiBurstCntSpec {}
