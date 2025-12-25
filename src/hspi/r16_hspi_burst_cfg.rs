#[doc = "Register `R16_HSPI_BURST_CFG` reader"]
pub type R = crate::R<R16HspiBurstCfgSpec>;
#[doc = "Register `R16_HSPI_BURST_CFG` writer"]
pub type W = crate::W<R16HspiBurstCfgSpec>;
#[doc = "Field `RB_HSPI_BURST_EN` reader - burst transmit enable"]
pub type RbHspiBurstEnR = crate::BitReader;
#[doc = "Field `RB_HSPI_BURST_EN` writer - burst transmit enable"]
pub type RbHspiBurstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_BURST_LEN` reader - burst transmit length"]
pub type RbHspiBurstLenR = crate::FieldReader;
#[doc = "Field `RB_HSPI_BURST_LEN` writer - burst transmit length"]
pub type RbHspiBurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - burst transmit enable"]
    #[inline(always)]
    pub fn rb_hspi_burst_en(&self) -> RbHspiBurstEnR {
        RbHspiBurstEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - burst transmit length"]
    #[inline(always)]
    pub fn rb_hspi_burst_len(&self) -> RbHspiBurstLenR {
        RbHspiBurstLenR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - burst transmit enable"]
    #[inline(always)]
    pub fn rb_hspi_burst_en(&mut self) -> RbHspiBurstEnW<'_, R16HspiBurstCfgSpec> {
        RbHspiBurstEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - burst transmit length"]
    #[inline(always)]
    pub fn rb_hspi_burst_len(&mut self) -> RbHspiBurstLenW<'_, R16HspiBurstCfgSpec> {
        RbHspiBurstLenW::new(self, 8)
    }
}
#[doc = "parallel if tx burst config register\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_hspi_burst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_hspi_burst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16HspiBurstCfgSpec;
impl crate::RegisterSpec for R16HspiBurstCfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_hspi_burst_cfg::R`](R) reader structure"]
impl crate::Readable for R16HspiBurstCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_hspi_burst_cfg::W`](W) writer structure"]
impl crate::Writable for R16HspiBurstCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_HSPI_BURST_CFG to value 0"]
impl crate::Resettable for R16HspiBurstCfgSpec {}
