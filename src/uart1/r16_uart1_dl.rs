#[doc = "Register `R16_UART1_DL` reader"]
pub type R = crate::R<R16Uart1DlSpec>;
#[doc = "Register `R16_UART1_DL` writer"]
pub type W = crate::W<R16Uart1DlSpec>;
#[doc = "Field `R16_UART1_DL` reader - UART divisor latch"]
pub type R16Uart1DlR = crate::FieldReader<u16>;
#[doc = "Field `R16_UART1_DL` writer - UART divisor latch"]
pub type R16Uart1DlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn r16_uart1_dl(&self) -> R16Uart1DlR {
        R16Uart1DlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn r16_uart1_dl(&mut self) -> R16Uart1DlW<'_, R16Uart1DlSpec> {
        R16Uart1DlW::new(self, 0)
    }
}
#[doc = "UART1 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart1_dl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart1_dl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uart1DlSpec;
impl crate::RegisterSpec for R16Uart1DlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uart1_dl::R`](R) reader structure"]
impl crate::Readable for R16Uart1DlSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uart1_dl::W`](W) writer structure"]
impl crate::Writable for R16Uart1DlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UART1_DL to value 0"]
impl crate::Resettable for R16Uart1DlSpec {}
