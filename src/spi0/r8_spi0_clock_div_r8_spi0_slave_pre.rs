#[doc = "Register `R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE` reader"]
pub type R = crate::R<R8Spi0ClockDivR8Spi0SlavePreSpec>;
#[doc = "Register `R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE` writer"]
pub type W = crate::W<R8Spi0ClockDivR8Spi0SlavePreSpec>;
#[doc = "Field `R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE` reader - master clock divisor / SPI0 slave preset value"]
pub type R8Spi0ClockDivR8Spi0SlavePreR = crate::FieldReader;
#[doc = "Field `R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE` writer - master clock divisor / SPI0 slave preset value"]
pub type R8Spi0ClockDivR8Spi0SlavePreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - master clock divisor / SPI0 slave preset value"]
    #[inline(always)]
    pub fn r8_spi0_clock_div_r8_spi0_slave_pre(&self) -> R8Spi0ClockDivR8Spi0SlavePreR {
        R8Spi0ClockDivR8Spi0SlavePreR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - master clock divisor / SPI0 slave preset value"]
    #[inline(always)]
    pub fn r8_spi0_clock_div_r8_spi0_slave_pre(
        &mut self,
    ) -> R8Spi0ClockDivR8Spi0SlavePreW<'_, R8Spi0ClockDivR8Spi0SlavePreSpec> {
        R8Spi0ClockDivR8Spi0SlavePreW::new(self, 0)
    }
}
#[doc = "SPI0 master clock divisor / SPI0 slave preset value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_clock_div_r8_spi0_slave_pre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_clock_div_r8_spi0_slave_pre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0ClockDivR8Spi0SlavePreSpec;
impl crate::RegisterSpec for R8Spi0ClockDivR8Spi0SlavePreSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_clock_div_r8_spi0_slave_pre::R`](R) reader structure"]
impl crate::Readable for R8Spi0ClockDivR8Spi0SlavePreSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi0_clock_div_r8_spi0_slave_pre::W`](W) writer structure"]
impl crate::Writable for R8Spi0ClockDivR8Spi0SlavePreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE to value 0x10"]
impl crate::Resettable for R8Spi0ClockDivR8Spi0SlavePreSpec {
    const RESET_VALUE: u8 = 0x10;
}
