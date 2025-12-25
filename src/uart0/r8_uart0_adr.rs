#[doc = "Register `R8_UART0_ADR` reader"]
pub type R = crate::R<R8Uart0AdrSpec>;
#[doc = "Register `R8_UART0_ADR` writer"]
pub type W = crate::W<R8Uart0AdrSpec>;
#[doc = "Field `R8_UART0_ADR` reader - UART0 slave address"]
pub type R8Uart0AdrR = crate::FieldReader;
#[doc = "Field `R8_UART0_ADR` writer - UART0 slave address"]
pub type R8Uart0AdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UART0 slave address"]
    #[inline(always)]
    pub fn r8_uart0_adr(&self) -> R8Uart0AdrR {
        R8Uart0AdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART0 slave address"]
    #[inline(always)]
    pub fn r8_uart0_adr(&mut self) -> R8Uart0AdrW<'_, R8Uart0AdrSpec> {
        R8Uart0AdrW::new(self, 0)
    }
}
#[doc = "UART0 slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0AdrSpec;
impl crate::RegisterSpec for R8Uart0AdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_adr::R`](R) reader structure"]
impl crate::Readable for R8Uart0AdrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart0_adr::W`](W) writer structure"]
impl crate::Writable for R8Uart0AdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART0_ADR to value 0xff"]
impl crate::Resettable for R8Uart0AdrSpec {
    const RESET_VALUE: u8 = 0xff;
}
