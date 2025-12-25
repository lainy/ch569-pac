#[doc = "Register `R16_UART2_DL` reader"]
pub type R = crate::R<R16Uart2DlSpec>;
#[doc = "Register `R16_UART2_DL` writer"]
pub type W = crate::W<R16Uart2DlSpec>;
#[doc = "Field `R16_UART2_DL` reader - UART divisor latch"]
pub type R16Uart2DlR = crate::FieldReader<u16>;
#[doc = "Field `R16_UART2_DL` writer - UART divisor latch"]
pub type R16Uart2DlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn r16_uart2_dl(&self) -> R16Uart2DlR {
        R16Uart2DlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn r16_uart2_dl(&mut self) -> R16Uart2DlW<'_, R16Uart2DlSpec> {
        R16Uart2DlW::new(self, 0)
    }
}
#[doc = "UART2 divisor latch\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uart2_dl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uart2_dl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uart2DlSpec;
impl crate::RegisterSpec for R16Uart2DlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uart2_dl::R`](R) reader structure"]
impl crate::Readable for R16Uart2DlSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uart2_dl::W`](W) writer structure"]
impl crate::Writable for R16Uart2DlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UART2_DL to value 0"]
impl crate::Resettable for R16Uart2DlSpec {}
