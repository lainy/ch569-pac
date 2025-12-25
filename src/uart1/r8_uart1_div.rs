#[doc = "Register `R8_UART1_DIV` reader"]
pub type R = crate::R<R8Uart1DivSpec>;
#[doc = "Register `R8_UART1_DIV` writer"]
pub type W = crate::W<R8Uart1DivSpec>;
#[doc = "Field `R8_UART1_DIV` reader - UART pre-divisor latch byte"]
pub type R8Uart1DivR = crate::FieldReader;
#[doc = "Field `R8_UART1_DIV` writer - UART pre-divisor latch byte"]
pub type R8Uart1DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn r8_uart1_div(&self) -> R8Uart1DivR {
        R8Uart1DivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn r8_uart1_div(&mut self) -> R8Uart1DivW<'_, R8Uart1DivSpec> {
        R8Uart1DivW::new(self, 0)
    }
}
#[doc = "UART1 pre-divisor latch byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1DivSpec;
impl crate::RegisterSpec for R8Uart1DivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_div::R`](R) reader structure"]
impl crate::Readable for R8Uart1DivSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart1_div::W`](W) writer structure"]
impl crate::Writable for R8Uart1DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART1_DIV to value 0"]
impl crate::Resettable for R8Uart1DivSpec {}
