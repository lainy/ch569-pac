#[doc = "Register `R8_UART3_DIV` reader"]
pub type R = crate::R<R8Uart3DivSpec>;
#[doc = "Register `R8_UART3_DIV` writer"]
pub type W = crate::W<R8Uart3DivSpec>;
#[doc = "Field `R8_UART3_DIV` reader - UART pre-divisor latch byte"]
pub type R8Uart3DivR = crate::FieldReader;
#[doc = "Field `R8_UART3_DIV` writer - UART pre-divisor latch byte"]
pub type R8Uart3DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn r8_uart3_div(&self) -> R8Uart3DivR {
        R8Uart3DivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn r8_uart3_div(&mut self) -> R8Uart3DivW<'_, R8Uart3DivSpec> {
        R8Uart3DivW::new(self, 0)
    }
}
#[doc = "UART3 pre-divisor latch byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart3_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart3DivSpec;
impl crate::RegisterSpec for R8Uart3DivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart3_div::R`](R) reader structure"]
impl crate::Readable for R8Uart3DivSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart3_div::W`](W) writer structure"]
impl crate::Writable for R8Uart3DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART3_DIV to value 0"]
impl crate::Resettable for R8Uart3DivSpec {}
