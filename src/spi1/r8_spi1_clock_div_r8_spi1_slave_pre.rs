#[doc = "Register `R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE` reader"]
pub type R = crate::R<R8Spi1ClockDivR8Spi1SlavePreSpec>;
#[doc = "Register `R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE` writer"]
pub type W = crate::W<R8Spi1ClockDivR8Spi1SlavePreSpec>;
#[doc = "Field `R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRESET` reader - master clock divisor / SPI0 slave preset value"]
pub type R8Spi1ClockDivR8Spi1SlavePresetR = crate::FieldReader;
#[doc = "Field `R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRESET` writer - master clock divisor / SPI0 slave preset value"]
pub type R8Spi1ClockDivR8Spi1SlavePresetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - master clock divisor / SPI0 slave preset value"]
    #[inline(always)]
    pub fn r8_spi1_clock_div_r8_spi1_slave_preset(&self) -> R8Spi1ClockDivR8Spi1SlavePresetR {
        R8Spi1ClockDivR8Spi1SlavePresetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - master clock divisor / SPI0 slave preset value"]
    #[inline(always)]
    pub fn r8_spi1_clock_div_r8_spi1_slave_preset(
        &mut self,
    ) -> R8Spi1ClockDivR8Spi1SlavePresetW<'_, R8Spi1ClockDivR8Spi1SlavePreSpec> {
        R8Spi1ClockDivR8Spi1SlavePresetW::new(self, 0)
    }
}
#[doc = "SPI1 master clock divisor / SPI0 slave preset value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi1_clock_div_r8_spi1_slave_pre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi1_clock_div_r8_spi1_slave_pre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi1ClockDivR8Spi1SlavePreSpec;
impl crate::RegisterSpec for R8Spi1ClockDivR8Spi1SlavePreSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi1_clock_div_r8_spi1_slave_pre::R`](R) reader structure"]
impl crate::Readable for R8Spi1ClockDivR8Spi1SlavePreSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi1_clock_div_r8_spi1_slave_pre::W`](W) writer structure"]
impl crate::Writable for R8Spi1ClockDivR8Spi1SlavePreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE to value 0x10"]
impl crate::Resettable for R8Spi1ClockDivR8Spi1SlavePreSpec {
    const RESET_VALUE: u8 = 0x10;
}
